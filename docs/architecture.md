# Architecture

## Overview

The applet follows the MVU (Model-View-Update) pattern from the iced GUI framework, as required by libcosmic. It runs as a panel applet via `cosmic::applet::run`.

## Module Structure

```
src/
├── main.rs       Entry point — initializes i18n, runs cosmic::applet::run::<AppModel>()
├── app.rs        AppModel + cosmic::Application trait (view, update, subscription)
├── config.rs     Config struct with CosmicConfigEntry — persists auto_rotate bool
├── sensor.rs     zbus D-Bus proxy to net.hadess.SensorProxy (system bus)
├── rotation.rs   Orientation enum, sensor string parsing, wlr-randr execution
├── i18n.rs       Fluent localization, fl!() macro
└── lib.rs        Re-exports sensor and rotation for integration tests
```

## Data Flow

```
iio-sensor-proxy (D-Bus)
    │
    ▼
sensor.rs ──subscription()──► SensorEvent::OrientationChanged(String)
    │
    ▼
app.rs ──update()──► Orientation::from_sensor_string()
    │
    ▼
rotation.rs ──apply_rotation()──► wlr-randr --output eDP-1 --transform {transform}
```

1. `sensor::subscription()` connects to the system D-Bus and streams orientation changes
2. `AppModel::update()` receives `SensorEvent`, parses the orientation string
3. `rotation::apply_rotation()` shells out to `wlr-randr` to apply the transform

## Key Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Rotation method | Shell out to `wlr-randr` | Simple, proven; native Wayland protocol later |
| Display detection | Auto-detect `eDP*` | Generic for laptops/tablets |
| Config | `cosmic-config` with `auto_rotate` bool | Minimal, extensible |
| License | GPL-3.0 | Matches COSMIC ecosystem |
| Packaging | GitHub Releases (binary + .deb), cosmic-flatpak (Flatpak) | .deb for direct install; Flatpak via cosmic-flatpak for COSMIC Store |
| Dev environment | Devcontainer | Zero local setup needed |

## Error Handling

- **Sensor unavailable**: yields `SensorEvent::Unavailable`, UI shows warning and dims toggler. Retries with exponential backoff (up to 5 attempts).
- **Display not found**: `detect_display()` returns `None`, rotation disabled, popup shows warning.
- **wlr-randr missing/fails**: logged via `tracing::warn!`, rotation silently skipped.
