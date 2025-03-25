# AI Agents and Vibe Coding 🤖✨

This document outlines best practices for using AI agents in the Brush development process, including the concept of "vibe coding" and setting appropriate expectations for AI-assisted contributions.

## Introduction to AI-Assisted Development 🌟

AI-assisted development uses large language models (LLMs) and AI agents to help with coding tasks. In the Brush project, we employ AI assistance to:

- Generate code suggestions
- Document existing code
- Debug complex issues
- Explore architectural options
- Optimize performance

## "Vibe Coding" Concept 🎭

"Vibe coding" refers to an experimental approach where AI agents help generate code that:

1. Aligns with the **aesthetic** and **style** of the existing codebase
2. Maintains consistent **patterns** and **idioms**
3. Captures the intended **functionality** while being adaptable
4. Creates a cohesive **developer experience**

```
┌────────────────┐     ┌────────────────┐     ┌────────────────┐
│                │     │                │     │                │
│  Developer     │────►│  AI Agent      │────►│  Code with     │
│  Intention     │     │  Translation   │     │  "Vibe"        │
│                │     │                │     │                │
└────────────────┘     └────────────────┘     └────────────────┘
```

### Key Characteristics

- **Style Consistency**: Matches naming conventions, formatting, and code organization
- **Idiomatic Code**: Uses Rust patterns common in the codebase
- **Contextual Adaptation**: Adapts to surrounding code context
- **Maintainability**: Prioritizes readability and long-term maintenance
- **Flexibility**: Avoids rigid, over-engineered solutions

## Guidelines for AI-Assisted Development 📝

### 1. Defining Tasks for AI Agents

When working with AI agents, define tasks with:

- **Clear Scope**: Specific, bounded functionality
- **Context**: Relevant background information
- **Constraints**: Performance requirements, dependencies, etc.
- **Examples**: Similar code in the codebase

Example Task Definition:

```
Task: Implement a function to serialize Gaussian splats to PLY format

Context:
- This will be part of the export functionality in brush-render
- We already have code to load PLY files in gaussian_splats.rs
- The function should handle both SH coefficients and RGB colors

Constraints:
- Must handle large numbers of splats efficiently
- Should be compatible with third-party viewers
- Must include all properties (position, rotation, scale, color)

Example: See the load_ply function in gaussian_splats.rs for reference
```

### 2. Human Review Process

All AI-generated code must go through human review:

1. **Correctness Check**: Verify logical correctness and functionality
2. **Performance Review**: Check for obvious performance issues
3. **Style Conformance**: Ensure code follows project conventions
4. **Test Coverage**: Add tests for the generated code
5. **Documentation**: Ensure appropriate documentation is included

### 3. Attribution Guidelines

When using AI-generated code:

- **Source Comments**: Add a comment indicating AI assistance when significant
- **Review Notes**: Mention AI assistance in PR descriptions
- **Modification Record**: Document any human modifications to AI-generated code

Example attribution comment:

```rust
// Initial implementation scaffolded with AI assistance, 
// with human modifications for performance optimization
```

## Appropriate Use Cases 🎯

AI agents are most effective for:

### Good Use Cases

- **Boilerplate Code**: Repetitive code structures
- **Documentation**: Generating API docs and examples
- **Test Generation**: Creating test cases and fixtures
- **Refactoring**: Suggests code improvements
- **Format Conversion**: Data transformation logic
- **UI Components**: Standard UI patterns and layouts

### Limited Use Cases

- **Critical Algorithms**: Core rendering or optimization algorithms
- **Performance-Critical Code**: Code where every cycle counts
- **Novel Approaches**: Truly innovative techniques
- **Security-Sensitive Code**: Authentication, encryption, etc.
- **Platform-Specific Features**: Low-level system integration

## Tools and Setup 🛠️

### Recommended AI Tools

- **Cursor**: IDE with integrated AI coding assistance
- **GitHub Copilot**: Code completion and suggestions
- **Claude**: Detailed code analysis and generation

### Integration in Development Workflow

1. **Planning**: Define features with human judgment
2. **Initial Implementation**: Use AI for scaffolding
3. **Review**: Human verification of generated code
4. **Refinement**: Manual optimization and fixes
5. **Testing**: Comprehensive testing of all code

## Expectations and Limitations ⚠️

### What AI Can Do Well

- Generate syntactically correct Rust code
- Follow patterns from examples
- Implement standard algorithms
- Adhere to type constraints
- Document existing code

### Current Limitations

- Limited understanding of performance implications
- May generate plausible but incorrect code
- Cannot fully understand project context
- Struggles with novel GPU or WebGPU patterns
- May suggest outdated or deprecated approaches

## Collaboration Process 👥

### Between Humans and AI

```
┌────────────┐    ┌────────────┐    ┌────────────┐    ┌────────────┐
│ Human      │    │ AI         │    │ Human      │    │ Human      │
│ Planning   │───►│ Initial    │───►│ Review &   │───►│ Testing &  │
│ & Design   │    │ Code       │    │ Refinement │    │ Integration │
└────────────┘    └────────────┘    └────────────┘    └────────────┘
```

### Between Multiple Developers Using AI

- **Consistent Prompting**: Use similar task descriptions
- **Share Generated Code**: Discuss AI-generated solutions
- **Collective Review**: Review AI-generated code together
- **Document Patterns**: Record effective AI prompt patterns

## Case Studies and Examples 📚

### Success Case: UI Panel Generation

AI successfully generated a consistent UI panel matching existing styles:

```rust
// Example of AI-generated UI panel code that matched
// existing conventions and seamlessly integrated
impl TrainingPanel {
    pub fn ui(&mut self, ui: &mut egui::Ui, training: &mut TrainingState) {
        ui.heading("Training Parameters");
        
        ui.add_space(8.0);
        ui.separator();
        ui.add_space(8.0);
        
        // Similar organization to other panels
        ui.horizontal(|ui| {
            ui.label("Learning Rate:");
            ui.add(egui::Slider::new(&mut training.learning_rate, 0.0001..=0.1)
                .logarithmic(true));
        });
        
        // More UI code following existing patterns...
    }
}
```

### Challenge Case: Shader Optimization

AI initially generated inefficient shader code that needed human intervention:

```wgsl
// AI initially generated this inefficient shader
@compute @workgroup_size(256)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let idx = global_id.x;
    if (idx >= u_globals.num_points) {
        return;
    }
    
    // Inefficient approach that needed human optimization
    for (var i = 0u; i < u_globals.num_points; i++) {
        // O(n²) algorithm that was later optimized by a human
    }
}
```

## Best Practices for Vibe Coding 💡

### For Developers

1. **Start with Clear Understanding**: Know what you're trying to build
2. **Use References**: Point AI to existing similar features
3. **Iterative Refinement**: Generate, review, refine, repeat
4. **Test Thoroughly**: AI code needs more testing, not less
5. **Learn from Patterns**: Study how AI interprets project conventions

### For AI Prompting

1. **Be Specific**: "Match the style in src/render.rs" instead of "Write good code"
2. **Provide Context**: Include relevant architectural details
3. **Set Boundaries**: Define inputs, outputs, and performance requirements
4. **Include Examples**: Reference similar existing code
5. **Request Explanations**: Ask AI to explain design decisions

## Measuring Success 📊

Evaluate AI-assisted code based on:

- **Integration Smoothness**: How well it fits with existing code
- **Maintainability**: How easy it is to understand and modify
- **Performance**: How efficiently it operates
- **Bug Density**: How many issues are found during review
- **Development Speed**: How much time is saved compared to manual coding

## Future Directions 🔮

As AI tools evolve, we anticipate:

- **More Context-Aware Agents**: Better understanding of the entire codebase
- **Performance-Aware Generation**: Code that considers computational efficiency
- **Enhanced Collaboration**: Better human-AI pairing workflows
- **Specialized for Graphics/GPU**: Models trained specifically for graphics programming

## References and Resources 📚

- [GitHub's guide on AI-assisted development](https://github.blog/2023-06-20-how-to-write-better-prompts-for-github-copilot/)
- [Cursor documentation](https://cursor.sh/docs)
- [Prompt engineering best practices](https://www.promptingguide.ai/)

## Next Steps 🔍

- Explore the [Training Module](training_module.md)
- Understand the [Rendering Module](rendering_module.md)
- Learn about [Debugging and Profiling](debugging_profiling.md) 