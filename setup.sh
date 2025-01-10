#!/bin/bash

check_command() {
    if ! command -v $1 &> /dev/null; then
        return 1
    fi
    return 0
}

check_debian_package() {
    if dpkg -l "$1" 2>/dev/null | grep -q "^ii"; then
        return 0
    fi
    return 1
}

check_nlohmann_json() {
    if is_debian_based; then
        if check_debian_package "nlohmann-json3-dev"; then
            return 0
        fi
    else
        if [ -f "/usr/include/nlohmann/json.hpp" ] || \
           [ -f "/usr/local/include/nlohmann/json.hpp" ] || \
           [ -f "/usr/include/json/json.hpp" ]; then
            return 0
        fi
    fi
    return 1
}

check_gmp() {
    if is_debian_based; then
        if check_debian_package "libgmp-dev"; then
            return 0
        fi
    else
        if [ -f "/usr/include/gmp.h" ] && (ldconfig -p | grep -q "libgmp"); then
            return 0
        fi
    fi
    return 1
}

is_debian_based() {
    if [ -f "/etc/debian_version" ]; then
        return 0
    fi
    return 1
}

echo "Checking system dependencies..."

# Checks for nlohmann-json
if ! check_nlohmann_json; then
    echo "Missing: nlohmann-json library"
    if is_debian_based; then
        echo "On Debian/Ubuntu systems, install with: sudo apt install nlohmann-json3-dev"
    else
        echo "Please install nlohmann-json development library for your system"
    fi
else
    echo "✓ nlohmann-json library installed"
fi

# Checks for GMP
if ! check_gmp; then
    echo "Missing: GMP library"
    if is_debian_based; then
        echo "On Debian/Ubuntu systems, install with: sudo apt install libgmp-dev"
    else
        echo "Please install GMP development library for your system"
    fi
else
    echo "✓ GMP library installed"
fi

# Checks for NASM
if ! check_command "nasm"; then
    echo "Missing: NASM assembler"
    if is_debian_based; then
        echo "On Debian/Ubuntu systems, install with: sudo apt install nasm"
    else
        echo "Please install NASM for your system"
    fi
else
    echo "✓ NASM installed"
fi

# Checks for Node.js
if ! check_command "node"; then
    echo "Missing: Node.js"
    echo "Please install Node.js from https://nodejs.org/"
    exit 1
fi

# Checks Node.js version
NODE_VERSION=$(node -v | cut -d'v' -f2)
if [ "$(printf '%s\n' "18.0.0" "$NODE_VERSION" | sort -V | head -n1)" = "18.0.0" ]; then
    echo "✓ Node.js version $NODE_VERSION"
else
    echo "Node.js version must be 18.0.0 or higher (current: $NODE_VERSION)"
    echo "Please upgrade Node.js from https://nodejs.org/"
    exit 1
fi

# Checks for Go
if ! check_command "go"; then
    echo "Missing: Go programming language"
    echo "Please install Go from https://golang.org/doc/install"
    exit 1
fi

# Checks Go version
GO_VERSION=$(go version | awk '{print $3}' | cut -c 3-)
if [ "$(printf '%s\n' "1.22.0" "$GO_VERSION" | sort -V | head -n1)" = "1.22.0" ]; then
    echo "✓ Go version $GO_VERSION"
else
    echo "Go version must be 1.22.0 or higher (current: $GO_VERSION)"
    echo "Please upgrade Go from https://golang.org/doc/install"
    exit 1
fi

# Checks for snarkjs
if ! check_command "snarkjs"; then
    echo "Missing: snarkjs"
    read -p "Do you want to install snarkjs globally? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        npm install -g snarkjs@latest
    else
        echo "Please install snarkjs manually with: npm install -g snarkjs@latest"
        exit 1
    fi
else
    echo "✓ snarkjs installed"
fi

# Checks for Rust
if ! check_command "rustc"; then
    echo "Missing: Rust"
    echo "Please install Rust using:"
    echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Checks for Rust nightly
if ! rustup toolchain list | grep -q "nightly"; then
    echo "Installing Rust nightly..."
    rustup install nightly
else
    echo "✓ Rust nightly installed"
fi

# Checks for RISC-Zero target
if ! rustup target list | grep -q "riscv32imac-unknown-none-elf"; then
    echo "Adding RISC-Zero target..."
    rustup target add riscv32imac-unknown-none-elf
else
    echo "✓ RISC-Zero target installed"
fi

# Checks for cargo-risczero
if ! check_command "cargo-risczero"; then
    echo "Installing cargo-risczero..."
    cargo install cargo-risczero
    cargo risczero install
else
    echo "✓ cargo-risczero installed"
fi

# Checks for SP1
if ! check_command "sp1up"; then
    echo "Installing SP1..."
    curl -L https://sp1.succinct.xyz | bash
    sp1up
else
    echo "✓ SP1 installed"
fi

# Make build.sh executable
if [ -f "circuits/snarkjs_groth16/build.sh" ]; then
    chmod +x circuits/snarkjs_groth16/build.sh
    echo "✓ circuits/snarkjs_groth16/build.sh is executable"
else
    echo "Warning: circuits/snarkjs_groth16/build.sh not found"
fi

# Checks if any of the required dependencies were missing
MISSING_DEPS=0
! check_nlohmann_json && MISSING_DEPS=1
! check_gmp && MISSING_DEPS=1
! check_command "nasm" && MISSING_DEPS=1

echo "Setup check completed!"
if [ $MISSING_DEPS -eq 0 ]; then
    echo "✓ All required dependencies are installed!"
else
    echo "⚠ Some dependencies are missing. Please install them before proceeding."
    exit 1
fi 