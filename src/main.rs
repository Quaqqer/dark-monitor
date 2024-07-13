#![feature(async_closure, try_blocks, async_fn_traits, let_chains)]

use anyhow::{bail, Context, Result};
use cs_writer::{ColorSchemeWriter, GSettings};
use std::io::Write;
use tokio::process::Command;

use clap::Parser;
use cli_args::CliArgs;
use color_scheme::ColorScheme;
use cs_reader::{ColorSchemeReader, FreedesktopColorSchemeReader};

pub mod color_scheme;

mod cli_args;
mod cs_reader;
mod cs_writer;

#[tokio::main]
async fn main() -> Result<()> {
    let args = CliArgs::parse();

    match args.command {
        cli_args::CliCommand::GetColorScheme { default_as } => {
            let preference = FreedesktopColorSchemeReader::get_preference().await?;
            let preference = preference.with_maybe_default_as(default_as.map(Into::into));
            println!("{}", preference);
            Ok(())
        }
        cli_args::CliCommand::Monitor {
            default_as,
            on_default,
            on_light,
            on_dark,
        } => {
            let default_as = default_as.map(Into::into);
            if let Some(default_as) = default_as
                && !on_default.is_empty()
            {
                bail!("Interprets default as '{default_as}' but commands are specified to run when changing to the default color-scheme");
            }

            let callback = async |preference: ColorScheme| {
                let preference = preference.with_maybe_default_as(default_as);

                let run_command = |cmd: &str| {
                    let cmd = cmd.to_string();

                    tokio::spawn(async move {
                        let child = (Command::new("/bin/sh").arg("-c").arg(&cmd))
                            .spawn()
                            .with_context(|| "Failed to spawn command.");

                        let exit_status = match child {
                            Ok(mut child) => child
                                .wait()
                                .await
                                .with_context(|| "Failed to wait for command to exit."),
                            Err(e) => Err(e),
                        };

                        let res = match exit_status {
                            Ok(status) if status.success() => Ok(()),
                            Ok(status) => status
                                .code()
                                .with_context(|| "Failed to get status code")
                                .and_then(|code| {
                                    bail!("Command `{cmd}` failed with exit code: {code}")
                                }),
                            Err(e) => Err(e),
                        };

                        match res {
                            Ok(_) => {}
                            Err(err) => eprintln!("{err}"),
                        }
                    });
                };

                match preference {
                    ColorScheme::Default => on_default.iter().for_each(|s| run_command(s.as_str())),
                    ColorScheme::Dark => on_dark.iter().for_each(|s| run_command(s.as_str())),
                    ColorScheme::Light => on_light.iter().for_each(|s| run_command(s.as_str())),
                }
            };

            callback(FreedesktopColorSchemeReader::get_preference().await?).await;

            FreedesktopColorSchemeReader::monitor_preference(callback).await
        }

        cli_args::CliCommand::SetColorScheme { color_scheme } => {
            GSettings::set_color_scheme(color_scheme.into()).await
        }

        cli_args::CliCommand::ToggleDarkMode { default_as } => {
            let preference = FreedesktopColorSchemeReader::get_preference().await?;
            let toggled = preference.into_dark_light(default_as.into()).toggle();
            GSettings::set_color_scheme(toggled.into()).await
        }

        cli_args::CliCommand::Listen { default_as } => {
            let default_as = default_as.map(Into::into);
            let callback = async |preference: ColorScheme| {
                let mut stdout = std::io::stdout().lock();
                write!(stdout, "{}\n", preference.with_maybe_default_as(default_as)).unwrap();
                stdout.flush().unwrap();
            };

            callback(FreedesktopColorSchemeReader::get_preference().await?).await;

            FreedesktopColorSchemeReader::monitor_preference(callback).await
        }
    }
}
