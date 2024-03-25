use std::{fs, path::PathBuf, io::{Error, ErrorKind}, env::current_dir};
use crate::utilities::in_dir;

fn rename(old_name: PathBuf, new_name: String) -> Result<(), std::io::Error> {
  let cur_dir = current_dir()?;

  let target_dir = PathBuf::from(old_name.file_name().unwrap());
  let mut dir_desc = cur_dir.read_dir()?.into_iter();
  

  while let Some(descendant) = dir_desc.next() {
    let desc_dir = PathBuf::from(descendant?
      .file_name()
      .to_str()
      .unwrap()
      .to_string());

      if desc_dir == old_name {
        println!("Target match old_name -> {:?} : desc_dir -> {:?}", old_name, desc_dir);
        break;
      }



  }
  // if old_name.is_dir() {
  //   let err = Error::new(ErrorKind::Other, "`old_name` is a directory when `source-path` is a directory, in order to search the directory for the file.");
  //   eprintln!("Unable to complete copy operation: {err}");
  //   std::process::exit(1);
  // }
  fs::rename(old_name, new_name)?;
  println!("File renamed successfully."); 
  Ok(())
}