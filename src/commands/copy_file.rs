use simple_error::SimpleError;
use std::boxed::Box;
use std::io::{copy, Error, ErrorKind};
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;
use std::{fs, fs::File};
use walkdir::{DirEntry, WalkDir};
pub fn cpy_file(
    src_dir: PathBuf,
    mut target_dir: PathBuf,
    src_file_name: &Option<PathBuf>,
) -> Result<(), Error> {

    if src_dir.is_file() {
      println!("in is file");
        //  Open the file to be copied
        let mut src_file = File::open(&src_dir)?;
        // Make buffer to store contents
        let mut buf = Vec::new();
        // Read data into buffer
        src_file.read_to_end(&mut buf)?;
        let f_name = if src_file_name.is_none() {
          src_dir.file_name().unwrap().to_str().unwrap().to_string()
        } else {
          src_file_name.clone().unwrap().to_str().unwrap().to_string()
        };
        
        let mut target_path = target_dir.parent().unwrap().to_owned();
        target_path.push(f_name);
        // Write contents of buffer to target path
        fs::write(target_path, buf)?;
    }
        if src_dir.is_dir() && src_file_name.is_none() {
          let err = Error::new(ErrorKind::Other, "`file_name` is required when `source-path` is a directory, in order to search the directory for the file.");
          eprintln!("Unable to complete copy operation: {err}"); 
          std::process::exit(1);
        } else {
          ()
        }
        println!("Here we are");
        let mut exists_in_dir: bool;
        let src_name = src_file_name.clone().unwrap().to_str().unwrap().to_string();
        println!("src_name: {} ", src_name);

        let mut walk_dir = WalkDir::new(src_dir).into_iter();

        // while walk_dir.next().is_some() {
        //   let entry_file_name = entry?.file_name().to_str().to_owned().unwrap().to_string();
        //   println!("entry file name: {}", entry_file_name);
          
        // }
        // for entry in WalkDir::new(src_dir)
        //   .into_iter()
        //   .filter_entry(|entry| { in_dir(entry,src_file_name.clone().unwrap())}) {
        //     println!("entry file name: {} ", entry_file_name);
        //     println!("src name: {} ", src_name);
        //     if entry_file_name != src_name {
        //       println!("Match between {} and {} found.", entry_file_name, src_name);
        //     }
            // println!(
            //     "entry name: {:?}, file name: {:?}",
            //     if let Some(entry).unwrap().file_name(),
            //     src_file_name
            // );

            Ok(())
        }

        // for entry in source_path.read_dir().expect("Unable to read directory") {
        //     if let Ok(entry) = entry {
        //         let file_metadata = entry.metadata().unwrap();
        //         if file_metadata.is_file() {
        //             f
        //         }
        //     }
        // }
        // Check source path for file name
        // If file name exists there, grab it
        // Check if file name exists in destination path
        // If so, throw file exists error
        // If not, copy the file
 


fn in_dir(entry: &DirEntry, file_name: PathBuf) -> bool {
    let entry_file_name = entry
        .file_name()
        .to_ascii_lowercase()
        .to_str()
        .unwrap()
        .to_owned();

    PathBuf::from(entry_file_name) == file_name
}
