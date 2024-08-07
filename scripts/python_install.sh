#!/bin/bash

# Function to display installation instructions
show_instructions() {
    echo "Please follow the instructions below to install Python and pip for your operating system:"
    echo
    echo "### Windows"
    echo "1. Download and install Python from the official website: https://www.python.org/downloads/"
    echo "2. During installation, ensure that the 'Add Python to PATH' option is checked."
    echo "3. Open Command Prompt and run: python -m pip install --upgrade pip"
    echo
    echo "### Linux"
    echo "1. Update package list and install Python and pip:"
    echo "   sudo apt update"
    echo "   sudo apt install python3 python3-pip"
    echo
    echo "### macOS"
    echo "1. Install Python using Homebrew:"
    echo "   brew install python"
    echo "2. Verify pip installation:"
    echo "   python3 -m pip --version"
    echo
}

# Check if Python is installed
if ! command -v python3 &>/dev/null; then
    echo "Python is not installed. Please install Python."
    show_instructions
    exit 1
fi

# Check if pip is installed
if ! command -v pip3 &>/dev/null; then
    echo "pip is not installed. Please install pip."
    show_instructions
    exit 1
fi

# Install dependencies
pip3 install -r requirements.txt

echo "All dependencies are installed."
