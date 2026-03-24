#!/bin/bash
set -euo pipefail

# =============================================================================
# Devcontainer post-create setup for cosmic-applet-rotation
# =============================================================================

# --- System libraries required by libcosmic (the COSMIC GUI toolkit) ---------
# libcosmic depends on native C libraries for rendering, input, and Wayland
# support. These are build-time dependencies that cannot be installed via
# cargo or devcontainer features.
sudo apt-get update
sudo apt-get install -y --no-install-recommends \
    cmake              `# Build system used by some native dependencies` \
    libexpat1-dev      `# XML parser, required by fontconfig` \
    libfontconfig-dev  `# Font configuration, required by cosmic text rendering` \
    libfreetype-dev    `# Font rasterizer, required by cosmic text rendering` \
    libxkbcommon-dev   `# Keyboard handling for Wayland compositors` \
    pkg-config         `# Helps cargo find native libraries during compilation` \
    libwayland-dev     `# Wayland protocol libraries, required by winit/wayland backend` \
    libdbus-1-dev      `# D-Bus IPC, required by zbus for sensor communication` \
    libinput-dev       `# Input device handling for Wayland` \
    libssl-dev         `# TLS support, required by various dependencies` \
    libudev-dev        `# Device enumeration, required by libinput`

# --- Cargo tools for quality checks and packaging ----------------------------

# Bootstrap cargo-binstall: downloads pre-built binaries instead of compiling from source.
# This makes subsequent tool installs seconds instead of minutes.
curl -L --proto '=https' --tlsv1.2 -sSf \
    https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh \
    | bash

# Checks dependencies for known security vulnerabilities via the RustSec advisory database
cargo binstall -y cargo-audit

# Lints dependencies for license compliance, banned crates, and advisories.
# Configured via deny.toml in the project root.
cargo binstall -y cargo-deny

# Builds Debian (.deb) packages from Cargo.toml metadata.
# Used by the release workflow to produce installable packages.
cargo binstall -y cargo-deb

# --- Python tools -------------------------------------------------------------

# Spec-kit: toolkit for spec-driven development with AI coding agents.
# Provides structured workflow (specify -> plan -> tasks -> implement).
# https://github.com/github/spec-kit
uv tool install specify-cli --from git+https://github.com/github/spec-kit.git

# --- Initial build cache ------------------------------------------------------

# Run an initial build to populate the target directory with correct permissions.
# Some devcontainer filesystems (e.g., bind mounts) can cause build-script
# binaries to lose execute permissions, leading to "Permission denied" errors
# on subsequent builds. Building once here ensures the cache is usable.
cargo build --release 2>&1 | tail -1

# --- Git hooks ----------------------------------------------------------------

# Registers pre-commit hooks defined in .pre-commit-config.yaml.
# Runs formatting, linting, and test checks before each commit.
pre-commit install
