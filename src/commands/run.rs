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
