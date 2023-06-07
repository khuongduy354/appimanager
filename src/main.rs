use appimanager::PathBufExtension;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod subcommands;

/// Make a desktop entry for your AppImage
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,

    /// Destination path that store all .desktop files
    #[arg(short, long, default_value = "~/.local/share/applications")]
    dest_dir: PathBuf,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Add {
        ///Path of appimage file
        appimage_path: PathBuf,

        /// Move appimage file to some location before creating desktop file   
        #[arg(short, long)]
        move_dir: Option<PathBuf>,
    },
    List,
    Delete {
        /// Delete desktop file by index (displayed by list subcommand)
        #[arg(short, long)]
        idx: usize,
    },
}
fn main() -> Result<(), std::io::Error> {
    let app = Cli::parse();
    let dest_dir = app.dest_dir.get_abs_path();
    match app.commands {
        Commands::Add {
            appimage_path,
            move_dir,
        } => subcommands::add(&dest_dir, &appimage_path, &move_dir)?,
        Commands::List => {
            subcommands::list(&dest_dir)?;
        }
        Commands::Delete { idx } => {
            subcommands::delete(idx, &dest_dir)?;
        }
    }
    Ok(())
}
