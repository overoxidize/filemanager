use walkdir::DirEntry;
use std::path::{Path, PathBuf};

pub fn in_dir(entry: PathBuf, file_name: PathBuf) -> bool {
  // let entry_file_name = entry
  //     .file_name()
  //     .to_ascii_lowercase()
  //     .to_str()
  //     .unwrap()
  //     .to_owned()
  //     .replace('\"', "");

  PathBuf::from(entry) == file_name
}

// pub fn search_dir(target: PathBuf) -> bool {


// }