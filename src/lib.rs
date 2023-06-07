use std::{env, fs::File, io::Write, os::unix::prelude::PermissionsExt, path::PathBuf};

pub fn make_desktop_file(dest_dir: &PathBuf, app_path: &PathBuf) -> Result<(), std::io::Error> {
    //TODO: validate dest_dir, app_path
    // if !dest_dir.is_dir() {
    //     unimplemented!()
    // }

    //TODO: absolute path

    //make .desktop path
    let app_name = app_path.file_stem().unwrap().to_str().unwrap();
    let app_file = format!("{}.desktop", app_name);
    let desktop_file_path = dest_dir.join(app_file);

    //write .desktop file
    let mut file = File::create(desktop_file_path)?;
    let content = format!(
        "[Desktop Entry]\nName={}\nExec={}\nType=Application\nCategories=Utility;",
        app_name,
        app_path.display(),
    );
    file.write_all(content.as_bytes())?;
    Ok(())
}

//TODO: relative?
pub fn get_abs_path(path: &PathBuf) -> PathBuf {
    if path.is_relative() {
        if path.starts_with("~") {
            //on linux so, this is acceptable
            let path = path.strip_prefix("~").unwrap();
            return env::home_dir()
                .expect("cant get home dir")
                .join(path)
                .canonicalize()
                .expect("cant get abs path");
        } else {
            env::current_dir()
                .unwrap()
                .join(path)
                .canonicalize()
                .expect("cant get abs path")
        }
    } else {
        path.clone()
    }
}
//TODO: absoute
pub fn is_exec(path: &PathBuf) -> bool {
    let metadata = std::fs::metadata(path).expect("cant get metadata");
    let permissions = metadata.permissions();
    if permissions.mode() & 0o111 != 0 {
        return true;
    } else {
        return false;
    }
}
