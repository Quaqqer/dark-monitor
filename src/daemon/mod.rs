use futures_util::StreamExt;
use zbus::{proxy::PropertyStream, Connection, MatchRule, MessageStream, Proxy};

// const DESKTOP_DESTINATION: &str = "org.freedesktop.portal.Desktop";
// const DESKTOP_PATH: &str = "/org/freedesktop/portal/desktop";
// const APPEARANCE_INTERFACE: &str = "org.freedesktop.appearance";
//
// /// Start the daemon. This creates an async runtime with tasks running forever.
// pub async fn start_daemon() {
//     // Create D-Bus connection
//     // Create proxy for appearance
//     let proxy = Proxy::new(
//         &conn,
//         DESKTOP_DESTINATION,
//         DESKTOP_PATH,
//         APPEARANCE_INTERFACE,
//     )
//     .await
//     .expect("Should be able to create proxy");
//
//     let res = conn
//         .call_method(
//             Some(DESKTOP_DESTINATION),
//             DESKTOP_PATH,
//             Some("org.freedesktop.portal.Settings"),
//             "Read",
//             &("org.freedesktop.appearance", "color-scheme"),
//         )
//         .await
//         .unwrap();
//
//     let match_rule_res: Result<_, zbus::Error> = try {
//         MatchRule::builder()
//             .interface("org.freedesktop.portal.Settings")?
//             .member("SettingChanged")?
//             .path("/org/freedesktop/portal/desktop")?
//             .arg(0, "org.freedesktop.appearance")?
//             .arg(1, "color-scheme")?
//             .build()
//     };
//     let match_rule = match_rule_res.unwrap();
//
//     let mr = MessageStream::for_match_rule(match_rule, &conn, Some(5))
//         .await
//         .unwrap();
//
//     mr.for_each(|v| async {
//         let msg = v.unwrap();
//         println!("{:?}", msg);
//     })
//     .await;
//
//     println!("v): {:?}", res);
//
//     let mut property_stream: PropertyStream<u32> =
//         proxy.receive_property_changed("color-scheme").await;
//
//     let property_listener = tokio::spawn(async move {
//         println!("Property stream");
//         while let Some(v) = property_stream.next().await {
//             println!("{}", v.get().await.unwrap());
//         }
//     });
//
//     let mut event_stream = proxy.receive_all_signals().await.unwrap();
//     let event_listener = tokio::spawn(async move {
//         println!("Event stream");
//         while let Some(e) = event_stream.next().await {
//             println!("{:?}", e);
//         }
//     });
//
//     property_listener.await.unwrap();
//     event_listener.await.unwrap();
// }
