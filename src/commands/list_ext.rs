/*
 *   Copyright (c) 2022
 *   All rights reserved.
 */
use super::VSCodeOptions;
use clap::Args;
use serde_json::Value;
// use std::{ops::Deref, process::Stdio};

#[derive(Args)]
/// Lists installed extensions for the selected profile.
pub struct ListCLI {
    /// Show versions of installed extensions
    #[arg(short, long)]
    versions: bool,

    /// Show extra information as commends
    #[arg(short, long)]
    commends: bool,

    /// Display as table
    #[arg(short, long)]
    table: bool,

    /// Show icons on termanal
    #[arg(short, long)]
    icons: bool,
}

/// The list subcommand
pub fn list(cli: &ListCLI, vs_options: &VSCodeOptions) {
    super::check_profile_dirs_exist(vs_options);

    let exts = get_extension_data(vs_options);

    for e in exts.iter() {
        if cli.commends {
            if e.display_name.is_some() {
                let s = e.display_name.as_ref().unwrap();
                if s.len() != 0 && s != "%ext.displayName%" {
                    println!("# {} ({})", s, e.version);
                }
            }
            if e.description.is_some() {
                let s = e.description.as_ref().unwrap();
                if s.len() != 0 && s != "%ext.description%" {
                    println!("# {}", e.description.as_ref().unwrap());
                }
            }
        }
        println!("{}", e.get_name(cli.versions));
        if cli.commends {
            println!("");
        }
    }
}

// Parse JSON file and return list of extensions as ExtensionOptions struct
fn get_extension_data(vs_options: &VSCodeOptions) -> Vec<ExtensionData> {
    let mut exts = Vec::new();
    let read_dir = std::fs::read_dir(vs_options.ext_dir.as_path()).unwrap();
    for entry in read_dir {
        let entry = entry.unwrap();
        let path = entry.path().join("package.json");
        if path.is_file() {
            let json_text = std::fs::read_to_string(&path).unwrap();
            let json = serde_json::from_str::<Value>(&json_text).unwrap();

            exts.push(ExtensionData {
                name: json["name"].as_str().unwrap().into(),
                version: json["version"].as_str().unwrap().into(),
                publisher: json["publisher"].as_str().unwrap().into(),
                display_name: json["displayName"].as_str().map(|i| i.into()),
                description: json["description"].as_str().map(|i| i.into()),
                icon: json["icon"].as_str().map(|i| i.into()),
            });
        }
    }
    exts.sort();
    exts
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct ExtensionData {
    publisher: String,
    name: String,
    version: String,
    display_name: Option<String>,
    description: Option<String>,
    icon: Option<String>,
}

// get extansion vs_code name with or without version
impl ExtensionData {
    fn get_name(&self, show_version: bool) -> String {
        if show_version {
            format!("{}.{}@{}", self.publisher, self.name, self.version)
        } else {
            format!("{}.{}", self.publisher, self.name)
        }
    }
}

impl PartialOrd for ExtensionData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Sort case insensitive as VS Code
        self.get_name(true)
            .to_uppercase()
            .partial_cmp(&other.get_name(true).to_uppercase())
    }
}
