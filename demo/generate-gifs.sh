#!/bin/bash

# PassGen Demo GIF Generator
# This script generates all demo GIFs using VHS

set -e

echo "🎬 Generating PassGen Demo GIFs..."
echo

# Check if VHS is installed
if ! command -v vhs &> /dev/null; then
    echo "❌ VHS is not installed. Please install it first:"
    echo "   go install github.com/charmbracelet/vhs@latest"
    exit 1
fi

# Generate all demo GIFs
demos=(
    "demo-basic:Basic password generation features"
    "demo-interactive:Interactive password building wizard"
    "demo-smart:Smart password generation modes"
)

for demo in "${demos[@]}"; do
    name="${demo%:*}"
    description="${demo#*:}"

    echo "📹 Generating $name.gif - $description"
    vhs "demo/$name.vhs" -o "demo/$name.gif"

    if [ $? -eq 0 ]; then
        echo "✅ $name.gif created successfully"
    else
        echo "❌ Failed to create $name.gif"
    fi
    echo
done

echo "🎉 All demo GIFs generated successfully!"
echo
echo "Generated files:"
ls -la *.gif
echo
echo "💡 You can now use these GIFs in your README.md or documentation"
