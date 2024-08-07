#!/bin/bash

# Check if Python is installed
if ! command -v python3 &>/dev/null; then
    echo "Python is not installed. Please install Python."
    exit 1
fi

# Check if pip is installed
if ! command -v pip3 &>/dev/null; then
    echo "pip is not installed. Please install pip."
    exit 1
fi

# Install dependencies
pip3 install -r requirements.txt

echo "All dependencies are installed."
