name := 'cosmic-applet-rotation'
appid := 'com.github.armaaar.CosmicAppletRotation'

rootdir := ''
prefix := '/usr'

base-dir := absolute_path(clean(rootdir / prefix))
cargo-target-dir := env('CARGO_TARGET_DIR', 'target')
bin-dst := base-dir / 'bin' / name
desktop-dst := base-dir / 'share' / 'applications' / appid + '.desktop'

# Default recipe
default: build-release

# Remove build artifacts
clean:
    cargo clean

# Build with debug profile
build-debug *args:
    cargo build {{args}}

# Build with release profile
build-release *args: (build-debug '--release' args)

# Run clippy
check *args:
    cargo clippy --all-features {{args}} -- -W clippy::pedantic

# Run the applet for testing
run *args:
    env RUST_BACKTRACE=full cargo run --release {{args}}

# Run all quality checks
quality:
    cargo fmt -- --check
    cargo clippy --all-targets -- -D warnings
    cargo test --all
    cargo deny check licenses advisories

# Run integration tests (requires real hardware + Wayland session)
test-integration:
    cargo test -- --ignored

# Install files
install:
    install -Dm0755 {{ cargo-target-dir / 'release' / name }} {{bin-dst}}
    install -Dm0644 resources/app.desktop {{desktop-dst}}

# Uninstall files
uninstall:
    rm -f {{bin-dst}} {{desktop-dst}}
