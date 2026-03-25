# Flatpak

The applet is distributed via the [cosmic-flatpak](https://github.com/pop-os/cosmic-flatpak) repository, which the COSMIC Store uses to discover community applets. Flathub does not accept panel applets, so `cosmic-flatpak` is the correct channel.

## Local Build & Test

```bash
# Install flatpak-builder if not already installed
sudo apt install flatpak-builder

# Install the Freedesktop runtime, SDK, and COSMIC BaseApp
flatpak install flathub org.freedesktop.Platform//24.08 org.freedesktop.Sdk//24.08
flatpak install flathub org.freedesktop.Sdk.Extension.rust-stable//24.08
flatpak install flathub com.system76.Cosmic.BaseApp//stable

# Generate vendored Cargo sources
pip install aiohttp tomlkit
curl -sL -o flatpak-cargo-generator.py \
  https://raw.githubusercontent.com/flatpak/flatpak-builder-tools/master/cargo/flatpak-cargo-generator.py
python3 flatpak-cargo-generator.py Cargo.lock -o flatpak/cargo-sources.json

# Build and install (override the git source to use local files)
flatpak-builder --user --install --force-clean build flatpak/com.github.armaaar.CosmicAppletRotation.json \
  --override-source=cosmic-applet-rotation=type:dir,path=$(pwd)

# Run
flatpak run com.github.armaaar.CosmicAppletRotation
```

> **Note:** Inside the Flatpak sandbox, `wlr-randr` is invoked on the host via `flatpak-spawn --host`. The host must have `wlr-randr` and `iio-sensor-proxy` installed.

## Publishing to the COSMIC Store

The COSMIC Store discovers applets from the [cosmic-flatpak](https://github.com/pop-os/cosmic-flatpak) repository maintained by System76.

### Steps

1. Test the Flatpak build locally (see above)
2. Generate `cargo-sources.json` from the current `Cargo.lock`:
   ```bash
   python3 flatpak-cargo-generator.py Cargo.lock -o flatpak/cargo-sources.json
   ```
3. Update the `tag` in `flatpak/com.github.armaaar.CosmicAppletRotation.json` to match the latest release tag
4. Open a PR to [pop-os/cosmic-flatpak](https://github.com/pop-os/cosmic-flatpak) adding:
   ```
   app/com.github.armaaar.CosmicAppletRotation/
   ├── com.github.armaaar.CosmicAppletRotation.json   # manifest
   └── cargo-sources.json                              # vendored deps
   ```
5. Reviewers will review and merge; the applet then appears in the COSMIC Store

### Optional: Community Visibility

Add the applet to [cosmic-project-collection applets.ron](https://github.com/cosmic-utils/cosmic-project-collection/blob/main/applets.ron) for listing on the community project catalog.

### Adding the cosmic-flatpak Remote (for users)

Users on non-Pop!_OS distros need to add the remote manually:

```bash
flatpak remote-add --if-not-exists --user cosmic \
  https://apt.pop-os.org/cosmic/cosmic.flatpakrepo
```

Pop!_OS ships with this remote pre-configured.
