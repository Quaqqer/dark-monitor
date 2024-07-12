use clap::CommandFactory;
use clap_complete::{
    generate_to,
    shells::{Bash, Fish, Zsh},
};
use std::env;
use std::io::Error;

include!("./src/cli_args.rs");

fn main() -> Result<(), Error> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let cmd = CliArgs::command();

    let bash_path = generate_to(Bash, &mut cmd.clone(), "dark-monitor", outdir.clone())?;
    println!("cargo::warning=bash completion generated: {bash_path:?}");

    let zsh_path = generate_to(Zsh, &mut cmd.clone(), "dark-monitor", outdir.clone())?;
    println!("cargo::warning=zsh completion generated: {zsh_path:?}");

    let fish_path = generate_to(Fish, &mut cmd.clone(), "dark-monitor", outdir.clone())?;
    println!("cargo::warning=fish completion generated: {fish_path:?}");

    Ok(())
}
