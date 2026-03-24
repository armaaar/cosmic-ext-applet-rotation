# Testing Strategy

## Unit Tests (`cargo test`)

Run in CI on every push/PR. No special hardware required.

- `Orientation::from_sensor_string()` — all 4 valid strings + unknown inputs
- `Orientation::to_wlr_transform()` — correct transform per orientation
- `parse_display_name()` — display name extraction from sample wlr-randr output
- Config defaults and serialization

```bash
cargo test --all
```

## Integration Tests (`tests/integration.rs`)

Tagged with `#[ignore]` — skipped in CI, run manually on real hardware with a Wayland session and `iio-sensor-proxy` running.

- `sensor_proxy_connects` — verifies D-Bus connection to iio-sensor-proxy
- `sensor_proxy_reads_orientation` — reads and validates orientation string
- `wlr_randr_available` — checks wlr-randr is installed and runnable
- `detect_display_finds_edp` — verifies eDP display detection on real hardware

```bash
just test-integration
# or: cargo test -- --ignored
```

## Quality Gates

Run locally or in CI via `just quality`:

1. `cargo fmt -- --check` — formatting
2. `cargo clippy --all-targets -- -D warnings` — linting
3. `cargo test --all` — unit tests
4. `cargo deny check licenses advisories` — license and vulnerability audit

The release build (`just build-release`) is also verified in CI.

## Manual Verification Checklist

For testing on a real device before release:

- [ ] Applet appears in COSMIC panel
- [ ] Popup opens/closes on click
- [ ] Orientation updates when device rotates
- [ ] Screen rotates to match orientation
- [ ] Auto-rotate toggle works
- [ ] Setting persists across restarts
- [ ] Graceful behavior when iio-sensor-proxy not installed
- [ ] Graceful behavior when wlr-randr not installed
- [ ] Release workflow produces artifacts
