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

use std::path::PathBuf;

use super::VSCodeOptions;
use clap::Args;
use serde_json::Value;

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

    let exts = get_extension_data(&vs_options.ext_dir);
    if cli.table {
        display_as_table(exts, cli);
    } else {
        display_as_list(exts, cli);
    }
}

// Display extensions as list
fn display_as_list(exts: Vec<ExtensionData>, cli: &ListCLI) {
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

            if e.categories.len() != 0 {
                println!("# Type: {}", e.categories.join(", "));
            }
        }
        println!("{}", e.get_name(cli.versions));
        if cli.commends {
            println!("");
        }
    }
}

fn display_as_table(exts: Vec<ExtensionData>, cli: &ListCLI) {
    use colored::*;
    use terminal_size::terminal_size;

    if exts.is_empty() {
        println!("There is no installed extensions on profile");
        return;
    }

    let (t_wdth, _) = terminal_size().unwrap();
    let names: Vec<String> = exts.iter().map(|e| e.get_name(false)).collect();
    let name_len = names.iter().max_by_key(|e| e.len()).unwrap().len();
    let desc_len: usize = usize::from(t_wdth.0) - name_len - 2;
    println!(
        "{:a$}{:c$}",
        "Name".bold().blue(),
        "Description".bold().blue(),
        a = name_len + 1,
        c = desc_len
    );

    for e in exts.iter() {
        let disp_name = e.display_name.clone().unwrap_or("".to_string());
        let descr = e.description.clone().unwrap_or("".to_string());
        let desc = if descr == "%ext.description%" {
            "".to_string()
        } else {
            truncate_ellipse(&descr, desc_len)
        };

        println!(
            "{:a$}{:c$}",
            e.get_name(false).bold(),
            disp_name.green(),
            a = name_len + 1,
            c = desc_len
        );
        if desc.len() != 0 && cli.commends {
            println!("{:a$}{:c$}", "", desc, a = name_len + 1, c = desc_len);
        }

        if e.categories.len() != 0 && cli.commends {
            let mut txt = format!("Type: {}", e.categories.join(", "));
            txt = truncate_ellipse(&txt, desc_len);
            println!("{:a$}{:c$}", "", txt, a = name_len + 1, c = desc_len);
        }

        if cli.commends {
            println!("");
        }
    }
}

// Parse JSON file and return list of extensions as ExtensionOptions struct
fn get_extension_data(extension_dir: &PathBuf) -> Vec<ExtensionData> {
    let mut exts = Vec::new();
    let read_dir = std::fs::read_dir(extension_dir).unwrap();
    for entry in read_dir {
        let entry = entry.unwrap();
        let path = entry.path().join("package.json");
        if path.is_file() {
            let json_text = std::fs::read_to_string(&path).unwrap();
            let json = serde_json::from_str::<Value>(&json_text).unwrap();

            let cats = json["categories"].as_array().unwrap();
            let cats: Vec<String> = cats.iter().map(|v| v.as_str().unwrap().into()).collect();

            exts.push(ExtensionData {
                name: json["name"].as_str().unwrap().into(),
                version: json["version"].as_str().unwrap().into(),
                publisher: json["publisher"].as_str().unwrap().into(),
                display_name: json["displayName"].as_str().map(|i| i.into()),
                description: json["description"].as_str().map(|i| i.into()),
                categories: cats,
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
    categories: Vec<String>,
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

fn truncate_ellipse(text: &str, len: usize) -> String {
    use unicode_segmentation::UnicodeSegmentation;

    if text.graphemes(true).count() <= len {
        return String::from(text);
    } else if len == 0 {
        return String::from("");
    }

    text.graphemes(true)
        .take(len)
        .chain("…".graphemes(true))
        .collect()
}
