use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::{fs, fs::File};
use walkdir::WalkDir;

pub fn cpy_file(
    src_dir: PathBuf,
    mut target_dir: PathBuf,
    src_file_name: &Option<PathBuf>,
) -> Result<(), Error> {
    let mut buf = Vec::new();
    if src_dir.is_file() {
        //  Open the file to be copied
        let mut src_file = File::open(&src_dir)?;
        // Make buffer to store contents
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
        fs::write(target_path, &buf)?;
    }
    if src_dir.is_dir() && src_file_name.is_none() {
        let err = Error::new(ErrorKind::Other, "`file_name` is required when `source-path` is a directory, in order to search the directory for the file.");
        eprintln!("Unable to complete copy operation: {err}");
        std::process::exit(1);
    }

    let src_name = src_file_name.clone().unwrap();

    let mut walk_dir = WalkDir::new(&src_dir).into_iter();

    let mut exists_in_dir = false;

    let mut full_path = PathBuf::new();
    buf.flush()?;

    while !exists_in_dir {
        let dir_entry = walk_dir.next().unwrap()?.path().to_owned();

        if dir_entry == src_name {
            println!("File match {:?} : {:?}", dir_entry, src_name);
            full_path = dir_entry;
            exists_in_dir = true;
        }
    }

    let mut cpy_file = File::open(&full_path)?;
    let file_name = full_path.file_name().unwrap();
    cpy_file.read_to_end(&mut buf)?;
    target_dir.push(file_name);

    fs::write(target_dir, buf)?;
    Ok(())
}
