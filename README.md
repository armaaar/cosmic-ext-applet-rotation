# COSMIC Rotation Applet

A native COSMIC panel applet for automatic screen rotation on Linux devices with accelerometers (tablets, convertibles). Monitors the accelerometer via `iio-sensor-proxy` over D-Bus and rotates the display using `wlr-randr`.

## Features

- Automatic screen rotation based on accelerometer orientation
- Toggle auto-rotate on/off from the panel
- Persistent settings via `cosmic-config`
- Supports normal, left-up, right-up, and bottom-up orientations

## Runtime Dependencies

- [COSMIC desktop environment](https://github.com/pop-os/cosmic-epoch)
- `iio-sensor-proxy` — provides accelerometer data over D-Bus
- `wlr-randr` — applies display rotation on wlr-based Wayland compositors

## Building

### Using the devcontainer (recommended)

Requires a [devcontainer](https://containers.dev)-compatible editor (VS Code, Cursor, etc.). The devcontainer installs all build dependencies automatically.

**First-time host setup** (run once before rebuilding the container, so auth persists across rebuilds):

```bash
git config --global user.name "Your Name"
git config --global user.email "you@example.com"
git config --global credential.helper store
echo "https://<username>:<token>@github.com" >> ~/.git-credentials
claude  # log in to Claude
```

### Manual build

Install the build dependencies listed in `.devcontainer/post-create.sh`, then:

```bash
just build-release    # Release build
just install          # Install to /usr/bin + desktop entry
```

## Quality Checks

```bash
just quality          # Run all checks (fmt, clippy, test, deny)
```

## License

GPL-3.0
