#!/bin/bash

set -e # Exit immediately if a command exits with a non-zero status.

# --- Configuration ---
INSTALL_DIR="/usr/local/bin"
PROJECT_NAME="passgen"

# --- Helper Functions ---
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

error_exit() {
    echo "Error: $1" >&2
    exit 1
}

success_message() {
    echo "✅ $1"
}

# --- Pre-flight Checks ---
echo "Checking dependencies..."
if ! command_exists cargo; then
    error_exit "Cargo (Rust toolchain) is not installed. Please install Rust (https://rustup.rs/) and try again."
fi
# Check if we are in the correct directory (basic check for Cargo.toml)
if [ ! -f "Cargo.toml" ]; then
    error_exit "Cargo.toml not found. Please run this script from the root directory of the '$PROJECT_NAME' project."
fi
success_message "Dependencies found and running in project root."

# --- Build and Installation Steps ---
echo "Starting build and installation for $PROJECT_NAME..."

# 1. Build the project
echo "Building project with Cargo (release mode)..."
cargo build --release || error_exit "Cargo build failed."
success_message "Build successful."

# 2. Install the binary
BINARY_PATH="target/release/$PROJECT_NAME"
if [ ! -f "$BINARY_PATH" ]; then
    error_exit "Compiled binary not found at $BINARY_PATH."
fi

echo "Attempting to install $PROJECT_NAME to $INSTALL_DIR..."
if [ -w "$INSTALL_DIR" ]; then
    # User has write permissions
    cp "$BINARY_PATH" "$INSTALL_DIR/" || error_exit "Failed to copy binary to $INSTALL_DIR."
else
    # Need sudo
    echo "Write permission denied for $INSTALL_DIR. Trying with sudo..."
    if command_exists sudo; then
        sudo cp "$BINARY_PATH" "$INSTALL_DIR/" || error_exit "Failed to copy binary to $INSTALL_DIR using sudo. Check permissions or try manual installation."
    else
        error_exit "sudo command not found, and no write permission for $INSTALL_DIR. Please install manually: sudo cp $BINARY_PATH $INSTALL_DIR/"
    fi
fi

success_message "$PROJECT_NAME installed successfully to $INSTALL_DIR/$PROJECT_NAME."

echo "Installation complete! You can now run '$PROJECT_NAME'." 