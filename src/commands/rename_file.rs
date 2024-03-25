use std::{fs, path::{PathBuf, Path}, io::{Error, ErrorKind}, env::current_dir};

pub fn rn_file(old_name: PathBuf, new_name: PathBuf) -> Result<(), std::io::Error> {
  let old_name = &mut old_name.clone();
  let file_path = old_name.file_name().unwrap();
  let old_file_name = Path::new(file_path);
  fs::rename(old_name, new_name)?;
  println!("File renamed successfully."); 
  Ok(())
}