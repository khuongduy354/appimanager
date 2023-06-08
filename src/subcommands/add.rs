use crate::{is_exec, make_desktop_file, AddConfig, PathBufExtension};
use std::path::PathBuf;

pub fn add(
    dest_dir: &PathBuf,
    appimage_path: &PathBuf,
    move_dir: &Option<PathBuf>,
    name: Option<String>,
    icon: Option<PathBuf>,
) -> Result<(), std::io::Error> {
    let dest_dir = dest_dir.get_abs_path();
    let app_file = appimage_path.file_name().expect("AppImage must be a file!");

    if appimage_path.is_file() && is_exec(&appimage_path) {
        let mut exec_path = appimage_path.get_abs_path();
        // move (if needed) before create .desktop
        if let Some(move_dir) = move_dir {
            let move_dir = move_dir.get_abs_path();
            if move_dir.is_dir() {
                let move_file_path = move_dir.join(&app_file);
                std::fs::rename(&appimage_path, &move_file_path).expect("cant move file");
                exec_path = move_file_path.get_abs_path();
                println!("Moved executable to: {}", exec_path.display());
            }
        }

        // create .desktop
        make_desktop_file(&dest_dir, &exec_path, AddConfig { icon, name })?;

        Ok(())
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "File not found or not executable!",
        ));
    }
}
