use clap::{Parser, Subcommand, ValueEnum};

use crate::color_scheme::{ColorScheme, DarkLight};

#[derive(Parser, Debug)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand, Debug)]
pub enum CliCommand {
    GetColorScheme {
        #[arg(long, value_enum)]
        default_as: Option<DarkLight>,
    },
    Monitor {
        #[arg(long, value_enum)]
        default_as: Option<DarkLight>,

        #[arg(long, help = "Command to run on default preference")]
        on_default: Vec<String>,

        #[arg(long, help = "Command to run on light preference")]
        on_light: Vec<String>,

        #[arg(long, help = "Command to run on dark preference")]
        on_dark: Vec<String>,
    },
    SetColorScheme {
        #[arg(value_enum)]
        preference: ColorScheme,
    },
    ToggleDarkMode {
        #[arg(long, value_enum, default_value_t=DarkLight::Light)]
        default_as: DarkLight,
    },
    Listen {
        #[arg(long, value_enum)]
        default_as: Option<DarkLight>,
    },
}
