use anyhow::{ensure, Context, Result};
use tokio::process::Command;

use crate::color_scheme::ColorScheme;

pub trait ColorSchemeWriter {
    async fn set_color_scheme(color_scheme: ColorScheme) -> Result<()>;
}

pub struct GSettings {}

impl ColorSchemeWriter for GSettings {
    async fn set_color_scheme(color_scheme: ColorScheme) -> Result<()> {
        let mut child = Command::new("gsettings")
            .args(["set", "org.gnome.desktop.interface", "color-scheme"])
            .arg(match color_scheme {
                ColorScheme::Default => "default",
                ColorScheme::Dark => "prefer-dark",
                ColorScheme::Light => "prefer-light",
            })
            .spawn()
            .with_context(|| "Failed to spawn gsettings. Is it installed?")?;

        let exit_status = child
            .wait()
            .await
            .with_context(|| "Failed to wait for gsettings.")?;

        ensure!(
            exit_status.success(),
            "gsettings exited with a bad exit code"
        );

        Ok(())
    }
}
