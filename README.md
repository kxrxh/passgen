# PassGen

A simple CLI tool for generating strong, secure passwords with customizable options.

## Features

- Generate passwords of any length
- Customize character sets (uppercase, lowercase, numbers, special characters)
- Option to exclude ambiguous characters to improve readability
- Ensure at least one character from each selected set for increased security
- Password strength analysis with entropy
- Batch password generation
- JSON output for programmatic use
- Detailed password analysis with crack time estimates
- zxcvbn-based password strength scoring
- Secure encrypted password storage
- Interactive password building wizard
- Smart password generation (phonetic, pattern-based)

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

# Generate 5 passwords with strength analysis
passgen -c 5 -t

# Generate a password with detailed analysis
passgen --detailed

# Generate 3 passwords in JSON format
passgen -c 3 -j

# Interactive password building wizard
passgen --interactive

# Generate phonetic password (easier to remember)
passgen --phonetic --detailed

# Generate password using pattern (U=uppercase, L=lowercase, D=digit, S=symbol)
passgen --pattern "UULLDDSS" --detailed



# Create a new encrypted password safe
passgen --create-safe mypasswords.safe

# Store a generated password in the safe
passgen --service "GitHub" --username "myuser" --store

# List passwords stored in the safe
passgen --open-safe mypasswords.safe --list-safe

# Show help
passgen --help
```

### Command Line Options

```
Options:
  -l, --length <LENGTH>      Length of the password [default: 16]
  -u, --uppercase            Include uppercase letters
  -w, --lowercase            Include lowercase letters
  -n, --numbers              Include numbers
  -s, --special              Include special characters
  -a, --avoid-ambiguous      Exclude ambiguous characters like 1, l, I, 0, O
  -c, --count <COUNT>        Number of passwords to generate [default: 1]
  -t, --strength             Show password strength analysis
  -j, --json                 Output in JSON format
  -d, --detailed             Show detailed analysis (implies --strength)
  -i, --interactive          Interactive password building wizard
      --phonetic             Generate phonetic password (easier to remember)
  -p, --pattern <PATTERN>    Generate password using pattern (U=uppercase, L=lowercase, D=digit, S=symbol)

      --create-safe <FILE>   Create a new password safe
      --open-safe <FILE>     Open an existing password safe
      --store                Add current generated password to safe
      --list-safe            List passwords in safe
      --service <SERVICE>    Service name for storage operations
      --username <USERNAME>  Username for storage operations
  -h, --help                 Print help
  -V, --version              Print version
```

### New Features

#### Password Strength Analysis

PassGen now includes comprehensive password strength analysis using the zxcvbn library:

```bash
# Basic strength analysis
passgen --strength

# Detailed analysis with recommendations
passgen --detailed
```

The analysis includes:

- **Entropy calculation** in bits
- **Strength score** (0-4 scale)
- **Estimated crack time** based on offline attacks
- **Character set information**

#### Batch Generation

Generate multiple passwords at once:

```bash
# Generate 5 passwords
passgen -c 5

# Generate 10 passwords with strength analysis
passgen -c 10 --strength
```

#### JSON Output

Perfect for programmatic use and integration with other tools:

```bash
# Get passwords in JSON format
passgen -c 3 --json
```

Example JSON output:

```json
{
  "passwords": [
    {
      "password": "ExamplePass123!",
      "entropy_bits": 103.4,
      "strength_score": 4,
      "strength_label": "Very Strong",
      "crack_time_seconds": 1.014e21,
      "crack_time_display": "32157549473065 years",
      "character_sets": ["uppercase", "lowercase", "numbers", "special"]
    }
  ],
  "count": 1,
  "average_entropy": 103.4,
  "average_strength_score": 4.0
}
```

#### Interactive Password Building Wizard

Build your perfect password through an interactive step-by-step process:

```bash
passgen --interactive
```

Features:

- **Guided setup**: Step-by-step password configuration
- **Character set selection**: Interactive multi-select for character types
- **Smart defaults**: Sensible defaults with easy customization
- **Configuration summary**: Review your choices before generation

#### Smart Password Generation

Generate passwords using intelligent methods:

**Phonetic Passwords (easier to remember):**

```bash
passgen --phonetic --detailed
```

Creates passwords that follow phonetic patterns (alternating consonants and vowels) with added numbers and symbols for security.

**Pattern-Based Generation:**

```bash
passgen --pattern "UULLDDSS" --detailed
```

Use patterns to create structured passwords:

- `U` = Uppercase letter
- `L` = Lowercase letter
- `D` = Digit
- `S` = Special character

#### Secure Password Storage

Store and manage your passwords securely with military-grade encryption:

**Create a new password safe:**

```bash
passgen --create-safe mypasswords.safe
```

Creates an encrypted file protected by a master password using Argon2 key derivation and AES-256-GCM encryption.

**Store generated passwords:**

```bash
passgen --service "GitHub" --username "myuser" --store
```

Automatically stores the generated password with metadata.

**Manage your password safe:**

```bash
# List stored passwords
passgen --open-safe mypasswords.safe --list-safe

# Store a password with custom options
passgen --length 24 --strength --store --service "Google" --username "me@gmail.com"
```

**Security Features:**

- **AES-256-GCM encryption** for data protection
- **Argon2 key derivation** for master password hashing
- **Random salt generation** for each safe
- **Master password verification** with confirmation
- **Secure file format** with authenticated encryption

## License

MIT
