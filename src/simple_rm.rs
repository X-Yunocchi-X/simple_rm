use std::{
    env,
    ffi::OsStr,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
    time::UNIX_EPOCH,
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
    pub fn new(args: Vec<String>) -> Self {
        let home = env::var("HOME").unwrap();
        let trash_path = PathBuf::from(format!("{}/.local/share/Trash", home));
        SimpleRemove { trash_path, args }
    }

    pub fn execute(&self) {
        self.args.iter().skip(1).for_each(|arg| {
            if arg.starts_with("-") {
                // -r(f)
                todo!()
            } else {
                let file_path = Path::new(arg);
                self.remove_file(file_path);
            }
        })
    }

    pub fn remove_file(&self, file_path: &Path) {
        if file_path.is_dir() {
            self.create_info(file_path).unwrap();
            self.remove_dir(file_path);
        } else {
            self.move_file_to_trash(file_path).unwrap();
            self.create_info(file_path).unwrap();
        }
        // self.synchronize();
    }

    fn get_absolute_path(file_path: &Path) -> io::Result<Path> {
        if file_path.is_absolute() {
            return Ok(file_path);
        } else {
            let pwd = env::current_dir()?;
            return Ok(&pwd.join(file_path));
        }
    }

    /// Remove file to trash bin
    fn move_file_to_trash(&self, file_path: &Path) -> io::Result<()> {
        let file_name = Self::get_file_name(file_path);
        let trash_dir = self.trash_path.join("files");
        fs::rename(file_path, trash_dir.join(file_name))?;
        Ok(())
    }

    fn create_info(&self, file_path: &Path) -> io::Result<()> {
        let info_dir = self.trash_path.join("info");
        let file_name = Self::get_file_name(file_path);
        let extended_name = format!(
            "{}.trashinfo",
            file_name
                .to_str()
                .expect("Can not convert filename to utf-8")
        );
        let mut info_file = File::create(info_dir.join(extended_name))?;
        let time = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        let info_content = format!(
            "[Trash Info]\nPath={}\nDeletionDate={}",
            file_path.display(),
            time
        );
        info_file.write_all(info_content.as_bytes())?;
        // println!("{}", info_content);
        Ok(())
    }

    /// Create directory cache in $trash/directorysizes
    /// format: [size] [mtime] [name] mtime is the time of last modification
    fn create_dir_cache(&self) {
        todo!()
    }

    fn get_modified_time(file_path: &Path) -> io::Result<u64> {
        let modified_time = fs::metadata(file_path)?.modified()?;
        let duration_since_epoch = modified_time
            .duration_since(UNIX_EPOCH)
            .expect("time is before UNIX epoch");
        Ok(duration_since_epoch.as_secs())
    }

    fn get_file_name(path: &Path) -> &OsStr {
        path.file_name()
            .expect("Can not get filename, maybe the filename has invalid utf-8 characters")
    }

    fn remove_dir(&self, file_path: &Path) -> ! {
        todo!()
    }

    fn synchronize(&self) -> ! {
        todo!()
    }
}
