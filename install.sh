#!/usr/bin/env bash
#
# cmdx installer
# Builds and installs cmdx to your system
#

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Default values
PREFIX="${HOME}/.local/bin"
INSTALL_COMPLETIONS=false
UNINSTALL=false

# Detect shell
detect_shell() {
    if [ -n "$SHELL" ]; then
        basename "$SHELL"
    else
        echo "bash"
    fi
}

DETECTED_SHELL=$(detect_shell)

# Print functions
info() {
    echo -e "${BLUE}::${NC} $1"
}

success() {
    echo -e "${GREEN}✓${NC} $1"
}

warn() {
    echo -e "${YELLOW}!${NC} $1"
}

error() {
    echo -e "${RED}✗${NC} $1"
    exit 1
}

# Help message
show_help() {
    cat << EOF
${CYAN}cmdx installer${NC}

Usage: ./install.sh [OPTIONS]

Options:
    --prefix <PATH>     Install directory (default: ~/.local/bin)
    --completions       Install shell completions for detected shell ($DETECTED_SHELL)
    --uninstall         Remove cmdx installation
    -h, --help          Show this help message

Examples:
    ./install.sh                          # Build and install to ~/.local/bin
    ./install.sh --prefix /usr/local/bin  # System-wide install (requires sudo)
    ./install.sh --completions            # Include shell completions
    ./install.sh --uninstall              # Remove installation

EOF
}

# Check for required dependencies
check_dependencies() {
    info "Checking dependencies..."

    if ! command -v cargo &> /dev/null; then
        error "Rust/Cargo not found. Install from https://rustup.rs"
    fi

    CARGO_VERSION=$(cargo --version)
    success "Found $CARGO_VERSION"
}

# Build release binary
build() {
    info "Building release binary..."

    if ! cargo build --release; then
        error "Build failed"
    fi

    success "Build complete"
}

# Install binary
install_binary() {
    info "Installing to $PREFIX..."

    # Create directory if needed
    if [ ! -d "$PREFIX" ]; then
        mkdir -p "$PREFIX" || error "Failed to create $PREFIX"
    fi

    # Copy binary
    if [ -f "target/release/cmdx" ]; then
        cp target/release/cmdx "$PREFIX/cmdx" || error "Failed to copy binary"
        chmod +x "$PREFIX/cmdx"
        success "Installed cmdx to $PREFIX/cmdx"
    else
        error "Binary not found at target/release/cmdx"
    fi

    # Check if PREFIX is in PATH
    if [[ ":$PATH:" != *":$PREFIX:"* ]]; then
        warn "$PREFIX is not in your PATH"
        echo ""
        echo "Add this to your shell config (~/.bashrc, ~/.zshrc, etc.):"
        echo ""
        echo "    export PATH=\"\$PATH:$PREFIX\""
        echo ""
    fi
}

# Install shell completions
install_completions() {
    info "Installing $DETECTED_SHELL completions..."

    local cmdx_bin="$PREFIX/cmdx"

    if [ ! -f "$cmdx_bin" ]; then
        error "cmdx binary not found. Run install first."
    fi

    case "$DETECTED_SHELL" in
        bash)
            local completion_dir="${XDG_DATA_HOME:-$HOME/.local/share}/bash-completion/completions"
            mkdir -p "$completion_dir"
            "$cmdx_bin" completions bash > "$completion_dir/cmdx"
            success "Installed bash completions to $completion_dir/cmdx"
            echo ""
            echo "Completions will be available in new shell sessions."
            echo "To use now, run: source $completion_dir/cmdx"
            ;;
        zsh)
            local completion_dir="${XDG_DATA_HOME:-$HOME/.local/share}/zsh/site-functions"
            mkdir -p "$completion_dir"
            "$cmdx_bin" completions zsh > "$completion_dir/_cmdx"
            success "Installed zsh completions to $completion_dir/_cmdx"
            echo ""
            echo "Add this to your ~/.zshrc if not already present:"
            echo ""
            echo "    fpath=($completion_dir \$fpath)"
            echo "    autoload -Uz compinit && compinit"
            ;;
        fish)
            local completion_dir="${XDG_CONFIG_HOME:-$HOME/.config}/fish/completions"
            mkdir -p "$completion_dir"
            "$cmdx_bin" completions fish > "$completion_dir/cmdx.fish"
            success "Installed fish completions to $completion_dir/cmdx.fish"
            ;;
        *)
            warn "Unknown shell: $DETECTED_SHELL"
            echo "Generate completions manually: cmdx completions <shell>"
            ;;
    esac
}

# Uninstall
uninstall() {
    info "Uninstalling cmdx..."

    local removed=false

    # Remove binary
    if [ -f "$PREFIX/cmdx" ]; then
        rm "$PREFIX/cmdx"
        success "Removed $PREFIX/cmdx"
        removed=true
    fi

    # Remove completions
    case "$DETECTED_SHELL" in
        bash)
            local comp="${XDG_DATA_HOME:-$HOME/.local/share}/bash-completion/completions/cmdx"
            if [ -f "$comp" ]; then
                rm "$comp"
                success "Removed bash completions"
                removed=true
            fi
            ;;
        zsh)
            local comp="${XDG_DATA_HOME:-$HOME/.local/share}/zsh/site-functions/_cmdx"
            if [ -f "$comp" ]; then
                rm "$comp"
                success "Removed zsh completions"
                removed=true
            fi
            ;;
        fish)
            local comp="${XDG_CONFIG_HOME:-$HOME/.config}/fish/completions/cmdx.fish"
            if [ -f "$comp" ]; then
                rm "$comp"
                success "Removed fish completions"
                removed=true
            fi
            ;;
    esac

    if [ "$removed" = true ]; then
        success "Uninstall complete"
        echo ""
        echo "Note: Your command store (~/.config/cmdx) was preserved."
        echo "Remove manually if no longer needed: rm -rf ~/.config/cmdx"
    else
        warn "Nothing to uninstall"
    fi
}

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --prefix)
            PREFIX="$2"
            shift 2
            ;;
        --completions)
            INSTALL_COMPLETIONS=true
            shift
            ;;
        --uninstall)
            UNINSTALL=true
            shift
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        *)
            error "Unknown option: $1. Use --help for usage."
            ;;
    esac
done

# Main
echo ""
echo -e "${CYAN}cmdx installer${NC}"
echo ""

if [ "$UNINSTALL" = true ]; then
    uninstall
else
    check_dependencies
    build
    install_binary

    if [ "$INSTALL_COMPLETIONS" = true ]; then
        install_completions
    fi

    echo ""
    success "Installation complete!"
    echo ""
    echo "Get started:"
    echo "    cmdx init           # Initialize command store"
    echo "    cmdx --help         # Show all commands"
    echo ""
fi
