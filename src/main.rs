use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod subcommands;

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
        } => subcommands::add(&dest_dir, &appimage_path, move_dir),
    }
    Ok(())
}
