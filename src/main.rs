use clap::Parser;
use std::path::PathBuf;
use std::fs::File;
use std::io::{self, BufRead};
use anyhow::{Context, Result};



#[derive(Parser)]
struct Cli {
    pattern: String,
    path: PathBuf,
    #[arg(short = 'i', long = "insensitive")]
    insensitive: bool,
    #[arg(short = 'v', long = "invert")]
    invert: bool,
}

fn open_to_reader(path: &PathBuf) -> Result<io::BufReader<File>> {
    let fd = File::open(path)
        .with_context(|| format!("Failed to open file {:?}", path))?;
    Ok(io::BufReader::new(fd))
}

fn insensitive_search(pattern: &str, path: &PathBuf) -> Result<()> {
    let reader = open_to_reader(path)?;
    let pattern_lower = pattern.to_lowercase();

    for line in reader.lines() {
        let line_lower = &line.as_ref().unwrap().to_lowercase();
        if line_lower.contains(&pattern_lower) {
            println!("{}", line?);
        }
    }
    Ok(())
}

fn case_sensitive_search(pattern: &str, path: &PathBuf) -> Result<()> {
    let reader = open_to_reader(path)?;

    for line in reader.lines() {
            if line.as_ref().unwrap().contains(pattern) {
                println!("{}", line?);
            }   
        }
    Ok(())
}

fn invert_search(pattern: &str, path: &PathBuf) -> Result<()> {
    let reader = open_to_reader(path)?;

    for line in reader.lines() {
        if !line.as_ref().unwrap().contains(pattern) {
            println!("{}", line?);
        }
    }
    Ok(())
}
fn main() -> Result<()> {
    let args = Cli::parse();
    
    if args.insensitive == true {
        insensitive_search(&args.pattern, &args.path)
            .with_context(|| format!("Failed to search for pattern '{}' in file {:?}", args.pattern, args.path))?;
    } else if args.invert == true {
        invert_search(&args.pattern, &args.path)
            .with_context(|| format!("Failed to search for pattern '{}' in file {:?}", args.pattern, args.path))?;
    } else {
    case_sensitive_search(&args.pattern, &args.path)
        .with_context(|| format!("Failed to search for pattern '{}' in file {:?}", args.pattern, args.path))?;
    }
    Ok(())
}
