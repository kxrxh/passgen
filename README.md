# PassGen

A simple CLI tool for generating strong, secure passwords with customizable options.

## Features

- Generate passwords of any length
- Customize character sets (uppercase, lowercase, numbers, special characters)
- Option to exclude ambiguous characters to improve readability
- Ensure at least one character from each selected set for increased security

## Installation

### Using the Installation Script (Linux/macOS)

An installation script is provided for convenience on Linux and macOS systems.

**Prerequisites:**
*   `git` must be installed.
*   `cargo` (Rust toolchain) must be installed. You can install it from [https://rustup.rs/](https://rustup.rs/).

**Steps:**

1.  Make the script executable:
    ```bash
    chmod +x passgen/install.sh
    ```
2.  Run the script from the directory *containing* the `passgen` folder:
    ```bash
    ./passgen/install.sh
    ```

The script will:
*   Check for `git` and `cargo`.
*   Build the project in release mode.
*   Attempt to copy the compiled `passgen` binary to `/usr/local/bin`. You might be prompted for your password if `sudo` is required for this step.

### Manual Installation

```bash
git clone https://github.com/kxrxh/passgen.git
cd passgen

cargo build --release

sudo cp target/release/passgen /usr/local/bin/
```

## Uninstallation (Script Method)

If you installed using the `install.sh` script or manually copied the binary to `/usr/local/bin`, you can use the `uninstall.sh` script.

1.  Make the script executable:
    ```bash
    chmod +x passgen/uninstall.sh
    ```
2.  Run the script:
    ```bash
    ./passgen/uninstall.sh
    ```

This script will attempt to remove the `passgen` binary from `/usr/local/bin`. You might be prompted for your password if `sudo` is required.

## Usage

```bash
# Generate a default 16-character password with all character sets
passgen

# Generate a 12-character password
passgen -l 12

# Generate a password with only uppercase letters and numbers
passgen -u -n

# Generate a 20-character password without ambiguous characters
passgen -l 20 -a

# Show help
passgen --help
```

### Command Line Options

```
Options:
  -l, --length <LENGTH>  Length of the password [default: 16]
  -u, --uppercase        Include uppercase letters
  -w, --lowercase        Include lowercase letters
  -n, --numbers          Include numbers
  -s, --special          Include special characters
  -a, --avoid-ambiguous  Exclude ambiguous characters like 1, l, I, 0, O
  -h, --help             Print help
  -V, --version          Print version
```

## License

MIT 
