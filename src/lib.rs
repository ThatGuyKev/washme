use clap::{App, Arg};
use std::error::Error;
use std::fs::{self, read_dir, DirEntry, File};
use std::io::{self, BufRead};
use std::path::Path;

const COMMENT_TAG: char = '#';
const ALL_TAG: char = '*';

type WashResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    path: Vec<String>,
    all: bool,
}

pub fn get_args() -> WashResult<Config> {
    let matches = App::new("washme")
        .version("0.1.0+nightly")
        .author("Ahmed Abdalla <ahmed_aallah@hotmail.com>")
        .arg(
            Arg::with_name("path")
                .value_name("PATH")
                .help("Directory to clear gitignore files/folders")
                .multiple(true)
                .default_value("."),
        )
        .arg(
            Arg::with_name("all")
                .short("a")
                .help("remove all files even with unstaged changes")
                .takes_value(false),
        )
        .get_matches();
    Ok(Config {
        path: matches.values_of_lossy("path").unwrap(),
        all: matches.is_present("all"),
    })
}
pub fn run(config: Config) -> WashResult<()> {
    for path in config.path {
        let mut git_ignore_path: String = path.to_owned();
        git_ignore_path.push_str("/.gitignore");

        if let Ok(lines) = read_lines(git_ignore_path) {
            for line in lines {
                if let Ok(file) = line {
                    if file.trim().is_empty() || is_comment(&file) {
                        continue;
                    }
                    println!("{}", file)
                }
            }
        }
        // let folders = files_to_remove.split("\n");
        // for folder in folders {
        //     let firstchar = folder.chars().next().unwrap();
        //     if firstchar == COMMENT_TAG || firstchar == ALL_TAG {
        //         continue;
        //     }
        //     println!(" {}", folder);
        //     let mut remove_file: String = path.to_owned();
        //     remove_file.push_str(folder);

        //     fs::remove_dir_all(remove_file)?;
        // }
    }
    Ok(())
}

fn do_something(entry: &DirEntry) {}

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn collect_entries() -> io::Result<()> {
    let mut entries = fs::read_dir(".")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_comment(line: &String) -> bool {
    if line.chars().next().unwrap() == COMMENT_TAG {
        return true;
    }
    false
}
