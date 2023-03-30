use clap::{arg, Command};
use infer;
use std::path::PathBuf;
use std::{ffi::OsString, path::Path};

fn filter_paths(paths: Vec<&PathBuf>) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for file_path in paths {
        let file = Path::new(file_path);

        if file.is_dir() {
            println!(
                "Expected a file for {:?} but received a directory instead.",
                file_path.display()
            );
            continue;
        } else if !file.is_file() {
            // If is not a directory nor a file, then raise this generic message
            println!("{:?} is not a file.", file_path.display());
            continue;
        }
        files.push(file_path.clone());
    }

    return files;
}

/// Returns the file type of the buffer
fn get_file_type(file: &PathBuf) -> String {
    let kind = infer::get_from_path(file).expect("file read error");

    return match kind {
        Some(kind) => kind.mime_type().to_string(),
        None => "Unknown".to_string(),
    };
}

fn cli() -> Command {
    Command::new("infertype")
        .about("Get the type of a file")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("check")
                .about("File(s) path to check")
                .arg_required_else_help(true)
                .arg(
                    arg!(<PATH> ... "File path to infer type")
                        .value_parser(clap::value_parser!(PathBuf)),
                ),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("check", sub_matches)) => {
            let paths = sub_matches
                .get_many::<PathBuf>("PATH")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

            // Filter existing files from the paths
            let files = filter_paths(paths);

            if files.is_empty() {
                return;
            }

            for file in files {
                let filename = file.file_name().unwrap();
                println!("* {:?} {}", filename, get_file_type(&file));
            }
        }

        Some((ext, sub_matches)) => {
            let args = sub_matches
                .get_many::<OsString>("")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("No command found for {ext:?} with {args:?}");
        }

        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
    }
}
