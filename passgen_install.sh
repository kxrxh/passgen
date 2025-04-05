#!/bin/bash

set -e # Exit immediately if a command exits with a non-zero status.

# --- Configuration ---
REPO_URL="https://github.com/yourusername/passgen.git" # !!! Replace with the actual repository URL !!!
INSTALL_DIR="/usr/local/bin"
PROJECT_NAME="passgen"
CLONE_DIR=$(basename "$REPO_URL" .git)

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
if ! command_exists git; then
    error_exit "git is not installed. Please install git and try again."
fi
if ! command_exists cargo; then
    error_exit "Cargo (Rust toolchain) is not installed. Please install Rust (https://rustup.rs/) and try again."
fi
success_message "Dependencies found."

# --- Installation Steps ---
echo "Starting installation for $PROJECT_NAME..."

# 1. Clone the repository
if [ -d "$CLONE_DIR" ]; then
    echo "Directory '$CLONE_DIR' already exists. Skipping clone."
    cd "$CLONE_DIR"
    # Optional: uncomment the next line to pull latest changes if directory exists
    # git pull origin main || error_exit "Failed to pull latest changes from repository."
else
    echo "Cloning repository from $REPO_URL..."
    git clone "$REPO_URL" || error_exit "Failed to clone repository."
    cd "$CLONE_DIR" || error_exit "Failed to change directory to $CLONE_DIR."
fi
success_message "Repository ready."

# 2. Build the project
echo "Building project with Cargo (release mode)..."
cargo build --release || error_exit "Cargo build failed."
success_message "Build successful."

# 3. Install the binary
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

# --- Cleanup ---
# cd .. # Optional: go back to the original directory
# rm -rf "$CLONE_DIR" # Optional: remove the cloned repository directory

echo "Installation complete! You can now run '$PROJECT_NAME'." 