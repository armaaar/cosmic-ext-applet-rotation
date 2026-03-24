# Flatpak

A `.flatpak` bundle is published alongside the binary and `.deb` in each [GitHub Release](https://github.com/armaaar/cosmic-applet-rotation/releases). The Cargo sources are generated automatically during the release build.

## Local Build & Test

```bash
# Install flatpak-builder if not already installed
sudo apt install flatpak-builder

# Install the Freedesktop runtime and SDK
flatpak install flathub org.freedesktop.Platform//24.08 org.freedesktop.Sdk//24.08
flatpak install flathub org.freedesktop.Sdk.Extension.rust-stable//24.08

# Generate vendored Cargo sources
pip install aiohttp toml
curl -sL -o flatpak-cargo-generator.py \
  https://raw.githubusercontent.com/nickvdyck/flatpak-builder-tools/master/cargo/flatpak-cargo-generator.py
python3 flatpak-cargo-generator.py Cargo.lock -o flatpak/generated-sources.json

# Build and install
flatpak-builder --user --install --force-clean build flatpak/com.github.armaaar.CosmicAppletRotation.yml

# Run
flatpak run com.github.armaaar.CosmicAppletRotation
```

> **Note:** Inside the Flatpak sandbox, `wlr-randr` is invoked on the host via `flatpak-spawn --host`. The host must have `wlr-randr` and `iio-sensor-proxy` installed.

## Publishing to COSMIC Store

The COSMIC Store discovers apps through [Flathub](https://flathub.org). To publish:

1. Test the Flatpak build locally (see above)
2. Fork [github.com/flathub/flathub](https://github.com/flathub/flathub)
3. Create a branch from `new-pr` (not `master`)
4. Add a directory `com.github.armaaar.CosmicAppletRotation` with the Flatpak manifest
5. Open a PR titled `Add com.github.armaaar.CosmicAppletRotation`
6. Flathub reviewers will review the manifest and permissions

See the [Flathub submission guide](https://docs.flathub.org/docs/for-app-authors/submission) for full requirements.
