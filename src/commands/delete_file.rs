use std::fs;
use std::path::PathBuf;
pub fn del_file(file_path: PathBuf, directory: PathBuf) -> std::io::Result<()> {
    if !file_path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File Path not found.",
        ));
    }
    if !directory.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Directory not found.",
        ));
    }
    match Some(directory) {
        Some(directory) => {
            let target_file = match Some(&file_path) {
                Some(file_path) => file_path,
                None => {
                    panic!("There's no file path");
                }
            };

            let cur_dir = std::env::current_dir()?;
            if cur_dir == directory {
                let _ = fs::remove_file(target_file);
            }

            let _ = fs::remove_file(file_path);
        }
        None => {
            let _ = fs::remove_file(file_path);
        }
    };

    println!("File deleted successfully.");
    Ok(())
}
