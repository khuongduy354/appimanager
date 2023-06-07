use std::{env, fs::File, io::Write, os::unix::prelude::PermissionsExt, path::PathBuf};

use clap::{Parser, Subcommand};

/// Make a desktop entry for your AppImage
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Add {
        ///Path of appimage file
        appimage_path: PathBuf,

        /// Destination of desktop file
        #[arg(short, long, default_value = "~/.local/share/applications")]
        dest_dir: PathBuf,

        /// Move appimage file to some location before creating desktop file   
        #[arg(short, long)]
        move_dir: Option<PathBuf>,
    },
}
fn make_desktop_file(dest_dir: &PathBuf, app_path: &PathBuf) -> Result<(), std::io::Error> {
    //validate dest_dir, app_path
    // if !dest_dir.is_dir() {
    //     unimplemented!()
    // }

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

fn get_abs_path(path: &PathBuf) -> PathBuf {
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
fn is_exec(path: &PathBuf) -> bool {
    let metadata = std::fs::metadata(path).expect("cant get metadata");
    let permissions = metadata.permissions();
    if permissions.mode() & 0o111 != 0 {
        return true;
    } else {
        return false;
    }
}
fn main() -> Result<(), std::io::Error> {
    let app = Cli::parse();
    match app.commands {
        Commands::Add {
            appimage_path,
            dest_dir,
            move_dir,
        } => {
            let dest_dir = get_abs_path(&dest_dir);
            let app_file = appimage_path.file_name().expect("AppImage must be a file!");

            if appimage_path.is_file() && is_exec(&appimage_path) {
                let mut exec_path = get_abs_path(&appimage_path);

                // move (if needed) before create .desktop
                if let Some(move_dir) = move_dir {
                    if move_dir.is_dir() {
                        let move_file_path = move_dir.join(&app_file);
                        std::fs::rename(&appimage_path, &move_file_path).expect("cant move file");
                        exec_path = get_abs_path(&move_file_path);
                    }
                }

                // create .desktop
                make_desktop_file(&dest_dir, &exec_path)?;
            } else {
                print!("File not found, or not an executable!");
            }
        }
    }
    Ok(())
}

//TODO: after seperate in tests folder
#[test]
fn test_is_exec() {
    // let test_file = PathBuf::from("./testasdf.AppImage");
    // File::create(&test_file).unwrap();
    //
    // assert!(is_exec(&test_file));
    //
    // assert!(std::fs::remove_file("./testasdf.AppImage").is_ok());
}
//TODO: integration test, seperate test
#[test]
fn test_make_desktop_file() {
    //create test file
    let test_file = PathBuf::from("./testasdf.AppImage");
    File::create(&test_file).unwrap();

    //relative dir test
    let dest_dir = get_abs_path(&PathBuf::from("."));
    let app_path = get_abs_path(&PathBuf::from("./testasdf.AppImage"));

    let result = make_desktop_file(&dest_dir, &app_path);
    assert!(result.is_ok());

    //check if .desktop file exists
    let app_name = app_path.file_stem().unwrap().to_str().unwrap();
    let app_file = format!("{}.desktop", app_name);
    let desktop_file_path = dest_dir.join(app_file);
    assert!(desktop_file_path.is_file());

    //cleanup testfiles
    assert!(std::fs::remove_file(desktop_file_path).is_ok());
    assert!(std::fs::remove_file("./testasdf.AppImage").is_ok());
}
