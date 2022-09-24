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

use std::env;
use std::path::PathBuf;

const PROFILE_LOCATION: &str = ".vscode_profiles";

#[derive(Debug)]
pub struct VSCodeOptions {
    pub executable: PathBuf,
    pub data_dir: PathBuf,
    pub ext_dir: PathBuf,
    pub be_verbose: bool,
}

impl VSCodeOptions {
    /// Create a struct VSCodeOptions from command line options
    pub fn from(cli: &crate::Cli) -> Self {
        // Get executable path. It guranteee to have a value
        let executable = cli.executable.as_deref().unwrap();
        let executable = which::which(executable);
        if executable.is_err() {
            //TODO: Color output
            let e = executable.err().unwrap();
            eprintln!("Error: {}", e);
            std::process::exit(exitcode::DATAERR);
        }
        let executable = executable.unwrap().to_path_buf();

        // Get user HOME directory
        let home_dir = env::var("HOME");
        if home_dir.is_err() {
            //TODO: Color output
            eprintln!("Error: Missing enviroment variable $HOME");
            std::process::exit(exitcode::DATAERR);
        }
        let home_dir = PathBuf::from(home_dir.unwrap());

        // Get data and extension directories
        let (data_dir, ext_dir) = match cli.profile.as_deref() {
            Some(profile) => {
                let base_dir = home_dir.join(PROFILE_LOCATION).join(profile);
                let data_dir = base_dir.clone().join("data");
                let ext_dir = base_dir.clone().join("extensions");
                (data_dir, ext_dir)
            }
            None => {
                let data_dir = home_dir.join(".config/Code");
                let ext_dir = home_dir.join(".vscode/extensions");
                (data_dir, ext_dir)
            }
        };

        // TODO: get from options
        let be_verbose = true;
        VSCodeOptions {
            executable,
            data_dir,
            ext_dir,
            be_verbose,
        }
    }

    // Get the command to run with basic options
    pub fn get_command(&self) -> std::process::Command {
        let mut command = std::process::Command::new(&self.executable);
        command.arg(format!(
            "--user-data-dir={}",
            self.data_dir.to_string_lossy()
        ));
        command.arg(format!(
            "--extensions-dir={}",
            self.ext_dir.to_string_lossy()
        ));
        command
    }
}

/// Check if profile directories exists (duh)
pub fn check_profile_dirs_exist(vs_options: &VSCodeOptions) {
    // Check if profile directories exists
    if !(vs_options.data_dir.as_path().is_dir() && vs_options.ext_dir.as_path().is_dir()) {
        eprintln!("Error: Profile directories is missing. Did you create the profile?");
        std::process::exit(exitcode::CONFIG);
    }
}

/// Debug runtime arguments
pub fn debug_run_args(vs_options: &VSCodeOptions, command: &std::process::Command) {
    if vs_options.be_verbose {
        print!("Running: {}", vs_options.executable.to_string_lossy());
        for arg in command.get_args() {
            print!(" {}", arg.to_string_lossy());
        }
        println!("");
    }
}
