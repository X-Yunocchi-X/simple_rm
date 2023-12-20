use std::{
    fs::{self, File},
    path::{Path, PathBuf}, io::{self, Write},
};

/// To remove file to trash bin, need to create file.trashinfo in $trash/info/
/// if remove a directory, need to add details in $trash/directorysizes
/// remove file to $trash/files/
pub struct SimpleRemove {
    trash_path: PathBuf, // Path to trash bin
    args: Vec<String>,
}

// Usage:
// simple_rm -rf <file>
impl SimpleRemove {
    pub fn new(trash_path: PathBuf, args: Vec<String>) -> Self {
        return SimpleRemove { trash_path, args };
    }

    pub fn remove_file(&self, file_path: &Path) {
        self._remove_file(file_path).unwrap();
        self.create_info(file_path).unwrap();
    }

    /// Remove file to trash bin
    fn _remove_file(&self, file_path: &Path) -> io::Result<()> {
        let file_name = file_path
            .file_name()
            .expect("Can not get filename, maybe the filename has invalid utf-8 characters");
        let trash_dir = self.trash_path.join("files");
        fs::rename(file_path, trash_dir.join(file_name))?;
        Ok(())
    }

    fn create_info(&self, file_path: &Path) -> io::Result<()> {
        let info_dir = self.trash_path.join("info");
        let file_name = file_path
            .file_name()
            .expect("Can not get filename, maybe the filename has invalid utf-8 characters");
        let extended_name = format!("{}.trashinfo", file_name.to_str().expect("Can not convert filename to utf-8"));
        let mut info_file = File::create(info_dir.join(extended_name))?;
        let time = chrono::Local::now();
        let format_time = time.format("%Y-%m-%dT%H:%M:%S").to_string();
        let info_content = format!("[Trash Info]\nPath={}\nDeletionDate={}", file_path.display(), format_time);
        info_file.write_all(info_content.as_bytes())?;
        println!("{}", info_content);
        Ok(())
    }
}
