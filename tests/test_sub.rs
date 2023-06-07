// add, list, delete

// list 3
// add 1 full args
// new one has icon, appname
// list 4 apps
// delete 1
// list 3
#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use appimanager::{subcommands, Cli, Commands};

    fn list_sub() -> Cli {
        Cli {
            commands: Commands::List,
            dest_dir: PathBuf::from("./tests/desktop-files"),
        }
    }

    fn execute(app: Cli) {
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
    }

    fn test_all_ver1_0_0() {}
}
