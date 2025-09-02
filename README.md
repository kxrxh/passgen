# 🔐 PassGen

> 🚀 **The ultimate password generator for security-conscious developers**

A powerful, feature-rich CLI tool for generating strong, secure passwords with enterprise-grade security features and interactive wizards.

## ✨ Features

### 🔐 **Core Security Features**

- **Military-Grade Passwords**: Generate cryptographically secure passwords
- **Entropy Analysis**: Real-time strength calculation with zxcvbn scoring
- **Crack Time Estimation**: Know exactly how long your password would last
- **Character Set Control**: Fine-tune exactly what characters to include
- **Ambiguous Character Exclusion**: Avoid visually similar characters (1, l, I, 0, O)

### 🧠 **Smart Generation**

- **Phonetic Passwords**: Easy-to-remember passwords following phonetic patterns
- **Pattern-Based Generation**: Structured passwords using custom templates
- **Interactive Wizard**: Step-by-step guided password creation

### 🚀 **Developer Experience**

- **JSON Output**: Perfect for automation and scripting
- **Batch Generation**: Create multiple passwords at once
- **Comprehensive CLI**: 15+ command-line options
- **Zero-Config Setup**: Works out of the box

## 🎬 Demos

### 🔧 Basic Usage

![Basic Password Generation](demo/demo-basic.gif)
*Generate secure passwords with strength analysis*

### 🎯 Interactive Mode

![Interactive Password Builder](demo/demo-interactive.gif)
*Step-by-step guided password creation wizard*

### 🧠 Smart Generation

![Smart Password Generation](demo/demo-smart.gif)
*Phonetic and pattern-based password generation*

## Installation

### Using the Installation Script (Linux/macOS)

An installation script is provided for convenience on Linux and macOS systems.

**Prerequisites:**

- `git` must be installed.
- `cargo` (Rust toolchain) must be installed. You can install it from [https://rustup.rs/](https://rustup.rs/).

**Steps:**

1. Make the script executable:

    ```bash
    chmod +x passgen/install.sh
    ```

2. Run the script from the directory *containing* the `passgen` folder:

    ```bash
    ./passgen/install.sh
    ```

The script will:

- Check for `git` and `cargo`.
- Build the project in release mode.
- Attempt to copy the compiled `passgen` binary to `/usr/local/bin`. You might be prompted for your password if `sudo` is required for this step.

### Manual Installation

```bash
git clone https://github.com/kxrxh/passgen.git
cd passgen

cargo build --release

sudo cp target/release/passgen /usr/local/bin/
```

## Uninstallation (Script Method)

If you installed using the `install.sh` script or manually copied the binary to `/usr/local/bin`, you can use the `uninstall.sh` script.

1. Make the script executable:

    ```bash
    chmod +x passgen/uninstall.sh
    ```

2. Run the script:

    ```bash
    ./passgen/uninstall.sh
    ```

This script will attempt to remove the `passgen` binary from `/usr/local/bin`. You might be prompted for your password if `sudo` is required.

## 🚀 Quick Start

### 💡 **Simple Usage**

```bash
# Generate a secure 16-character password
passgen

# Create a 24-character password with strength analysis
passgen -l 24 --strength

# Generate 5 passwords at once
passgen -c 5 --detailed
```

### 🎯 **Advanced Examples**

#### **Smart Password Generation**

```bash
# Phonetic password (easy to remember)
passgen --phonetic --detailed

# Pattern-based password (structured)
passgen --pattern "UULLDDSS" --detailed

# Interactive password builder
passgen --interactive
```

#### **Developer Automation**

```bash
# JSON output for scripts
passgen -c 3 --json

# Custom character sets
passgen -u -n -s -a --length 32 --strength
```

### 📊 **Command Reference**

```bash
# Full help
passgen --help
```

### ⚙️ **Command Line Options**

#### **Password Generation**

| Option | Description |
|--------|-------------|
| `-l, --length <LENGTH>` | Password length (default: 16) |
| `-u, --uppercase` | Include uppercase letters (A-Z) |
| `-w, --lowercase` | Include lowercase letters (a-z) |
| `-n, --numbers` | Include numbers (0-9) |
| `-s, --special` | Include special characters |
| `-a, --avoid-ambiguous` | Exclude ambiguous chars (1, l, I, 0, O) |

#### **Analysis & Output**

| Option | Description |
|--------|-------------|
| `-c, --count <COUNT>` | Number of passwords to generate |
| `-t, --strength` | Show password strength analysis |
| `-j, --json` | Output in JSON format |
| `-d, --detailed` | Show detailed analysis |

#### **Smart Generation**

| Option | Description |
|--------|-------------|
| `-i, --interactive` | Interactive password building wizard |
| `--phonetic` | Generate phonetic password (easier to remember) |
| `-p, --pattern <PATTERN>` | Pattern-based generation (U/L/D/S) |

#### **General**

| Option | Description |
|--------|-------------|
| `-h, --help` | Show help information |
| `-V, --version` | Show version information |

## 📈 **Why PassGen?**

### 🎯 **Perfect For**

- **Developers** who need strong passwords for multiple services
- **DevOps** engineers managing server credentials
- **Security professionals** testing password policies
- **Teams** requiring consistent password generation
- **Automation scripts** needing secure password generation

### 🚀 **Performance**

- **Lightning fast** password generation
- **Minimal resource usage** (perfect for CI/CD)
- **No network dependencies** (works offline)
- **Cross-platform compatibility** (Linux, macOS, Windows)

## Installation

### 📦 **Quick Install**

```bash
# Using cargo (recommended)
cargo install passgen

# Or build from source
git clone https://github.com/kxrxh/passgen.git
cd passgen
cargo build --release
```

### 🔧 **System Requirements**

- **Rust 1.70+** (for building from source)
- **No external dependencies** (works offline)

### 🏃‍♂️ **Getting Started**

```bash
# Generate your first password
passgen

# Get detailed strength analysis
passgen --detailed

# Try the interactive mode
passgen --interactive
```

## License

MIT License
