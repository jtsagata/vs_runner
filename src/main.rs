mod commands;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
        default_value = "code",
        value_parser=executable_exists
    )]
    executable: Option<PathBuf>,

    /// Be verbose
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    // /// Run VS Code with selected profile if any
    // Run {
    //     /// Extra agrs to pass into VS Code
    //     #[clap(last = true)]
    //     extra_args: Vec<String>,
    // },
    List(commands::ListCLI),
}

fn main() {
    better_panic::install();
    let cli = Cli::parse();

    let vs_code_options = commands::VSCodeOptions::from(&cli);
    match &cli.command {
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
