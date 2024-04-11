use std::path::PathBuf;
use std::fs::DirBuilder;

pub fn create_dir(dir_name: PathBuf, parent_dir: Option<PathBuf>) -> Result<(), std::io::Error>  {
  let mut builder = DirBuilder::new();
  if parent_dir.is_none() {
    let cwd = std::env::current_dir().unwrap();
    let path = cwd.join(&dir_name);
    builder.recursive(false).create(path).unwrap();

  }

  let target_path = parent_dir.unwrap().join(dir_name);

  builder.recursive(false).create(target_path).unwrap();

  Ok(())
}