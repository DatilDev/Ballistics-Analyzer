#!/bin/bash
set -e

if [ ! -f /etc/arch-release ]; then
    echo "This script is for Arch Linux only"
    exit 1
fi

# Build and install
makepkg -si --noconfirm
echo "Installation complete!"