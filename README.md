# COSMIC Rotation Applet

[![CI](https://github.com/armaaar/cosmic-applet-rotation/actions/workflows/ci.yml/badge.svg)](https://github.com/armaaar/cosmic-applet-rotation/actions/workflows/ci.yml)
[![License: GPL-3.0](https://img.shields.io/badge/License-GPL--3.0-blue.svg)](LICENSE)

A native [COSMIC](https://github.com/pop-os/cosmic-epoch) panel applet for automatic screen rotation on Linux devices with accelerometers (tablets, convertibles). Monitors the accelerometer via `iio-sensor-proxy` over D-Bus and rotates the display using `wlr-randr`.

![Applet demo](resources/screenshots/demo.gif)

## Features

- Automatic screen rotation based on accelerometer orientation
- Toggle auto-rotate on/off from the panel
- Persistent settings via `cosmic-config`
- Supports normal, left-up, right-up, and bottom-up orientations
- Graceful degradation when sensor or display tools are unavailable
- Sensor reconnection with exponential backoff

## Runtime Dependencies

- [COSMIC desktop environment](https://github.com/pop-os/cosmic-epoch)
- [`iio-sensor-proxy`](https://gitlab.freedesktop.org/hadess/iio-sensor-proxy) — provides accelerometer data over D-Bus
- [`wlr-randr`](https://sr.ht/~emersion/wlr-randr/) — applies display rotation on wlr-based Wayland compositors

On Debian/Ubuntu-based systems:

```bash
sudo apt install iio-sensor-proxy wlr-randr
```

Verify your device has an accelerometer and the sensor service is running:

```bash
# Check if the sensor proxy is active
systemctl status iio-sensor-proxy

# Test accelerometer readings
monitor-sensor
```

## Installation

### From GitHub Releases

Download the latest `.deb` package or binary from [Releases](https://github.com/armaaar/cosmic-applet-rotation/releases). Tag pushes (`v*`) trigger automatic builds.

### Building from source

#### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2021 or later)
- [just](https://github.com/casey/just#installation) — command runner

#### Using the devcontainer (recommended)

Open in any [devcontainer](https://containers.dev)-compatible editor (VS Code, Cursor, etc.). All build dependencies are installed automatically.

#### Manual build

Install system dependencies:

```bash
sudo apt install cmake libexpat1-dev libfontconfig-dev libfreetype-dev \
  libxkbcommon-dev pkg-config libwayland-dev libdbus-1-dev \
  libinput-dev libssl-dev libudev-dev
```

Then build and install:

```bash
just build-release    # Release build
just install          # Install to /usr/bin + desktop entry
```

## Usage

1. Add the applet to your COSMIC panel
2. Click the applet icon to open the popup
3. Toggle auto-rotate on/off
4. The applet auto-detects your internal display (`eDP*`) and applies rotation

### Orientation Mapping

| Device position | Screen rotation |
|-----------------|-----------------|
| Normal          | 0° (normal)     |
| Right side up   | 270°            |
| Left side up    | 90°             |
| Upside down     | 180°            |

### Flatpak

See [docs/flatpak.md](docs/flatpak.md) for local Flatpak build instructions and COSMIC Store publishing guide.

## Development

```bash
just run              # Run with RUST_BACKTRACE=full
just quality          # Run all checks (fmt, clippy, test, deny)
just test-integration # Integration tests (requires real hardware + Wayland)
```

See [docs/architecture.md](docs/architecture.md) for module structure and design decisions, and [docs/testing.md](docs/testing.md) for the full testing strategy.

## Releasing

Push a version tag to trigger the release workflow:

```bash
git tag v0.1.0
git push origin v0.1.0
```

This builds the release binary + `.deb` package and publishes them as a GitHub Release.

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## License

GPL-3.0
