#!/bin/bash
# cmdx install script - builds and copies to dotfiles

set -e

GREEN='\033[0;32m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
DOTFILES_DIR="${DOTFILES_DIR:-$HOME/dotfiles}"

echo "Building cmdx..."
cd "$PROJECT_DIR"
cargo build --release

echo "Copying to dotfiles..."
mkdir -p "$DOTFILES_DIR/cmdx/.local/bin"
cp "$PROJECT_DIR/target/release/cmdx" "$DOTFILES_DIR/cmdx/.local/bin/"

echo -e "${GREEN}✓${NC} Installed cmdx to $DOTFILES_DIR/cmdx/.local/bin/cmdx"
echo ""
echo "To activate, run:"
echo "  cd $DOTFILES_DIR && stow cmdx"
