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
