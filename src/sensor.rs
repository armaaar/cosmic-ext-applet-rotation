// SPDX-License-Identifier: GPL-3.0-only

#![allow(dead_code)]

use cosmic::iced::Subscription;
use cosmic::iced_futures::stream;
use futures_util::StreamExt;

#[zbus::proxy(
    interface = "net.hadess.SensorProxy",
    default_service = "net.hadess.SensorProxy",
    default_path = "/net/hadess/SensorProxy"
)]
trait SensorProxy {
    fn claim_accelerometer(&self) -> zbus::Result<()>;
    fn release_accelerometer(&self) -> zbus::Result<()>;

    #[zbus(property)]
    fn accelerometer_orientation(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn has_accelerometer(&self) -> zbus::Result<bool>;
}

/// Events emitted by the sensor subscription.
#[derive(Debug, Clone)]
pub enum SensorEvent {
    /// Accelerometer orientation changed.
    OrientationChanged(String),
    /// Sensor is not available (no iio-sensor-proxy or no accelerometer).
    Unavailable,
}

/// Creates a cosmic subscription that streams accelerometer orientation changes.
/// Uses system D-Bus -> net.hadess.SensorProxy.
pub fn subscription() -> Subscription<SensorEvent> {
    Subscription::run(sensor_stream)
}

fn sensor_stream() -> impl futures_util::Stream<Item = SensorEvent> {
    stream::channel(16, async |mut output| {
        use futures_util::SinkExt;

        let connection = match zbus::Connection::system().await {
            Ok(c) => c,
            Err(_) => {
                let _ = output.send(SensorEvent::Unavailable).await;
                return;
            }
        };

        let proxy = match SensorProxyProxy::new(&connection).await {
            Ok(p) => p,
            Err(_) => {
                let _ = output.send(SensorEvent::Unavailable).await;
                return;
            }
        };

        match proxy.has_accelerometer().await {
            Ok(true) => {}
            _ => {
                let _ = output.send(SensorEvent::Unavailable).await;
                return;
            }
        }

        if proxy.claim_accelerometer().await.is_err() {
            let _ = output.send(SensorEvent::Unavailable).await;
            return;
        }

        match proxy.accelerometer_orientation().await {
            Ok(initial) => {
                let _ = output.send(SensorEvent::OrientationChanged(initial)).await;
            }
            Err(_) => {
                let _ = output.send(SensorEvent::Unavailable).await;
                return;
            }
        }

        let mut changes = proxy.receive_accelerometer_orientation_changed().await;

        loop {
            match changes.next().await {
                Some(change) => match change.get().await {
                    Ok(orientation) => {
                        let _ = output
                            .send(SensorEvent::OrientationChanged(orientation))
                            .await;
                    }
                    Err(_) => {
                        let _ = output.send(SensorEvent::Unavailable).await;
                        return;
                    }
                },
                None => {
                    let _ = output.send(SensorEvent::Unavailable).await;
                    return;
                }
            }
        }
    })
}
