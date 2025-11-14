#!/bin/bash

# Build script for Solana client

echo "Building Solana client..."

# Check if solana-cli is installed
if ! command -v solana &> /dev/null
then
    echo "Solana CLI could not be found. Please install it first."
    exit 1
fi

# Build the client
echo "Compiling client..."
cargo build --lib

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "Build successful!"
else
    echo "Build failed!"
    exit 1
fi