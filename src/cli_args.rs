use clap::{Parser, Subcommand, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
pub enum CliDarkLight {
    Dark,
    Light,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum CliColorScheme {
    Default,
    Dark,
    Light,
}

#[derive(Parser, Debug)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand, Debug)]
pub enum CliCommand {
    /// Get the color scheme
    GetColorScheme {
        #[arg(long, value_enum)]
        /// Interpret the default color-scheme as dark or light instead
        default_as: Option<CliDarkLight>,
    },
    /// Monitor and run commands on color-scheme changes. Runs the command for the current color-scheme on startup.
    Monitor {
        #[arg(long, value_enum)]
        /// Interpret the default color-scheme as dark or light instead
        default_as: Option<CliDarkLight>,

        #[arg(long)]
        /// Command to run when switching to default color-scheme
        on_default: Vec<String>,

        #[arg(long)]
        /// Command to run when switching to dark color-scheme
        on_dark: Vec<String>,

        #[arg(long)]
        /// Command to run when switching to light color-scheme
        on_light: Vec<String>,
    },
    /// Set the color-scheme
    SetColorScheme {
        #[arg(value_enum)]
        color_scheme: CliColorScheme,
    },
    /// Toggle dark mode
    ToggleDarkMode {
        #[arg(long, value_enum, default_value_t=CliDarkLight::Light)]
        /// Interpret the default color-scheme as dark or light when toggling
        default_as: CliDarkLight,
    },
    /// Listen to changes and output to stdout. Outputs the current color-scheme on startup.
    Listen {
        #[arg(long, value_enum)]
        /// Interpret the default color-scheme as dark or light instead
        default_as: Option<CliDarkLight>,
    },
}
