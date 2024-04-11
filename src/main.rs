mod commands;
extern crate walkdir;
use clap::{Parser, Subcommand};
use commands::copy_file::cpy_file;
use commands::delete_file::del_file;
use commands::move_file::mv_file;
use commands::properties::file_prop;
use commands::rename_file::rn_file;
use commands::create_directory::create_dir;
use std::{
    fs::{self},
    io,
    path::PathBuf,
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}
#[derive(Subcommand, Debug)]
enum Command {
    #[clap(about = "Rename a file")]
    Rename {
        #[clap(value_parser)]
        old_name: PathBuf,
        #[clap(value_parser)]
        new_name: PathBuf,
    },
    #[clap(about = "Create a new file")]
    Create {
        #[clap(value_parser)]
        file_name: PathBuf,
        #[clap(short, long, value_parser)]
        directory: Option<String>,
    },
    #[clap(about = "Delete a file")]
    Delete {
        #[clap(value_parser)]
        file_path: PathBuf,
        #[clap(short, long, value_parser)]
        directory: Option<PathBuf>,
    },
    #[clap(about = "Move a file")]
    Move {
        #[clap(short, long, value_parser)]
        source_directory: PathBuf,
        #[clap(short, long, value_parser)]
        destination_directory: PathBuf,
    },
    #[clap(about = "Copy a file")]
    Copy {
        #[clap(value_parser)]
        file_name: Option<PathBuf>,
        #[clap(short, long, value_parser)]
        source_path: PathBuf,
        #[clap(short, long, value_parser)]
        destination_path: PathBuf,
    },
    #[clap(about = "Get file properties")]
    Properties {
        #[clap(value_parser)]
        file_name: PathBuf,
    },
    #[clap(about = "Create a new directory")]
    CreateDirectory {
        #[clap(value_parser)]
        directory_name: PathBuf,
        #[clap(short, long, value_parser)]
        parent_directory: Option<PathBuf>,
    },
    #[clap(about = "Rename a directory")]
    RenameDirectory {
        #[clap(value_parser)]
        old_dir: String,
        #[clap(value_parser)]
        new_dir: String,
    },
    #[clap(about = "Delete a directory")]
    DeleteDirectory {
        #[clap(value_parser)]
        directory_name: String,
    },
    #[clap(about = "List files in a directory")]
    ListFiles {
        #[clap(short, long, value_parser)]
        directory: Option<String>,
    },
    #[clap(about = "List sub-directories of a directory")]
    ListDirectories {
        #[clap(short, long, value_parser)]
        directory: Option<String>,
    },
}
fn main() -> io::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Rename { old_name, new_name } => {
            rn_file(old_name, new_name)?;
        }
        Command::Create {
            ref file_name,
            ref directory,
        } => {
            let file_path = PathBuf::from(&file_name);
            let directory = directory.clone();
            if directory.is_none() {
                fs::File::create(file_path)?;
            } else if directory.is_some() {
                let dir_path = PathBuf::from(directory.unwrap()).join(file_name);
                fs::File::create(dir_path)?;
            }
            println!("File created successfully.");
        }
        Command::Delete {
            ref file_path,
            ref directory,
        } => {
            if let Err(e) = del_file(file_path.to_path_buf(), directory.clone().unwrap()) {
                match e.kind() {
                    std::io::ErrorKind::Other => {
                        println!("Error: {}", e);
                    }
                    _ => panic!("Unknown Error"),
                }
            } else {
                return Ok(());
            }
        }
        Command::Move {
            ref source_directory,
            ref destination_directory,
        } => {
            if let Err(e) = mv_file(
                source_directory.to_path_buf(),
                destination_directory.to_path_buf(),
            ) {
                match e.kind() {
                    std::io::ErrorKind::Other => {
                        println!("Error: {}", e);
                    }
                    _ => panic!("Unknown Error"),
                }
            } else {
                return Ok(());
            }
        }
        Command::Copy {
            ref source_path,
            ref destination_path,
            ref file_name,
        } => {
            if let Err(e) = cpy_file(
                source_path.to_owned(),
                destination_path.to_owned(),
                file_name,
            ) {
                match e.kind() {
                    std::io::ErrorKind::Other => {
                        println!("Error: {}", e);
                    }
                    _ => {
                        println!("Unknown Error");
                    }
                }
            } else {
                return Ok(());
            }
        }
        Command::Properties { ref file_name } => {
            file_prop(file_name.to_owned())?;
        }
        Command::CreateDirectory { directory_name, parent_directory } => {
            if let Err(e) = create_dir(directory_name, parent_directory) {
                match e.kind() {
                    std::io::ErrorKind::Other => {
                        println!("Error: {}", e);
                    }
                    _ => {
                        println!("Unknown Error");
                    }
                }
            } else {
                return Ok(());
            }
        }
        _ => println!("Not done yet"),
    }

    // dbg!(&cli);
    Ok(())
}
