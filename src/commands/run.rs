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

use super::VSCodeOptions;
use clap::Args;
use execute::Execute;
// use execute::Execute;

#[derive(Args)]
/// Run VS Code with selected profile if any
pub struct RunCLI {
    /// The project directory
    #[arg(value_name = "Directory")]
    work_dir: Option<String>,

    /// Be verbose
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Extra agrs to pass into VS Code
    #[arg(last = true)]
    extra_args: Vec<String>,
}

/// The run subcommand
pub fn run(cli: &RunCLI, vs_options: &VSCodeOptions) {
    super::check_profile_dirs_exist(vs_options);

    let mut command = vs_options.get_command();

    // Extra args
    for arg in cli.extra_args.iter() {
        command.arg(arg);
    }

    if let Some(work_dir) = cli.work_dir.as_deref() {
        command.arg(work_dir);
    }

    super::debug_run_args(vs_options, &command);

    if let Some(exit_code) = command.execute().unwrap() {
        std::process::exit(exit_code);
    }
}
