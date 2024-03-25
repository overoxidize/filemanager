mod commands;
extern crate walkdir;
use chrono::DateTime;
use clap::{Parser, Subcommand};
use commands::copy_file::cpy_file;
use commands::delete_file::del_file;
use commands::move_file::mv_file;
use std::{
    ffi::OsStr,
    fs::{self},
    io,
    path::{Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};

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
        file_name: String,
        #[clap(short, long, value_parser)]
        directory: Option<String>,
    },
    #[clap(about = "Create a new directory")]
    CreateDirectory {
        #[clap(value_parser)]
        directory_name: String,
        #[clap(short, long, value_parser)]
        parent_directory: Option<String>,
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
        // Possible errors
        // Path is absolute,
        Command::Rename {
            ref old_name,
            ref new_name,
        } => {
            let old_name = &mut old_name.clone();
            let file_path = old_name.file_name().unwrap();
            let old_file_name = Path::new(file_path);
            fs::rename(old_file_name, new_name)?;
            println!("File renamed successfully.");
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
            cpy_file(
                source_path.to_owned(),
                destination_path.to_owned(),
                file_name,
            )
            .ok();
 
        }
        Command::Properties { ref file_name, ref directory} => {
            
        }
        _ => println!("Not done yet"),
    }

    // dbg!(&cli);
    Ok(())
}

fn in_dir(entry: &DirEntry, file_name: PathBuf) -> bool {
    let entry_file_name = entry
        .file_name()
        .to_ascii_lowercase()
        .to_str()
        .unwrap()
        .to_owned();

    PathBuf::from(entry_file_name) == file_name
}

// fn create_file(path: &str) {
//     let path = env::current_dir().unwrap().join(path);

//     match fs::File::create(path) {
//         Ok(_) => println!("File created successfully"),
//         Err(error) => println!("Error creating file: {}", error),
//     }
// }

// fn delete_file(path: &str) {
//     let path = env::current_dir().unwrap().join(path);

//     match fs::remove_file(path) {
//         Ok(_) => println!("File deleted successfully"),
//         Err(error) => println!("Error deleting file: {}", error),
//     }
// }

// fn move_file(path: &str, destination: &str) {
//     let path = env::current_dir().unwrap().join(path);
//     let destination = env::current_dir().unwrap().join(destination);

//     match fs::rename(path, destination) {
//         Ok(_) => println!("File moved successfully"),
//         Err(error) => println!("Error moving file: {}", error),
//     }
// }

// fn copy_file(path: &str, destination: &str) {
//     let path = env::current_dir().unwrap().join(path);
//     let destination = env::current_dir().unwrap().join(destination);

//     match fs::copy(path, destination) {
//         Ok(_) => println!("File copied successfully"),
//         Err(error) => println!("Error copying file: {}", error),
//     }
// }

// // fn check_file_properties(path: &str) {
// //     let path = env::current_dir().unwrap().join(path);

// //     match fs::metadata(path) {
// //         Ok(metadata) => {
// //             println!("Size: {} bytes", metadata.len());
// //             println!("Extension: {}", metadata.file_name().unwrap().extension().unwrap());
// //             println!("Creation time: {}", metadata.created().unwrap());
// //         }
// //         Err(error) => println!("Error getting file
