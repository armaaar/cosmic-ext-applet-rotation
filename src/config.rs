// SPDX-License-Identifier: GPL-3.0-only

use cosmic::cosmic_config::{self, cosmic_config_derive::CosmicConfigEntry, CosmicConfigEntry};

#[derive(Debug, Default, Clone, CosmicConfigEntry, Eq, PartialEq)]
#[version = 1]
pub struct Config {
    pub auto_rotate: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_auto_rotate_is_false() {
        // The first-run sync logic in app.rs relies on this default.
        // If changed, update the init() first-run branch accordingly.
        let config = Config::default();
        assert!(!config.auto_rotate);
    }
}
