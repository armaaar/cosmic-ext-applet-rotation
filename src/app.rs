// SPDX-License-Identifier: GPL-3.0-only

use crate::config::Config;
use crate::fl;
use crate::rotation::{self, Orientation};
use crate::sensor::{self, SensorEvent};
use cosmic::cosmic_config::{self, CosmicConfigEntry};
use cosmic::iced::{window::Id, Limits, Subscription};
use cosmic::iced_winit::commands::popup::{destroy_popup, get_popup};
use cosmic::prelude::*;
use cosmic::widget;

/// Application state for the rotation applet.
pub struct AppModel {
    core: cosmic::Core,
    popup: Option<Id>,
    config: Config,
    orientation: Orientation,
    sensor_available: bool,
    display_name: Option<String>,
}

impl Default for AppModel {
    fn default() -> Self {
        Self {
            core: cosmic::Core::default(),
            popup: None,
            config: Config::default(),
            orientation: Orientation::default(),
            sensor_available: true,
            display_name: None,
        }
    }
}

/// Messages emitted by the applet.
#[derive(Debug, Clone)]
pub enum Message {
    TogglePopup,
    PopupClosed(Id),
    UpdateConfig(Config),
    ToggleAutoRotate(bool),
    SensorEvent(SensorEvent),
}

impl cosmic::Application for AppModel {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = "com.github.armaaar.CosmicAppletRotation";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    fn init(
        core: cosmic::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<cosmic::Action<Self::Message>>) {
        let (config, is_first_run) = cosmic_config::Config::new(Self::APP_ID, Config::VERSION)
            .map(|context| match Config::get_entry(&context) {
                Ok(config) => (config, false),
                Err((_errors, config)) => (config, true),
            })
            .unwrap_or_else(|_| (Config::default(), true));

        let display_name = rotation::detect_display();

        // On first run, force the system state to match the default config
        // so the applet and system are in sync from the start.
        if is_first_run {
            if let Some(display) = &display_name {
                if !config.auto_rotate {
                    let _ = rotation::apply_rotation(display, Orientation::Normal);
                }
            }
        }

        let app = AppModel {
            core,
            config,
            orientation: Orientation::default(),
            sensor_available: true,
            display_name,
            ..Default::default()
        };

        (app, Task::none())
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let icon_name = if self.config.auto_rotate {
            "rotation-allowed-symbolic"
        } else {
            "rotation-locked-symbolic"
        };

        self.core
            .applet
            .icon_button(icon_name)
            .on_press(Message::TogglePopup)
            .into()
    }

    fn view_window(&self, _id: Id) -> Element<'_, Self::Message> {
        let orientation_label = match self.orientation {
            Orientation::Normal => fl!("normal"),
            Orientation::LeftUp => fl!("left-up"),
            Orientation::RightUp => fl!("right-up"),
            Orientation::BottomUp => fl!("bottom-up"),
        };

        let toggler = if self.sensor_available && self.display_name.is_some() {
            widget::toggler(self.config.auto_rotate).on_toggle(Message::ToggleAutoRotate)
        } else {
            widget::toggler(self.config.auto_rotate)
        };

        let mut content_list = widget::list_column()
            .padding(5)
            .spacing(0)
            .add(widget::settings::item(fl!("auto-rotate"), toggler))
            .add(widget::settings::item(
                fl!("orientation"),
                widget::text(orientation_label),
            ));

        if !self.sensor_available {
            content_list = content_list.add(widget::text(fl!("sensor-unavailable")));
        }

        if self.display_name.is_none() {
            content_list = content_list.add(widget::text(fl!("display-unavailable")));
        }

        self.core.applet.popup_container(content_list).into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let mut subscriptions = vec![self
            .core()
            .watch_config::<Config>(Self::APP_ID)
            .map(|update| Message::UpdateConfig(update.config))];

        if self.config.auto_rotate {
            subscriptions.push(sensor::subscription().map(Message::SensorEvent));
        }

        Subscription::batch(subscriptions)
    }

    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        match message {
            Message::UpdateConfig(config) => {
                self.config = config;
            }
            Message::ToggleAutoRotate(enabled) => {
                self.config.auto_rotate = enabled;
                if let Ok(context) = cosmic_config::Config::new(Self::APP_ID, Config::VERSION) {
                    _ = self.config.write_entry(&context);
                }
                if !enabled {
                    if let Some(display) = &self.display_name {
                        let _ = rotation::apply_rotation(display, Orientation::Normal);
                    }
                }
            }
            Message::TogglePopup => {
                return if let Some(p) = self.popup.take() {
                    destroy_popup(p)
                } else {
                    let new_id = Id::unique();
                    self.popup.replace(new_id);
                    let mut popup_settings = self.core.applet.get_popup_settings(
                        self.core.main_window_id().unwrap(),
                        new_id,
                        None,
                        None,
                        None,
                    );
                    popup_settings.positioner.size_limits = Limits::NONE
                        .max_width(372.0)
                        .min_width(300.0)
                        .min_height(100.0)
                        .max_height(400.0);
                    get_popup(popup_settings)
                };
            }
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
            }
            Message::SensorEvent(SensorEvent::OrientationChanged(s)) => {
                if let Some(orientation) = Orientation::from_sensor_string(&s) {
                    self.orientation = orientation;
                    if self.config.auto_rotate {
                        if let Some(display) = &self.display_name {
                            let _ = rotation::apply_rotation(display, orientation);
                        }
                    }
                }
            }
            Message::SensorEvent(SensorEvent::Unavailable) => {
                self.sensor_available = false;
            }
        }
        Task::none()
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }
}
