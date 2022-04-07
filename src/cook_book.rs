use std::{env, fs};

fn visit_dir() -> Result<()> {
    let dir = env::current_dir()?;

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        let metadata = fs::metadata(&path)?;
        if metadata.is_file() {
            // check if file name is ".gitignore" and apply ignore rules for the dir
            // check if file expressions applies and push to files_to_remove
        }
        if metadata.is_dir() {
            // check if dir expressions applies and push to dirs_to_remove
            // else
            // visit_dir(path)
        }
    }
}
