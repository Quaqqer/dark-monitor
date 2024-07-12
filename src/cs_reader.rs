use std::ops::AsyncFn;

use anyhow::{bail, Context, Result};
use futures_util::StreamExt;
use zbus::{MatchRule, MessageStream};

use crate::color_scheme::ColorScheme;

pub trait ColorSchemeReader {
    /// Get the color scheme preference from the desktop
    async fn get_preference() -> Result<ColorScheme>;

    /// Monitor the color scheme preference from the desktop
    ///
    /// * `f`: The callback for when the preference changes
    async fn monitor_preference(f: impl AsyncFn(ColorScheme)) -> Result<()>;
}

pub struct FreedesktopColorSchemeReader {}

impl ColorSchemeReader for FreedesktopColorSchemeReader {
    async fn get_preference() -> Result<ColorScheme> {
        let connection = zbus::Connection::session()
            .await
            .with_context(|| "Failed to connect to user D-Bus session, is D-Bus running?")?;

        let msg = connection
            .call_method(
                Some("org.freedesktop.portal.Desktop"),
                "/org/freedesktop/portal/desktop",
                Some("org.freedesktop.portal.Settings"),
                "Read",
                &("org.freedesktop.appearance", "color-scheme"),
            )
            .await
            .with_context(|| {
                "Failed to get color-scheme preference, are you missing a color-scheme provider?
Some example providers are:
- xdg-desktop-portal-gnome
- xdg-desktop-portal-kde
- xdg-desktop-portal-gtk
- darkman"
            })?;

        let deserialized = msg
            .body()
            .deserialize::<zbus::zvariant::Value>()
            .with_context(|| "Failed to deserialize dbus value")?
            .downcast::<u32>()
            .with_context(|| "Failed to convert value to number.")?;

        Ok(match deserialized {
            0 => ColorScheme::Default,
            1 => ColorScheme::Dark,
            2 => ColorScheme::Light,
            unexpected => bail!("Unexpected color-scheme value: {unexpected}"),
        })
    }

    async fn monitor_preference(f: impl AsyncFn(ColorScheme)) -> Result<()> {
        let connection = zbus::Connection::session()
            .await
            .with_context(|| "Failed to connect to user D-Bus session, is D-Bus running?")?;

        let match_rule = MatchRule::builder()
            .interface("org.freedesktop.portal.Settings")
            .unwrap()
            .member("SettingChanged")
            .unwrap()
            .path("/org/freedesktop/portal/desktop")
            .unwrap()
            .arg(0, "org.freedesktop.appearance")
            .unwrap()
            .arg(1, "color-scheme")?
            .build();

        let mut ms = MessageStream::for_match_rule(match_rule, &connection, Some(5))
            .await
            .with_context(|| "Failed to subscribe to D-Bus.")?;

        while let Some(msg) = ms.next().await {
            let msg = msg.with_context(|| "Failed to get message from D-Bus.")?;

            let (_interface, _property, zbus::zvariant::Value::U32(preference_u32)) = msg
                .body()
                .deserialize::<(&str, &str, zbus::zvariant::Value)>()
                .with_context(|| "Failed to deserialize D-Bus message.")?
            else {
                bail!("Unexpected message value.")
            };

            let pref = ColorScheme::try_from(preference_u32)
                .ok()
                .with_context(|| "Failed to convert color scheme preference")?;

            f(pref).await;
        }

        Ok(())
    }
}
