use crate::{get_desk_list, PathBufExtension};
use std::path::PathBuf;
pub fn list(dest_dir: &PathBuf) -> Result<(), std::io::Error> {
    let dest_dir = dest_dir.get_abs_path();
    let desk_list = get_desk_list(&dest_dir)?;
    for (idx, entry) in desk_list.iter().enumerate() {
        let digit_count = &desk_list.len().to_string().len();
        println!(
            "\n{:>digit_count$}. {:10}\n{}",
            idx,
            entry.file_name,
            entry.path.display()
        );
    }
    Ok(())
}
