// SPDX-License-Identifier: GPL-3.0-only

use std::process::{Command, Output};

/// Returns a `Command` to invoke wlr-randr, accounting for the Flatpak sandbox.
/// Inside Flatpak, uses `flatpak-spawn --host wlr-randr` so the binary runs on the host.
fn wlr_randr_command() -> Command {
    if std::env::var_os("FLATPAK_ID").is_some() {
        let mut cmd = Command::new("flatpak-spawn");
        cmd.arg("--host").arg("wlr-randr");
        cmd
    } else {
        Command::new("wlr-randr")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Orientation {
    #[default]
    Normal,
    LeftUp,
    RightUp,
    BottomUp,
}

impl Orientation {
    pub fn from_sensor_string(s: &str) -> Option<Self> {
        match s {
            "normal" => Some(Self::Normal),
            "left-up" => Some(Self::LeftUp),
            "right-up" => Some(Self::RightUp),
            "bottom-up" => Some(Self::BottomUp),
            _ => None,
        }
    }

    pub fn to_wlr_transform(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::LeftUp => "90",
            Self::RightUp => "270",
            Self::BottomUp => "180",
        }
    }

    pub fn fl_key(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::LeftUp => "left-up",
            Self::RightUp => "right-up",
            Self::BottomUp => "bottom-up",
        }
    }
}

pub fn detect_display() -> Option<String> {
    let output = match wlr_randr_command().output() {
        Ok(o) => o,
        Err(e) => {
            tracing::warn!("wlr-randr not available: {e}");
            return None;
        }
    };
    let stdout = String::from_utf8_lossy(&output.stdout);
    let name = parse_display_name(&stdout);
    if name.is_none() {
        tracing::warn!("no eDP display found in wlr-randr output");
    }
    name
}

pub fn parse_display_name(output: &str) -> Option<String> {
    for line in output.lines() {
        if line.starts_with("eDP") {
            return line.split_whitespace().next().map(|s| s.to_owned());
        }
    }
    None
}

pub fn apply_rotation(display: &str, orientation: Orientation) -> std::io::Result<Output> {
    let result = wlr_randr_command()
        .args([
            "--output",
            display,
            "--transform",
            orientation.to_wlr_transform(),
        ])
        .output();
    match &result {
        Ok(out) if !out.status.success() => {
            tracing::warn!(
                "wlr-randr exited with status {}: {}",
                out.status,
                String::from_utf8_lossy(&out.stderr)
            );
        }
        Err(e) => {
            tracing::warn!("failed to run wlr-randr: {e}");
        }
        _ => {}
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_known_orientations() {
        assert_eq!(
            Orientation::from_sensor_string("normal"),
            Some(Orientation::Normal)
        );
        assert_eq!(
            Orientation::from_sensor_string("left-up"),
            Some(Orientation::LeftUp)
        );
        assert_eq!(
            Orientation::from_sensor_string("right-up"),
            Some(Orientation::RightUp)
        );
        assert_eq!(
            Orientation::from_sensor_string("bottom-up"),
            Some(Orientation::BottomUp)
        );
    }

    #[test]
    fn parse_unknown_returns_none() {
        assert_eq!(Orientation::from_sensor_string("undefined"), None);
        assert_eq!(Orientation::from_sensor_string(""), None);
        assert_eq!(Orientation::from_sensor_string("garbage"), None);
    }

    #[test]
    fn transform_mapping() {
        assert_eq!(Orientation::Normal.to_wlr_transform(), "normal");
        assert_eq!(Orientation::LeftUp.to_wlr_transform(), "90");
        assert_eq!(Orientation::RightUp.to_wlr_transform(), "270");
        assert_eq!(Orientation::BottomUp.to_wlr_transform(), "180");
    }

    #[test]
    fn wlr_randr_command_native() {
        std::env::remove_var("FLATPAK_ID");
        let cmd = wlr_randr_command();
        assert_eq!(cmd.get_program(), "wlr-randr");
    }

    #[test]
    fn wlr_randr_command_flatpak() {
        std::env::set_var("FLATPAK_ID", "com.github.armaaar.CosmicAppletRotation");
        let cmd = wlr_randr_command();
        assert_eq!(cmd.get_program(), "flatpak-spawn");
        let args: Vec<_> = cmd.get_args().collect();
        assert_eq!(args, &["--host", "wlr-randr"]);
        std::env::remove_var("FLATPAK_ID");
    }

    #[test]
    fn detect_display_parsing() {
        let sample = r#"eDP-1 "AU Optronics 0x243D" (eDP-1)
  Physical size: 260x170 mm
  Enabled: yes
HDMI-A-1 "Some Monitor" (HDMI-A-1)
  Physical size: 527x296 mm
  Enabled: no
"#;
        assert_eq!(parse_display_name(sample), Some("eDP-1".to_owned()));

        assert_eq!(
            parse_display_name("HDMI-A-1 \"Monitor\"\n  Enabled: yes\n"),
            None
        );
        assert_eq!(parse_display_name(""), None);
    }
}
