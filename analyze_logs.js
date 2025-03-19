#!/usr/bin/env node

/**
 * MCP Log Analyzer Tool for Brush Debugging
 * 
 * This tool analyzes MCP server logs to find issues and potential bugs.
 * It can be used manually or as part of an automated testing process.
 */

const fs = require('fs');
const path = require('path');
const readline = require('readline');

// Configuration
const config = {
    logFile: process.argv[2] || 'mcp_log.txt',
    outputFile: process.argv[3] || 'analysis_report.md',
    knownPatterns: {
        errors: [
            { pattern: /no filesystem on this platform/i, description: 'WASM Filesystem Error', severity: 'high' },
            { pattern: /RuntimeError: unreachable/i, description: 'Rust panic', severity: 'critical' },
            { pattern: /❌/i, description: 'Application-reported error', severity: 'medium' },
            { pattern: /BRUSH_ERROR/i, description: 'Application error log', severity: 'high' },
            { pattern: /panicked at/i, description: 'Rust panic with location', severity: 'critical' },
        ],
        warnings: [
            { pattern: /BRUSH_WARN/i, description: 'Application warning', severity: 'low' },
            { pattern: /⚠️/i, description: 'Application-reported warning', severity: 'low' },
        ],
        infos: [
            { pattern: /🧪 DEBUG/i, description: 'Debug mode log', severity: 'info' },
            { pattern: /🔍/i, description: 'Diagnostic information', severity: 'info' },
            { pattern: /BRUSH_INFO/i, description: 'Application info log', severity: 'info' },
        ]
    }
};

// Analysis results
const results = {
    errors: [],
    warnings: [],
    infos: [],
    stats: {
        totalLines: 0,
        errorCount: 0,
        warningCount: 0,
        infoCount: 0,
        startTime: null,
        endTime: null
    }
};

/**
 * Add timestamp to analysis entries
 */
function parseTimestamp(line) {
    const timestampMatch = line.match(/timestamp: (\d+)/);
    if (timestampMatch && timestampMatch[1]) {
        const timestamp = parseInt(timestampMatch[1]);
        if (!isNaN(timestamp)) {
            const date = new Date(timestamp);
            return date.toISOString();
        }
    }
    return null;
}

/**
 * Process a single log line
 */
async function processLine(line) {
    results.stats.totalLines++;
    
    // Extract timestamp if present
    const timestamp = parseTimestamp(line);
    if (timestamp) {
        if (!results.stats.startTime || timestamp < results.stats.startTime) {
            results.stats.startTime = timestamp;
        }
        if (!results.stats.endTime || timestamp > results.stats.endTime) {
            results.stats.endTime = timestamp;
        }
    }
    
    // Check for error patterns
    for (const errorPattern of config.knownPatterns.errors) {
        if (errorPattern.pattern.test(line)) {
            results.errors.push({
                line: results.stats.totalLines,
                timestamp,
                pattern: errorPattern.description,
                severity: errorPattern.severity,
                content: line.trim()
            });
            results.stats.errorCount++;
            return; // Stop after finding an error
        }
    }
    
    // Check for warning patterns
    for (const warnPattern of config.knownPatterns.warnings) {
        if (warnPattern.pattern.test(line)) {
            results.warnings.push({
                line: results.stats.totalLines,
                timestamp,
                pattern: warnPattern.description,
                severity: warnPattern.severity,
                content: line.trim()
            });
            results.stats.warningCount++;
            return; // Stop after finding a warning
        }
    }
    
    // Check for info patterns
    for (const infoPattern of config.knownPatterns.infos) {
        if (infoPattern.pattern.test(line)) {
            results.infos.push({
                line: results.stats.totalLines,
                timestamp,
                pattern: infoPattern.description,
                content: line.trim()
            });
            results.stats.infoCount++;
            return; // Stop after finding info
        }
    }
}

/**
 * Process the entire log file
 */
async function processLogFile() {
    console.log(`Analyzing log file: ${config.logFile}`);
    
    try {
        if (!fs.existsSync(config.logFile)) {
            console.error(`Error: Log file '${config.logFile}' does not exist.`);
            return false;
        }
        
        const fileStream = fs.createReadStream(config.logFile);
        const rl = readline.createInterface({
            input: fileStream,
            crlfDelay: Infinity
        });
        
        for await (const line of rl) {
            await processLine(line);
        }
        
        return true;
    } catch (error) {
        console.error('Error processing log file:', error);
        return false;
    }
}

/**
 * Generate markdown report from analysis results
 */
function generateReport() {
    let report = `# Brush Debug Log Analysis\n\n`;
    report += `**Generated on:** ${new Date().toISOString()}\n`;
    report += `**Log file:** ${config.logFile}\n\n`;
    
    // Summary
    report += `## Summary\n\n`;
    report += `- **Total log lines:** ${results.stats.totalLines}\n`;
    report += `- **Errors found:** ${results.stats.errorCount}\n`;
    report += `- **Warnings found:** ${results.stats.warningCount}\n`;
    report += `- **Info messages:** ${results.stats.infoCount}\n`;
    
    if (results.stats.startTime && results.stats.endTime) {
        report += `- **Log timespan:** ${results.stats.startTime} to ${results.stats.endTime}\n`;
    }
    
    // Critical errors first
    if (results.errors.length > 0) {
        report += `\n## Errors (${results.errors.length})\n\n`;
        
        // Group by severity
        const criticalErrors = results.errors.filter(e => e.severity === 'critical');
        const highErrors = results.errors.filter(e => e.severity === 'high');
        const mediumErrors = results.errors.filter(e => e.severity === 'medium');
        
        if (criticalErrors.length > 0) {
            report += `### Critical Errors (${criticalErrors.length})\n\n`;
            for (const error of criticalErrors) {
                report += `- **Line ${error.line}**: ${error.pattern}\n`;
                report += `  - ${error.timestamp ? `Time: ${error.timestamp}` : 'No timestamp'}\n`;
                report += `  - \`${error.content}\`\n\n`;
            }
        }
        
        if (highErrors.length > 0) {
            report += `### High Severity Errors (${highErrors.length})\n\n`;
            for (const error of highErrors) {
                report += `- **Line ${error.line}**: ${error.pattern}\n`;
                report += `  - ${error.timestamp ? `Time: ${error.timestamp}` : 'No timestamp'}\n`;
                report += `  - \`${error.content}\`\n\n`;
            }
        }
        
        if (mediumErrors.length > 0) {
            report += `### Medium Severity Errors (${mediumErrors.length})\n\n`;
            for (const error of mediumErrors) {
                report += `- **Line ${error.line}**: ${error.pattern}\n`;
                report += `  - ${error.timestamp ? `Time: ${error.timestamp}` : 'No timestamp'}\n`;
                report += `  - \`${error.content}\`\n\n`;
            }
        }
    }
    
    // Warnings
    if (results.warnings.length > 0) {
        report += `\n## Warnings (${results.warnings.length})\n\n`;
        for (const warning of results.warnings) {
            report += `- **Line ${warning.line}**: ${warning.pattern}\n`;
            report += `  - ${warning.timestamp ? `Time: ${warning.timestamp}` : 'No timestamp'}\n`;
            report += `  - \`${warning.content}\`\n\n`;
        }
    }
    
    // Info Messages (limited to keep report manageable)
    const infoLimit = 20;
    if (results.infos.length > 0) {
        report += `\n## Info Messages ${results.infos.length > infoLimit ? `(showing ${infoLimit} of ${results.infos.length})` : `(${results.infos.length})`}\n\n`;
        for (const info of results.infos.slice(0, infoLimit)) {
            report += `- **Line ${info.line}**: ${info.pattern}\n`;
            report += `  - ${info.timestamp ? `Time: ${info.timestamp}` : 'No timestamp'}\n`;
            report += `  - \`${info.content}\`\n\n`;
        }
        
        if (results.infos.length > infoLimit) {
            report += `\n*...and ${results.infos.length - infoLimit} more info messages*\n`;
        }
    }
    
    // Recommendations
    report += `\n## Recommendations\n\n`;
    
    if (results.errors.length === 0 && results.warnings.length === 0) {
        report += `✅ No errors or warnings detected in the logs. Application appears to be running correctly.\n`;
    } else {
        if (results.errors.filter(e => e.severity === 'critical').length > 0) {
            report += `❌ **Critical issues detected!** Application is crashing or experiencing severe errors.\n`;
        }
        
        if (results.errors.filter(e => /filesystem|RuntimeError/.test(e.content)).length > 0) {
            report += `⚠️ Web platform filesystem errors detected. Check for proper WASM environment handling.\n`;
        }
        
        if (results.errors.filter(e => /panic/.test(e.content)).length > 0) {
            report += `⚠️ Rust panics detected. Review error messages and stack traces for the source of the crash.\n`;
        }
    }
    
    return report;
}

/**
 * Main function
 */
async function main() {
    const success = await processLogFile();
    
    if (success) {
        const report = generateReport();
        
        // Write to file
        fs.writeFileSync(config.outputFile, report);
        console.log(`Analysis complete. Report written to: ${config.outputFile}`);
        
        // Print quick summary to console
        console.log('\nQuick summary:');
        console.log(`- Total log lines: ${results.stats.totalLines}`);
        console.log(`- Errors found: ${results.stats.errorCount}`);
        console.log(`- Warnings found: ${results.stats.warningCount}`);
        console.log(`- Info messages: ${results.stats.infoCount}`);
        
        // Exit with error if critical issues found
        if (results.errors.filter(e => e.severity === 'critical').length > 0) {
            console.error('\n⚠️ CRITICAL ISSUES DETECTED! See report for details.');
            process.exit(1);
        }
    } else {
        console.error('Analysis failed.');
        process.exit(1);
    }
}

// Run the script
main().catch(error => {
    console.error('Unhandled error:', error);
    process.exit(1);
}); 