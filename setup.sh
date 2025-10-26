#!/usr/bin/env bash
set -euo pipefail

# ==============================================================================
# KEAR_OS / Pico 2 RTOS environment setup
# ------------------------------------------------------------------------------
# This script installs:
#   - ARM bare-metal toolchain (gcc-arm-none-eabi, g++)
#   - CMake, Ninja, and build dependencies
#   - Rust target for Cortex-M33
#   - Pico SDK submodules
#   - Environment variables for CMake and toolchain
# ==============================================================================

echo "[1/6] Updating system packages..."
sudo apt update -y

echo "[2/6] Installing ARM toolchain & build tools..."
sudo apt install -y \
    cmake ninja-build make git \
    gcc-arm-none-eabi

echo "[3/6] Installing Rust components..."
rustup update
rustup target add thumbv8m.main-none-eabi
rustup component add rust-src

echo "[4/6] Checking/initializing Pico SDK submodule..."
if [ ! -d "sdk" ]; then
    echo "[-] SDK directory not found. Please clone pico-sdk as a submodule first."
    echo "   Example: git submodule add https://github.com/raspberrypi/pico-sdk.git sdk"
else
    git -C sdk submodule update --init --recursive
    echo "[+] Pico SDK initialized."
fi

echo "[5/6] Setting persistent environment variables..."

# Add ARM toolchain to PATH in future shells if missing
if ! grep -q "arm-none-eabi" ~/.bashrc; then
    echo "" >> ~/.bashrc
    echo "# >>> KEAR_OS toolchain setup >>>" >> ~/.bashrc
    echo "export PATH=\$PATH:/usr/bin" >> ~/.bashrc
    echo "export CC=arm-none-eabi-gcc" >> ~/.bashrc
    echo "export CXX=arm-none-eabi-g++" >> ~/.bashrc
    echo "export PICO_SDK_PATH=\$PWD/sdk" >> ~/.bashrc
    echo "# <<< KEAR_OS toolchain setup <<<" >> ~/.bashrc
    echo "[+] Added environment variables to ~/.bashrc"
else
    echo "[.]  Environment variables already present in ~/.bashrc"
fi

# Export them for the current shell too
export CC=arm-none-eabi-gcc
export CXX=arm-none-eabi-g++
export PICO_SDK_PATH="$PWD/sdk"

echo "[6/6] Verifying toolchain..."
which arm-none-eabi-gcc >/dev/null || { echo "[-] GCC not found in PATH"; exit 1; }
arm-none-eabi-gcc --version | head -n 1
rustup show | grep 'active toolchain' || true

echo ""
echo "[+] Environment ready!"
echo "Now you can build with:"
echo "   cargo build --release"
echo ""
echo "If you want to flash directly:"
echo "   cargo run --release"