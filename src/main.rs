use std::{env, ffi::OsStr, fs::File, io::Write, path::PathBuf};

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
        move_dest: Option<PathBuf>,
    },
}
//TODO: simplify arguments
fn make_desktop_file(dest_dir: &PathBuf, app_path: &PathBuf, app_name: &str) {
    if !dest_dir.is_dir() {
        unimplemented!()
    }
    let mut app_file = app_name.to_string();
    app_file.push_str(".desktop");

    let desktop_file_path = dest_dir.join(app_file);
    let mut file = File::create(desktop_file_path).expect("Unable to create file");
    let content = format!(
        "[Desktop Entry]\nName={}\nExec={}\nType=Application\nCategories=Utility;",
        app_name,
        app_path.display(),
    );
    file.write_all(content.as_bytes())
        .expect("Unable to write data");
}
fn get_abs_path(path: &PathBuf) -> PathBuf {
    if path.is_relative() {
        env::current_dir()
            .unwrap()
            .join(path)
            .canonicalize()
            .unwrap()
            .clone()
    } else {
        path.clone()
    }
}

fn main() {
    let app = Cli::parse();
    match app.commands {
        Commands::Add {
            appimage_path,
            dest_dir,
            move_dest,
        } => {
            let app_file = appimage_path.file_name().expect("AppImage must be a file!");
            let app_name = PathBuf::from(app_file.clone());
            let app_name = app_name.file_stem().unwrap().to_str().unwrap();

            if appimage_path.is_file() {
                let mut exec_path = get_abs_path(&appimage_path);

                if let Some(ex) = appimage_path.extension() {
                    if ex == "AppImage" {
                        // move (if needed) before desktop Entry
                        if let Some(move_dest) = move_dest {
                            if move_dest.is_dir() {
                                //TODO: handle  move_dest relative
                                let move_dest = move_dest.join(&app_file);
                                std::fs::rename(&appimage_path, &move_dest)
                                    .expect("cant move file");
                                exec_path = move_dest;
                            }
                        }
                        make_desktop_file(&dest_dir, &exec_path, app_name);
                    } else {
                        println!("Not an AppImage");
                    }
                } else {
                    println!("File dont have extension");
                }
            } else {
                print!("File not found");
            }
        }
    }
}
