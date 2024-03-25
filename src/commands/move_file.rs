use std::fs;
use std::path::PathBuf;
pub fn mv_file(src_file: PathBuf, target_dir: PathBuf) -> std::io::Result<()> {
    if !src_file.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "File Path not found.",
        ));
    }
    if !target_dir.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Directory not found.",
        ));
    }

    match fs::rename(&src_file, &target_dir) {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::Other => {
            fs::copy(&src_file, &target_dir)?;
            fs::remove_file(&src_file)?;
            Ok(())
        }
        Err(e) => Err(e),
    }
}
