use appimanager::{is_exec, make_desktop_file, PathBufExtension};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

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
fn main() -> Result<(), std::io::Error> {
    let app = Cli::parse();
    match app.commands {
        Commands::Add {
            appimage_path,
            dest_dir,
            move_dir,
        } => {
            let dest_dir = dest_dir.get_abs_path();
            let app_file = appimage_path.file_name().expect("AppImage must be a file!");

            if appimage_path.is_file() && is_exec(&appimage_path) {
                let mut exec_path = appimage_path.get_abs_path();
                // move (if needed) before create .desktop
                if let Some(move_dir) = move_dir {
                    if move_dir.is_dir() {
                        let move_file_path = move_dir.get_abs_path().join(&app_file);
                        std::fs::rename(&appimage_path, &move_file_path).expect("cant move file");
                        exec_path = move_file_path.get_abs_path();
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
