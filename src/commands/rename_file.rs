use std::{fs, path::{PathBuf}};

pub fn rn_file(old_name: PathBuf, new_name: PathBuf) -> Result<(), std::io::Error> {
  fs::rename(old_name, new_name)?;
  println!("File renamed successfully."); 
  Ok(())
}