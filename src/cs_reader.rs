use std::ops::AsyncFn;

use futures_util::StreamExt;
use zbus::{MatchRule, MessageStream};

use crate::color_scheme::ColorScheme;

pub trait ColorSchemeReader {
    /// Get the color scheme preference from the desktop
    async fn get_preference() -> ColorScheme;

    /// Monitor the color scheme preference from the desktop
    ///
    /// * `f`: The callback for when the preference changes
    async fn monitor_preference(f: impl AsyncFn(ColorScheme));
}

pub struct FreedesktopColorSchemeReader {}

impl ColorSchemeReader for FreedesktopColorSchemeReader {
    async fn get_preference() -> ColorScheme {
        let connection = zbus::Connection::session()
            .await
            .expect("Failed to connect to D-Bus session");

        let msg = connection
            .call_method(
                Some("org.freedesktop.portal.Desktop"),
                "/org/freedesktop/portal/desktop",
                Some("org.freedesktop.portal.Settings"),
                "Read",
                &("org.freedesktop.appearance", "color-scheme"),
            )
            .await
            .expect("Failed to get preference");

        ColorScheme::try_from(
            msg.body()
                .deserialize::<zbus::zvariant::Value>()
                .expect("Could not deserialize color-scheme preference")
                .downcast::<u32>()
                .unwrap(),
        )
        .unwrap()
    }

    async fn monitor_preference(f: impl AsyncFn(ColorScheme)) {
        let connection = zbus::Connection::session()
            .await
            .expect("Failed to connect to D-Bus sessiosion");

        let match_rule_res: std::result::Result<_, zbus::Error> = try {
            MatchRule::builder()
                .interface("org.freedesktop.portal.Settings")?
                .member("SettingChanged")?
                .path("/org/freedesktop/portal/desktop")?
                .arg(0, "org.freedesktop.appearance")?
                .arg(1, "color-scheme")?
                .build()
        };
        let match_rule = match_rule_res.unwrap();

        let mr = MessageStream::for_match_rule(match_rule, &connection, Some(5))
            .await
            .unwrap();

        mr.for_each(|msg| async {
            let msg = msg.expect("Could get message");
            let (_interface, _property, zbus::zvariant::Value::U32(preference_u32)) = msg
                .body()
                .deserialize::<(&str, &str, zbus::zvariant::Value)>()
                .unwrap()
            else {
                panic!("Unexpected return value");
            };

            let pref = ColorScheme::try_from(preference_u32).unwrap();

            f(pref).await;
        })
        .await;
    }
}
