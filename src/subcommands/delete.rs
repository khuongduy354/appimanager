use std::path::PathBuf;

use crate::get_desk_list;
pub fn delete(idx: usize, dest_dir: &PathBuf) -> Result<(), std::io::Error> {
    let desk_list = get_desk_list(dest_dir)?;
    let entry = &desk_list[idx];
    std::fs::remove_file(&entry.path)?;
    println!("{} deleted", entry.file_name);

    Ok(())
}
