use std::ops::AsyncFn;

use futures_util::StreamExt;
use zbus::{MatchRule, MessageStream};

use crate::color_scheme::ColorSchemePreference;

type Result<T> = std::result::Result<T, &'static str>;

#[derive(Clone)]
pub struct AppearanceConnection {
    connection: zbus::Connection,
}

impl AppearanceConnection {
    pub async fn connect() -> Self {
        let connection = zbus::Connection::session()
            .await
            .expect("Failed to connect to D-Bus sessiosion");

        Self { connection }
    }

    pub async fn get_preference(&self) -> Result<ColorSchemePreference> {
        let msg = self
            .connection
            .call_method(
                Some("org.freedesktop.portal.Desktop"),
                "/org/freedesktop/portal/desktop",
                Some("org.freedesktop.portal.Settings"),
                "Read",
                &("org.freedesktop.appearance", "color-scheme"),
            )
            .await
            .expect("Failed to get preference");

        let preference = ColorSchemePreference::try_from(
            msg.body()
                .deserialize::<zbus::zvariant::Value>()
                .expect("Could not deserialize color-scheme preference")
                .downcast::<u32>()
                .unwrap(),
        )
        .unwrap();

        Ok(preference)
    }

    pub async fn listen(&self, f: impl AsyncFn(ColorSchemePreference)) {
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

        let mr = MessageStream::for_match_rule(match_rule, &self.connection, Some(5))
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

            let pref = ColorSchemePreference::try_from(preference_u32).unwrap();

            f(pref).await;
        })
        .await;
    }
}
