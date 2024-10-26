use clap::Parser;
use std::fs;
use std::io::{self};
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    /// The file being searched for.
    #[arg(short = 'f', long = "file_name", required = true)]
    file_name: Option<String>,
    /// The directory to look for the file. Searches recursively in any present folders.
    #[arg(short = 'd', long = "directory", required = true)]
    directory: Option<PathBuf>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if args.directory.is_some() && args.file_name.is_some() {
        let _result = search_in_directory(
            &args.directory.expect("Directory missing"),
            &args.file_name.expect("File name missing"),
        );
    } else {
        println!("Error: Run program with --help");
    }

    Ok(())
}

fn search_in_directory(directory: &PathBuf, file_name: &String) -> io::Result<()> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        let str: std::path::Display<'_> = path.display();

        if path.is_dir() {
            search_in_directory(&path, file_name)?; // Recursively search in subdirectories
        } else if path.is_file() {
            if let Some(name) = path.file_name() {
                if let Some(name_str) = name.to_str() {
                    if name_str == file_name {
                        println!("{str}");
                    }
                }
            }
        }
    }
    Ok(())
}
