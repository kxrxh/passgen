#!/bin/bash

set -e # Exit immediately if a command exits with a non-zero status.

# --- Configuration ---
INSTALL_DIR="/usr/local/bin"
PROJECT_NAME="passgen"
BINARY_PATH="$INSTALL_DIR/$PROJECT_NAME"

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

info_message() {
    echo "ℹ️ $1"
}

# --- Uninstall Steps ---
echo "Starting uninstallation for $PROJECT_NAME..."

# 1. Check if binary exists
if [ ! -f "$BINARY_PATH" ]; then
    info_message "$PROJECT_NAME binary not found at $BINARY_PATH. Nothing to uninstall."
    exit 0
fi

echo "Found $PROJECT_NAME at $BINARY_PATH."

# 2. Attempt to remove the binary
echo "Attempting to remove $BINARY_PATH..."
if [ -w "$INSTALL_DIR" ]; then
    # User has write permissions
    rm -f "$BINARY_PATH" || error_exit "Failed to remove binary from $BINARY_PATH."
else
    # Need sudo
    echo "Write permission denied for $INSTALL_DIR. Trying with sudo..."
    if command_exists sudo; then
        sudo rm -f "$BINARY_PATH" || error_exit "Failed to remove binary from $BINARY_PATH using sudo. Check permissions or try manual removal."
    else
        error_exit "sudo command not found, and no write permission for $INSTALL_DIR. Please remove manually: sudo rm -f $BINARY_PATH"
    fi
fi

success_message "$PROJECT_NAME successfully uninstalled from $BINARY_PATH."

echo "Uninstallation complete." 