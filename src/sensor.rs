// SPDX-License-Identifier: GPL-3.0-only

use cosmic::iced::Subscription;
use cosmic::iced_futures::stream;
use futures_util::StreamExt;
use std::time::Duration;

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

        const MAX_RETRIES: u32 = 5;
        const BASE_BACKOFF_MS: u64 = 500;

        for attempt in 0..MAX_RETRIES {
            if attempt > 0 {
                let delay = Duration::from_millis(BASE_BACKOFF_MS * (1 << (attempt - 1)));
                tokio::time::sleep(delay).await;
            }

            let connection = match zbus::Connection::system().await {
                Ok(c) => c,
                Err(e) => {
                    tracing::warn!("sensor: D-Bus connection failed (attempt {attempt}): {e}");
                    continue;
                }
            };

            let proxy = match SensorProxyProxy::new(&connection).await {
                Ok(p) => p,
                Err(e) => {
                    tracing::warn!("sensor: proxy creation failed (attempt {attempt}): {e}");
                    continue;
                }
            };

            match proxy.has_accelerometer().await {
                Ok(true) => {}
                Ok(false) => {
                    tracing::warn!("sensor: no accelerometer detected");
                    let _ = output.send(SensorEvent::Unavailable).await;
                    return;
                }
                Err(e) => {
                    tracing::warn!(
                        "sensor: has_accelerometer check failed (attempt {attempt}): {e}"
                    );
                    continue;
                }
            }

            if let Err(e) = proxy.claim_accelerometer().await {
                tracing::warn!("sensor: claim_accelerometer failed (attempt {attempt}): {e}");
                continue;
            }

            let initial = match proxy.accelerometer_orientation().await {
                Ok(v) => v,
                Err(e) => {
                    tracing::warn!(
                        "sensor: read initial orientation failed (attempt {attempt}): {e}"
                    );
                    continue;
                }
            };
            let _ = output.send(SensorEvent::OrientationChanged(initial)).await;

            let mut changes = proxy.receive_accelerometer_orientation_changed().await;

            loop {
                match changes.next().await {
                    Some(change) => match change.get().await {
                        Ok(orientation) => {
                            let _ = output
                                .send(SensorEvent::OrientationChanged(orientation))
                                .await;
                        }
                        Err(e) => {
                            tracing::warn!("sensor: failed to read orientation change: {e}");
                            let _ = output.send(SensorEvent::Unavailable).await;
                            return;
                        }
                    },
                    None => {
                        tracing::warn!("sensor: orientation change stream ended");
                        let _ = output.send(SensorEvent::Unavailable).await;
                        return;
                    }
                }
            }
        }

        tracing::warn!("sensor: exhausted retries, sensor unavailable");
        let _ = output.send(SensorEvent::Unavailable).await;
    })
}
