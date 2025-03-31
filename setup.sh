#!/bin/bash

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "Rust is not installed. Please install Rust from https://www.rust-lang.org/tools/install"
    exit 1
fi

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "wasm-pack is not installed. Installing wasm-pack..."
    cargo install wasm-pack
fi

# Check if Node.js and npm are installed
if ! command -v node &> /dev/null || ! command -v npm &> /dev/null; then
    echo "Node.js or npm is not installed. Please install Node.js from https://nodejs.org/"
    exit 1
fi

# Install npm dependencies
echo "Installing npm dependencies..."
npm install

echo "Setup completed successfully!"
echo "You can now run 'npm start' to start the development server."