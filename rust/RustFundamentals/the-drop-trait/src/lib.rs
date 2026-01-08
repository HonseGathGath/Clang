use std::{
    env::temp_dir,
    fs::{remove_file, File},
    io::Error,
    path::PathBuf,
};

pub struct TempFile {
    pub path: PathBuf,
}

impl TempFile {
    pub fn new(file_name: impl AsRef<str>) -> Result<Self, Error> {
        let mut temp_file = temp_dir();
        temp_file.push(file_name.as_ref());
        File::create(temp_file.as_path())?;
        Ok(TempFile { path: temp_file })
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = remove_file(self.path.as_path());
    }
}

// Example usage
pub fn main() {
    let file_path = PathBuf::from("example_temp_file.tmp");
    let tempfile =
        TempFile::new(file_path.to_str().unwrap()).expect("Failed to create temporary file");

    assert!(tempfile.path.exists(), "File does not exist");

    drop(tempfile);

    assert!(!file_path.exists(), "File was not deleted");

    let tempfile_2 = TempFile::new(&String::from("example_temp_file_2.tmp"))
        .expect("Failed to create temporary file");

    drop(tempfile_2);
}
