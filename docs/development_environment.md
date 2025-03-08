# Brush Development Environment Setup

This guide provides instructions for setting up your development environment to contribute to the Brush project. Don't worry if some of this seems technical - we've organized it to be approachable for everyone!

## For Casual Developers & Beginners

If you're new to development or just want to get started quickly, here's what you need:

### Absolute Basics

1. **Choose a directory** on your computer where you'll store the Brush code
   - This can be anywhere you like, such as `Documents/Code/brush`
   - You'll need to remember this location

2. **Install an AI-powered code editor**
   - We recommend [Cursor](https://cursor.sh/) which comes with AI assistance built-in
   - Cursor works on macOS, Windows, and Linux

3. **Let AI help you with the rest!**
   - If you're using Cursor with Claude, GPT, or another AI coding assistant, they can guide you through setting up the remaining requirements
   - Simply ask: "I want to set up my environment for Brush development. Can you help me install the necessary components for my operating system?"
   - The AI can read this document and provide step-by-step instructions specific to your needs

### Quick Start with AI Assistance

If you're using an AI coding assistant like Claude in Cursor:

1. Clone the repository:
   ```bash
   git clone https://github.com/ArthurBrussee/brush.git
   ```

2. Ask your AI assistant:
   ```
   I've cloned the Brush repository. Can you help me set up all the required dependencies for development on [your OS]?
   ```

3. The AI will guide you through installing Rust, WebAssembly tools, and other requirements based on your operating system

4. Once setup is complete, you can build and run Brush with:
   ```bash
   cargo run --bin brush_app
   ```

---

## For Experienced Developers & AI Agents

The following sections contain detailed technical information about all requirements and components needed for Brush development. If you're an experienced developer or an AI agent helping with setup, these details will be useful.

### Core Requirements

- **Rust and Cargo** - The Rust programming language and package manager
  ```bash
  # Install rustup (https://rustup.rs/)
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  
  # Update to the latest stable Rust
  rustup update stable
  ```

- **WebAssembly Target** - For web compilation support
  ```bash
  # Add the WebAssembly target
  rustup target add wasm32-unknown-unknown
  ```

- **Git** - Version control system
  ```bash
  # macOS (via Homebrew)
  brew install git
  
  # Ubuntu/Debian
  sudo apt install git
  
  # Windows
  # Download from https://git-scm.com/download/win
  ```

### Build Tools

- **Trunk** - Build tool for bundling Rust/WASM web applications
  ```bash
  # Install Trunk
  cargo install trunk
  ```

- **wasm-bindgen-cli** - Tool for generating JavaScript bindings for Rust code
  ```bash
  # Install wasm-bindgen-cli
  cargo install wasm-bindgen-cli
  ```

### Platform-Specific Requirements

#### macOS

- **Xcode Command Line Tools** - Required for compilation
  ```bash
  xcode-select --install
  ```

- **Homebrew** - Package manager for macOS
  ```bash
  # Install Homebrew
  /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
  ```

#### Linux (Ubuntu/Debian)

- **Build essentials** - Required for compilation
  ```bash
  sudo apt update
  sudo apt install build-essential pkg-config libssl-dev
  ```

#### Windows

- **Visual Studio Build Tools** - Required for compilation
  - Download and install from [Visual Studio Downloads](https://visualstudio.microsoft.com/downloads/)
  - Select "C++ build tools" during installation

## IDE Setup

While you can use any editor, we recommend:

- **Cursor** - AI-powered IDE with excellent Rust support
  - Download from [Cursor.sh](https://cursor.sh/)
  - Works with Claude, GPT, and other AI assistants
  - AI can help you navigate and understand the codebase

- **VS Code with rust-analyzer** - Popular alternative
  ```bash
  # Install rust-analyzer extension in VS Code
  code --install-extension rust-lang.rust-analyzer
  ```

## Verifying Your Setup

Run the following commands to verify your setup:

```bash
# Check Rust version
rustc --version

# Check Cargo version
cargo --version

# Check Trunk version
trunk --version

# Check wasm-bindgen version
wasm-bindgen --version
```

## Building and Running Brush

### Desktop Application

```bash
# Build the desktop application
cargo build --bin brush_app

# Run the desktop application
cargo run --bin brush_app
```

### Web Application

```bash
# Build the web application
# Note: Currently there's a Burn bug affecting web functionality
cd crates/brush-app
trunk build

# Serve the web application locally
trunk serve
```

## Common Issues and Solutions

### Compilation Errors

- **Missing dependencies**: Run `cargo check` to identify missing dependencies
- **Incompatible versions**: Check `Cargo.toml` for version constraints
- **Platform-specific issues**: See platform-specific requirements above

### Web Build Issues

- **WASM target missing**: Ensure you've run `rustup target add wasm32-unknown-unknown`
- **Trunk not found**: Make sure Trunk is installed and in your PATH
- **Burn-related issues**: There's a known Burn bug affecting web functionality; use the desktop app for testing UI changes

## Getting Help

If you encounter issues setting up your development environment, please:

1. Check the [GitHub Issues](https://github.com/ArthurBrussee/brush/issues) for similar problems
2. Consult the [Rust Forum](https://users.rust-lang.org/) for Rust-specific questions
3. Ask for help in our community channels
4. If you're using an AI coding assistant, ask it to explain the error and suggest solutions

---

This guide should help you get started with Brush development, regardless of your experience level. If you discover additional requirements or better approaches, please contribute back to this document! 