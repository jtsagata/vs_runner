// Copyright (C) 2022 asfodelus
//
// This file is part of vs_runner.
//
// vs_runner is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// vs_runner is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with vs_runner.  If not, see <http://www.gnu.org/licenses/>.

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
