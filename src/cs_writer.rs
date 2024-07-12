use tokio::process::Command;

use crate::color_scheme::ColorScheme;

pub trait ColorSchemeWriter {
    async fn set_color_scheme(color_scheme: ColorScheme);
}

pub struct GSettings {}

impl ColorSchemeWriter for GSettings {
    async fn set_color_scheme(color_scheme: ColorScheme) {
        let mut cmd = Command::new("gsettings")
            .args(["set", "org.gnome.desktop.interface", "color-scheme"])
            .arg(match color_scheme {
                ColorScheme::Default => "default",
                ColorScheme::Dark => "prefer-dark",
                ColorScheme::Light => "prefer-light",
            })
            .spawn()
            .expect("Failed to spawn command");

        let _status = cmd.wait().await;
    }
}
