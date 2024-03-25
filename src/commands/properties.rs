use std::{fs::File, path::PathBuf, fmt, time::SystemTime};
use file_format::{FileFormat, Kind};


#[derive(Debug)]
 
pub struct FileProperty {
  file_name: String,
  is_dir: bool, 
  is_file: bool,
  file_ext: String,
  file_kind: Kind,
  file_format: FileFormat,
  byte_len: u64,

}

impl fmt::Display for FileProperty {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let file_name = &self.file_name;
    println!("The properties of {} are: ", &file_name);
    write!(f, "Is {} a directory: {}", &file_name, self.is_dir)?;
    println!("");
    write!(f, "Is {} a file: {}", &file_name, self.is_file)?;
    println!("");
    write!(f, "The extension of {} is : {}", &file_name, self.file_ext)?;
    println!("");
    write!(f, "The kind of {} is: {:?}", &file_name, self.file_kind)?;
    println!("");
    write!(f, "The format of {} is: {}", &file_name, self.file_format)?;
    println!("");
    write!(f, "The byte length of {} is: {}", &file_name, self.byte_len)?;

    Ok(())
  }
  
}
pub fn file_prop(file: PathBuf) -> Result<(), std::io::Error> {
  let metadata = &file.metadata().unwrap();
  let byte_len = metadata.len();

  let format = FileFormat::from_file(&file)?;
  let file_kind = format.kind();

  let file_ext = file.extension().unwrap().to_str().unwrap().to_string();
  let is_dir = file.is_dir();
  let is_file = file.is_file();
  let file_name = file.file_name().unwrap().to_str().unwrap().to_string();


  let file_property = FileProperty {
    file_name,
    is_dir,
    is_file,
    file_ext,
    file_kind,
    file_format: format,
    byte_len
  };

  println!("{file_property}");

  // Ok(file_property)
  Ok(())

}