#!/bin/bash

# Example usage script for notego

echo "Notego - Example Usage"
echo "======================"
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust from https://rustup.rs/"
    exit 1
fi

echo "Building notego..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed"
    exit 1
fi

echo "✓ Build successful!"
echo ""
echo "Example commands:"
echo ""
echo "1. Dry run (preview without writing):"
echo "   ./target/release/notego --folder \"logs\" --dry-run"
echo ""
echo "2. Export to ./out directory:"
echo "   ./target/release/notego --folder \"logs\""
echo ""
echo "3. Export with custom options:"
echo "   ./target/release/notego --folder \"logs\" --out ~/notes --ext md --date created"
echo ""
echo "Replace 'logs' with your Notes folder name."
