use super::VSCodeOptions;
use clap::Args;
use execute::Execute;

#[derive(Args, Debug)]
/// List VS Code installed extensions
pub struct ListCLI {
    /// Show versions of installed extensions
    #[arg(short = 'a', long, value_parser, value_name = "Profile")]
    versions: bool,
}

/// The list subcommand
pub fn list(cli: &ListCLI, vs_options: &VSCodeOptions) {
    // Check if profile directories exists
    if !(vs_options.data_dir.as_path().is_dir() && vs_options.ext_dir.as_path().is_dir()) {
        eprintln!("Error: Profile directories is missing. Did you create the profile?");
        std::process::exit(exitcode::CONFIG);
    }

    let mut command = vs_options.get_command();
    command.arg("--list-extensions");

    let versions = cli.versions;
    if versions {
        command.arg("--show-versions");
    }

    if vs_options.be_verbose {
        print!("Running: {}", vs_options.executable.to_string_lossy());
        for arg in command.get_args() {
            print!(" {}", arg.to_string_lossy());
        }
        println!("");
    }

    // Run the command and show the output
    let output = command.execute_output().unwrap();
    let status = output.status.code();
    std::process::exit(status.unwrap());
}
