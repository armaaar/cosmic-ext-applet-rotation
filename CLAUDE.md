# COSMIC Rotation Applet

## Overview

COSMIC panel applet for automatic screen rotation on devices with accelerometers. Built with libcosmic (Rust), follows the MVU (Model-View-Update) pattern from the iced GUI framework.

## Build & Run

```bash
just build-release    # Release build
just run              # Run with RUST_BACKTRACE=full
just install          # Install to /usr/bin + desktop entry
just uninstall        # Remove installed files
```

## Quality Commands

```bash
cargo fmt -- --check                       # Format check
cargo clippy --all-targets -- -D warnings  # Lint
cargo test --all                           # Unit tests
cargo deny check licenses advisories       # License & vulnerability check
cargo audit                                # Security audit
just quality                               # Run all checks at once
```

## Architecture

- **Entry point**: `src/main.rs` — initializes i18n, runs `cosmic::applet::run::<AppModel>()`
- **App model**: `src/app.rs` — `AppModel` struct + `cosmic::Application` trait impl (view, update, subscription)
- **Config**: `src/config.rs` — `Config` struct with `CosmicConfigEntry` derive, persisted via cosmic-config
- **Sensor**: `src/sensor.rs` — zbus D-Bus proxy to `net.hadess.SensorProxy` (system bus), streams `AccelerometerOrientation`
- **Rotation**: `src/rotation.rs` — `Orientation` enum, sensor string parsing, `wlr-randr` execution
- **i18n**: `src/i18n.rs` — fluent localization, `fl!()` macro

## Commit & Versioning Conventions

- Use [Conventional Commits](https://www.conventionalcommits.org/): `type(scope): description`
  - Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`, `ci`, `build`, `perf`
  - Scopes (optional): `sensor`, `rotation`, `config`, `ui`, `i18n`, `ci`, `devcontainer`, `flatpak`
  - Breaking changes: append `!` after type/scope (e.g., `feat!: ...`) and include `BREAKING CHANGE:` in footer
- Follow [Semantic Versioning](https://semver.org/) (MAJOR.MINOR.PATCH):
  - `fix` → PATCH bump, `feat` → MINOR bump, breaking change → MAJOR bump
  - Version lives in `Cargo.toml` `[package].version`
  - Git tags: `v0.1.0`, `v0.2.0`, etc. — tag push triggers the release workflow

## Key Conventions

- Use `cosmic::applet::run` (not `cosmic::app::run`) — this is a panel applet, not a standalone app
- Sensor proxy lives on the **system** D-Bus bus (`zbus::Connection::system()`)
- Must call `ClaimAccelerometer()` before reading orientation
- Shell out to `wlr-randr` for rotation; in Flatpak, uses `flatpak-spawn --host wlr-randr` (see `wlr_randr_command()` in `rotation.rs`)
- Auto-detect internal display by matching `eDP*` output names

## Runtime Dependencies

- `iio-sensor-proxy` — provides accelerometer over D-Bus
- `wlr-randr` — applies display transforms on wlr-based compositors

## Orientation Mapping

| Sensor string | wlr-randr transform |
|---------------|---------------------|
| `normal`      | `normal`            |
| `right-up`    | `270`               |
| `left-up`     | `90`                |
| `bottom-up`   | `180`               |
