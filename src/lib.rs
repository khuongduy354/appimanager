use std::{env, fs::File, io::Write, os::unix::prelude::PermissionsExt, path::PathBuf};
pub struct DesktopEntry {
    pub idx: usize,
    pub path: PathBuf,
    pub file_name: String,
}
pub trait PathBufExtension {
    fn get_abs_path(&self) -> PathBuf;
}
impl PathBufExtension for PathBuf {
    fn get_abs_path(&self) -> PathBuf {
        if self.is_relative() {
            if self.starts_with("~") {
                //on linux so, this is acceptable
                let temp = self.strip_prefix("~").unwrap();
                return env::home_dir()
                    .expect("cant get home dir")
                    .join(temp)
                    .canonicalize()
                    .expect("cant get abs path");
            } else {
                env::current_dir()
                    .unwrap()
                    .join(self)
                    .canonicalize()
                    .expect("cant get abs path")
            }
        } else {
            self.clone()
        }
    }
}
pub fn get_desk_list(dest_dir: &PathBuf) -> Result<Vec<DesktopEntry>, std::io::Error> {
    let dest_dir = dest_dir.get_abs_path();
    let mut result = Vec::new();
    if dest_dir.is_dir() {
        for (idx, entry) in dest_dir.read_dir()?.enumerate() {
            let entry = entry?;
            if let Some(ext) = entry.path().extension() {
                if ext == "desktop" {
                    let desk_entry = DesktopEntry {
                        idx,
                        path: entry.path(),
                        file_name: entry.file_name().to_str().unwrap().to_string(),
                    };
                    result.push(desk_entry);
                }
            }
        }
        Ok(result)
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Directory not found",
        ));
    }
}
pub fn extract_appname(path: &PathBuf) -> String {
    let app_name = path.file_stem().unwrap().to_str().unwrap();
    app_name.to_string()
}
pub fn make_desktop_file(dest_dir: &PathBuf, app_path: &PathBuf) -> Result<(), std::io::Error> {
    if !dest_dir.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "dest_dir is not a directory",
        ));
    } else if !app_path.is_file() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "app_path is not a file",
        ));
    }

    let dest_dir = dest_dir.get_abs_path();
    let app_path = app_path.get_abs_path();

    //make .desktop path
    let app_name = extract_appname(&app_path);
    let app_file = format!("{}.desktop", app_name);
    let desktop_file_path = dest_dir.join(&app_file);

    //write .desktop file
    let mut file = File::create(desktop_file_path)?;
    let content = format!(
        "[Desktop Entry]\nName={}\nExec={}\nType=Application\nCategories=Utility;",
        app_name,
        app_path.display(),
    );
    file.write_all(content.as_bytes())?;
    println!("{} created", &app_file);
    Ok(())
}

pub fn is_exec(path: &PathBuf) -> bool {
    let metadata = std::fs::metadata(path.get_abs_path()).expect("cant get metadata");
    let permissions = metadata.permissions();
    if permissions.mode() & 0o111 != 0 {
        return true;
    } else {
        return false;
    }
}
