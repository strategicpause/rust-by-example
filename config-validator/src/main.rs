use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    /// The path to the config file.
    #[arg(short, long)]
    config: String,

    /// A comma-separated list of keys to look for in a config file.
    #[arg(short, long)]
    keys: String,

    /// The path to the output file.
    #[arg(short, long)]
    output: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut keys: HashSet<String> = read_keys(args.keys);

    let file = File::open(args.config)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
            if parts.len() == 2 {
                keys.remove(parts[0]);
            }
        }
    }


    if keys.is_empty() {
        File::create(args.output)?;
    }

    Ok(())
}

// Given a comma-separated list of keys, return a HashSet of unique & trimmed keys.
fn read_keys(keys: String) -> HashSet<String> {
    keys.split(',')
        .map(str::trim)
        .filter(|&s| !s.is_empty())
        .map(String::from)
        .collect()
}
