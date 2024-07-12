#![feature(async_closure, try_blocks, async_fn_traits)]

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
async fn main() {
    let args = CliArgs::parse();

    match args.command {
        cli_args::CliCommand::GetColorScheme { default_as } => {
            let preference = FreedesktopColorSchemeReader::get_preference().await;
            let preference = preference.with_maybe_default_as(default_as.map(Into::into));
            println!("{}", preference);
        }
        cli_args::CliCommand::Monitor {
            default_as,
            quiet,
            on_default,
            on_light,
            on_dark,
        } => {
            let callback = async |preference: ColorScheme| {
                let preference = preference.with_maybe_default_as(default_as.map(Into::into));

                if !quiet {
                    println!("{}", preference);
                }

                let run_command = |cmd: &str| {
                    let cmd = cmd.to_string();

                    tokio::spawn(async {
                        let mut child = Command::new("/bin/sh")
                            .arg("-c")
                            .arg(cmd)
                            .spawn()
                            .expect("Failed to spawn command");

                        child.wait().await.expect("Failed to wait for child");
                    });
                };

                match preference {
                    ColorScheme::Default => on_default.iter().for_each(|s| run_command(s.as_str())),
                    ColorScheme::Dark => on_dark.iter().for_each(|s| run_command(s.as_str())),
                    ColorScheme::Light => on_light.iter().for_each(|s| run_command(s.as_str())),
                }
            };

            callback(FreedesktopColorSchemeReader::get_preference().await).await;

            FreedesktopColorSchemeReader::monitor_preference(callback).await;
        }
        cli_args::CliCommand::SetColorScheme { preference } => {
            todo!("Set preference {:?}", preference)
        }
        cli_args::CliCommand::ToggleDarkMode { default_as } => {
            let preference = FreedesktopColorSchemeReader::get_preference().await;
            let toggled = preference.into_dark_light(default_as).toggle();
            todo!("Toggle to {}", toggled);
        }
    }
}
