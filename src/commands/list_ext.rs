use super::VSCodeOptions;
use clap::Args;
use execute::Execute;

#[derive(Args)]
/// Lists installed extensions for the selected profile.
pub struct ListCLI {
    /// Show versions of installed extensions
    #[arg(short = 'a', long, value_parser, value_name = "Profile")]
    versions: bool,
}

/// The list subcommand
pub fn list(cli: &ListCLI, vs_options: &VSCodeOptions) {
    super::check_profile_dirs_exist(vs_options);

    let mut command = vs_options.get_command();
    command.arg("--list-extensions");

    let versions = cli.versions;
    if versions {
        command.arg("--show-versions");
    }

    super::debug_run_args(vs_options, &command);

    // Run the command and show the output
    let output = command.execute_output().unwrap();
    let status = output.status.code();
    std::process::exit(status.unwrap());
}
