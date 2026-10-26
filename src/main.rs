// SPDX-License-Identifier: GPL-3.0-only

mod app;
mod config;
mod i18n;
use cosmic_ext_applet_rotation::rotation;
use cosmic_ext_applet_rotation::sensor;

fn main() -> cosmic::iced::Result {
    let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();
    i18n::init(&requested_languages);
    cosmic::applet::run::<app::AppModel>(())
}
