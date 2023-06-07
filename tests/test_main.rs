#[cfg(test)]
mod test {
    use appimanager::{get_abs_path, is_exec, make_desktop_file};
    use std::path::PathBuf;

    #[test]
    fn test_is_exec() {
        let exec = PathBuf::from("tests/isexec.AppImage");
        let notexec = PathBuf::from("tests/notexec.AppImage");

        assert!(is_exec(&exec));
        assert!(!is_exec(&notexec));
    }

    #[test]
    fn test_make_desktop_file() {
        // create .desktop file
        let dest_dir = get_abs_path(&PathBuf::from("./tests"));
        let app_path = get_abs_path(&PathBuf::from("./tests/sample.AppImage"));

        let result = make_desktop_file(&dest_dir, &app_path);
        assert!(result.is_ok());

        //check if .desktop file exists
        let app_name = app_path.file_stem().unwrap().to_str().unwrap();
        let app_file = format!("{}.desktop", app_name);
        let desktop_file_path = dest_dir.join(app_file);
        assert!(desktop_file_path.is_file());

        //cleanup desktop file
        assert!(std::fs::remove_file(desktop_file_path).is_ok());
    }
}
