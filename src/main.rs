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
        /// Name of the person to greet
        #[arg(short, long)]
        appimage_path: PathBuf,
        // /// Number of times to greet
        // #[arg(short, long, default_value_t = 1)]
        // count: u8,
    },
}
fn make_desktop_file(dest: &PathBuf, app_path: &PathBuf, app_name: &str) {
    println!("Dest: {:?}", app_path);
    let mut file = File::create(dest).expect("Unable to create file");
    let content = format!(
        "[Desktop Entry]\nName={}\nExec={}\nType=Application\nCategories=Utility;",
        app_name,
        app_path.display(),
    );
    file.write_all(content.as_bytes())
        .expect("Unable to write data");
}

fn main() {
    let app = Cli::parse();
    match app.commands {
        Commands::Add { appimage_path } => {
            let mut dest = PathBuf::from("/usr/share/applications");
            let app_file = appimage_path.file_name().expect("AppImage must be a file!");
            dest.push(&app_file);
            let app_name = PathBuf::from(app_file.clone());
            let app_name = app_name.file_stem().unwrap().to_str().unwrap();

            if appimage_path.is_file() {
                let current_dir = env::current_dir().expect("cant get abs path");
                let exec_path = current_dir.join(app_file);

                if let Some(ex) = dest.extension() {
                    if ex == "AppImage" {
                        make_desktop_file(&dest, &exec_path, app_name);
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
