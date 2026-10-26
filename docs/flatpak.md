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
flatpak-builder --user --install --force-clean build flatpak/com.github.armaaar.CosmicExtAppletRotation.json \
  --override-source=cosmic-ext-applet-rotation=type:dir,path=$(pwd)

# Run
flatpak run com.github.armaaar.CosmicExtAppletRotation
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
3. Update the source in `flatpak/com.github.armaaar.CosmicExtAppletRotation.json` to the latest release. Use `"commit": "<sha>"` pinned to the release tag's commit, which is what currently merged apps in cosmic-flatpak do, and keep `runtime-version` on the runtime the other apps there use
4. Open a PR to [pop-os/cosmic-flatpak](https://github.com/pop-os/cosmic-flatpak) adding:
   ```
   app/com.github.armaaar.CosmicExtAppletRotation/
   ├── com.github.armaaar.CosmicExtAppletRotation.json   # manifest
   └── cargo-sources.json                              # vendored deps
   ```
5. Reviewers will review and merge; the applet then appears in the COSMIC Store

### cosmic-flatpak PR requirements (why the first submission was closed)

The first submission ([pop-os/cosmic-flatpak#140](https://github.com/pop-os/cosmic-flatpak/pull/140)) was closed by a maintainer with a pointer to the [COSMIC Trademark Policy](https://github.com/pop-os/cosmic-epoch/blob/master/TRADEMARK.md). The policy reserves the `cosmic-` package namespace and `com.system76.` app IDs for official software and asks third-party applets to use `cosmic-ext-`, which is why the project was renamed in 0.2.0. When submitting:

- Keep the `cosmic-ext-` name and the `CosmicExt` app ID. Describe the applet as "for the COSMIC™ desktop", not as a COSMIC applet.
- Use the repository's PR template as-is and tick its checklist; a CI check closes PRs without it.
- The template forbids LLM-generated content in the PR other than your own app code and manifest, including the PR description and commit message. Write those yourself.

### Optional: Community Visibility

Add the applet to [cosmic-project-collection applets.ron](https://github.com/cosmic-utils/cosmic-project-collection/blob/main/applets.ron) for listing on the community project catalog.

### Adding the cosmic-flatpak Remote (for users)

Users on non-Pop!_OS distros need to add the remote manually:

```bash
flatpak remote-add --if-not-exists --user cosmic \
  https://apt.pop-os.org/cosmic/cosmic.flatpakrepo
```

Pop!_OS ships with this remote pre-configured.
