use std::fs;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let git_ignore: String = fs::read_to_string(".gitignore")?.parse()?;
    println!("{}", git_ignore);
    Ok(())
}
