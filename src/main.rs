use std::{env, fs::File, io::Write, path::PathBuf};

use clap::{Parser, Subcommand};

/// Simple program to greet a person
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
        #[arg(short, long, default_value = "/usr/share/applications")]
        dest_dir: PathBuf,

        /// Move appimage file to some location before creating desktop file   
        #[arg(short, long)]
        move_dir: Option<PathBuf>,
    },
}
fn make_desktop_file(dest_dir: &PathBuf, app_path: &PathBuf) -> Result<(), std::io::Error> {
    //validate dest_dir, app_path
    if !dest_dir.is_dir() {
        unimplemented!()
    }

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
    // println!("path: {:?}", path);
    if path.is_relative() {
        env::current_dir()
            .unwrap()
            .join(path)
            .canonicalize()
            .expect("cant get abs path")
    } else {
        path.clone()
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
            let app_file = appimage_path.file_name().expect("AppImage must be a file!");

            if appimage_path.is_file() {
                let mut exec_path = get_abs_path(&appimage_path);

                if let Some(ex) = appimage_path.extension() {
                    if ex == "AppImage" {
                        // move (if needed) before create .desktop
                        if let Some(move_dir) = move_dir {
                            if move_dir.is_dir() {
                                //TODO: dont know why dont run
                                let move_file_path = move_dir.join(&app_file);
                                std::fs::rename(&appimage_path, &move_file_path)
                                    .expect("cant move file");
                                exec_path = get_abs_path(&move_file_path);
                            }
                        }

                        // create .desktop
                        make_desktop_file(&dest_dir, &exec_path)?;
                    } else {
                        println!("Not an AppImage");
                    }
                } else {
                    println!("File dont have extension");
                }
            } else {
                print!("AppImage not found");
            }
        }
    }
    Ok(())
}

//TODO: seperate test
// integration tests consists of all option
#[test]
fn test_make_desktop_file() {
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

    //remove that file
    assert!(std::fs::remove_file(desktop_file_path).is_ok());
}
