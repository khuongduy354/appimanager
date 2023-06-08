use clap::{Parser, Subcommand};
pub mod subcommands;
use std::{env, fs::File, io::Write, os::unix::prelude::PermissionsExt, path::PathBuf};
/// Make a desktop entry for your AppImage
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,

    /// Destination path that store all .desktop files
    #[arg(short, long, default_value = "~/.local/share/applications")]
    pub dest_dir: PathBuf,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Generate .desktop file
    Add {
        ///Path of appimage file
        appimage_path: PathBuf,

        /// Move appimage file to some location before creating desktop file   
        #[arg(short, long)]
        move_dir: Option<PathBuf>,

        /// Name of app
        #[arg(short, long)]
        name: Option<String>,

        /// Path to icon
        #[arg(short, long)]
        icon: Option<PathBuf>,
    },

    /// List .desktop files in dest_dir (default=~/.local/share/applications)
    List,

    /// Delete .desktop file by index (displayed by list subcommand)
    Delete { idx: usize },
}
#[derive(Debug)]
pub struct DesktopEntry {
    pub path: PathBuf,
    pub file_name: String,
}

pub struct AddConfig {
    pub icon: Option<PathBuf>,
    pub name: Option<String>,
}
impl AddConfig {
    fn parse(&self, icon_default: PathBuf, name_default: String) -> (PathBuf, String) {
        let icon = self.icon.as_ref().unwrap_or(&icon_default);
        let name = self.name.as_ref().unwrap_or(&name_default);
        (icon.get_abs_path().clone(), name.clone())
    }
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
        for entry in dest_dir.read_dir()? {
            let entry = entry?;
            if let Some(ext) = entry.path().extension() {
                if ext == "desktop" {
                    let desk_entry = DesktopEntry {
                        path: entry.path(),
                        file_name: entry.file_name().to_str().unwrap().to_string(),
                    };
                    result.push(desk_entry);
                }
            }
        }
        result.sort_by(|a, b| a.file_name.cmp(&b.file_name));
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
pub fn make_desktop_file(
    dest_dir: &PathBuf,
    app_path: &PathBuf,
    config: AddConfig,
) -> Result<(), std::io::Error> {
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

    //prepare content

    let (icon_path, app_name) = config.parse(PathBuf::from(""), extract_appname(&app_path));

    let app_file = format!("{}.desktop", app_name);
    let desktop_file_path = dest_dir.join(&app_file);

    //write .desktop file
    let mut file = File::create(desktop_file_path)?;
    let content = format!(
        "[Desktop Entry]\nName={}\nExec={}\nType=Application\nCategories=Utility;\nIcon={}",
        app_name,
        app_path.display(),
        icon_path.display()
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
