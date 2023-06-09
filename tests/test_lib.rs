#[cfg(test)]
mod test {
    use appimanager::{
        extract_appname, get_desk_list, is_exec, make_desktop_file, AddConfig, PathBufExtension,
    };
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
        let dest_dir = PathBuf::from("./tests").get_abs_path();
        let app_path = PathBuf::from("./tests/sample.AppImage").get_abs_path();

        let result = make_desktop_file(
            &dest_dir,
            &app_path,
            AddConfig {
                icon: None,
                name: None,
            },
        )
        .unwrap();

        //check if .desktop file exists
        let app_name = extract_appname(&app_path);
        let app_file = format!("{}.desktop", app_name);
        let desktop_file_path = dest_dir.join(app_file);
        assert!(desktop_file_path.is_file());

        //cleanup desktop file
        assert!(std::fs::remove_file(desktop_file_path).is_ok());
    }

    #[test]
    fn test_get_desk_list() {
        let desk_list = get_desk_list(&PathBuf::from("./tests/desktop-files")).unwrap();
        let mut file_names = Vec::new();
        for entry in desk_list {
            file_names.push(entry.file_name);
        }
        assert_eq!(
            file_names,
            vec!["app1.desktop", "app2.desktop", "app3.desktop"]
        );
    }
}
