// SPDX-License-Identifier: GPL-3.0-only

//! Integration tests — require a running system D-Bus with iio-sensor-proxy.
//! Run on real hardware: `cargo test -- --ignored`

#[tokio::test]
#[ignore]
async fn sensor_proxy_connects() {
    let connection = zbus::Connection::system()
        .await
        .expect("failed to connect to system D-Bus");

    #[zbus::proxy(
        interface = "net.hadess.SensorProxy",
        default_service = "net.hadess.SensorProxy",
        default_path = "/net/hadess/SensorProxy"
    )]
    trait SensorProxy {
        #[zbus(property)]
        fn has_accelerometer(&self) -> zbus::Result<bool>;
    }

    let proxy = SensorProxyProxy::new(&connection)
        .await
        .expect("failed to create SensorProxy");

    proxy
        .has_accelerometer()
        .await
        .expect("has_accelerometer() should return Ok");
}

#[tokio::test]
#[ignore]
async fn sensor_proxy_reads_orientation() {
    let connection = zbus::Connection::system()
        .await
        .expect("failed to connect to system D-Bus");

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
    }

    let proxy = SensorProxyProxy::new(&connection)
        .await
        .expect("failed to create SensorProxy");

    proxy
        .claim_accelerometer()
        .await
        .expect("claim_accelerometer() should succeed");

    let orientation = proxy
        .accelerometer_orientation()
        .await
        .expect("accelerometer_orientation() should return Ok");

    let known = ["normal", "left-up", "right-up", "bottom-up"];
    assert!(
        known.contains(&orientation.as_str()),
        "unexpected orientation: {orientation}"
    );

    proxy
        .release_accelerometer()
        .await
        .expect("release_accelerometer() should succeed");
}

#[test]
#[ignore]
fn wlr_randr_available() {
    let status = std::process::Command::new("which")
        .arg("wlr-randr")
        .status()
        .expect("failed to run which");
    assert!(status.success(), "wlr-randr not found in PATH");
}

#[test]
#[ignore]
fn detect_display_finds_edp() {
    let name = cosmic_applet_rotation::rotation::detect_display();
    assert!(
        name.is_some(),
        "detect_display() should return Some on hardware with an eDP display"
    );
    let name = name.unwrap();
    assert!(
        name.starts_with("eDP"),
        "display name should start with eDP, got: {name}"
    );
}
