use futures_util::StreamExt;
use zbus::{Connection, Proxy};

const DESKTOP_DESTINATION: &str = "org.freedesktop.portal.Desktop";
const DESKTOP_PATH: &str = "/org/freedesktop/portal/desktop";
const APPEARANCE_INTERFACE: &str = "org.freedesktop.appearance";

/// Start the daemon. This creates an async runtime with tasks running forever.
pub async fn start_daemon() {
    // Create D-Bus connection
    let conn = Connection::session()
        .await
        .expect("Should be able to create D-Bus connection");

    // Create proxy for appearance
    let proxy = Proxy::new(
        &conn,
        DESKTOP_DESTINATION,
        DESKTOP_PATH,
        APPEARANCE_INTERFACE,
    )
    .await
    .expect("Should be able to create proxy");

    let mut event_stream = proxy.receive_all_signals().await.unwrap();
    while let Some(e) = event_stream.next().await {
        println!("{:?}", e);
    }

    // let mut color_scheme_stream: PropertyStream<u32> =
    //     proxy.receive_property_changed("color-scheme").await;
    //
    // while let Some(v) = color_scheme_stream.next().await {
    //     println!("{:?}", v.get().await.unwrap());
    // }
}
