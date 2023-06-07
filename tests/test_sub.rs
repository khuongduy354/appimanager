// add, list, delete integration test

// list 3 files
// add 1
// list 4 files
// delete 1
// list 3 files
#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use appimanager::{
        extract_appname, get_desk_list, make_desktop_file, subcommands, AddConfig, Cli, Commands,
        PathBufExtension,
    };

    fn list() -> Vec<String> {
        let desk_list = get_desk_list(&PathBuf::from("./tests/desktop-files")).unwrap();
        let mut file_names = Vec::new();
        for entry in desk_list {
            file_names.push(entry.file_name);
        }
        return file_names;
    }
    fn add() {
        // create .desktop file
        let dest_dir = PathBuf::from("./tests/desktop-files").get_abs_path();
        let app_path = PathBuf::from("./tests/sample.AppImage").get_abs_path();

        make_desktop_file(
            &dest_dir,
            &app_path,
            AddConfig {
                icon: Some(PathBuf::from("./tests/icon.png").get_abs_path()),
                name: Some("app4".to_string()),
            },
        )
        .unwrap();

        //check if .desktop file exists
        let app_name = "app4";
        let app_file = format!("{}.desktop", app_name);
        let desktop_file_path = dest_dir.join(app_file);
        assert!(desktop_file_path.is_file());
    }
    fn delete() {
        subcommands::delete(3, &PathBuf::from("./tests/desktop-files")).unwrap();
    }

    #[test]
    fn test_all_ver1_0_0() {
        let desk_files = list();
        assert_eq!(
            desk_files,
            vec!["app1.desktop", "app2.desktop", "app3.desktop"]
        );
        add();
        let desk_files = list();
        assert_eq!(
            desk_files,
            vec![
                "app1.desktop",
                "app2.desktop",
                "app3.desktop",
                "app4.desktop"
            ]
        );
        delete();
        let desk_files = list();
        assert_eq!(
            desk_files,
            vec!["app1.desktop", "app2.desktop", "app3.desktop"]
        );
    }
}
