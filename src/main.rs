use appim::{subcommands, Cli, Commands, PathBufExtension};
use clap::Parser;
// pub mod subcommands;

fn main() -> Result<(), std::io::Error> {
    let app = Cli::parse();
    let dest_dir = app.dest_dir.get_abs_path();
    match app.commands {
        Commands::Add {
            appimage_path,
            move_dir,
            name,
            icon,
        } => subcommands::add(&dest_dir, &appimage_path, &move_dir, name, icon)?,
        Commands::List => {
            subcommands::list(&dest_dir)?;
        }
        Commands::Delete { idx } => {
            subcommands::delete(idx, &dest_dir)?;
        }
    }
    Ok(())
}
