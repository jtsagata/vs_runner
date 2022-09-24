/*
 *   Copyright (c) 2022
 *   All rights reserved.
 */
mod commands;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

const DEFAULT_EXECUTABLE: &str = "code";

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// The VS Code Profile
    #[arg(short, long, value_name = "Profile", global = true)]
    profile: Option<String>,

    /// Alternative excutable
    #[arg(
        short,
        long,
        global=true,
        value_name = "Executable",
        default_value = DEFAULT_EXECUTABLE,
        value_parser=executable_exists
    )]
    executable: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Run(commands::RunCLI),
    #[command(name = "extensions")]
    List(commands::ListCLI),
}

fn main() {
    better_panic::install();
    let cli = Cli::parse();

    let vs_code_options = commands::VSCodeOptions::from(&cli);
    match &cli.command {
        Some(Commands::Run(cli)) => {
            commands::run(cli, &vs_code_options);
        }
        Some(Commands::List(cli)) => {
            commands::list(cli, &vs_code_options);
        }
        None => {}
    }

    std::process::exit(exitcode::OK);
}

/// Test if an executable exists
fn executable_exists(code: &str) -> Result<PathBuf, which::Error> {
    which::which(code)
}
