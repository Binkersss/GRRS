mod testing;

use clap::Parser;
use anyhow::{Context, Result};
use std::io::BufRead;
use std::time::Duration;

#[derive(Parser)]
struct Cli {
    pattern: String,
    // PathBuf is like a string but for cross-platform file system paths
    path: std::path::PathBuf,
}

fn find_and_print_matches(line: &mut String, pattern: &str, 
                          reader: &mut std::io::BufReader<std::fs::File>, 
                          mut writer: impl std::io::Write) {
    loop {
        line.clear();

        let bytes_read = reader.read_line(line).expect("could not read line");

        if bytes_read == 0 {
            break;
        }

        if line.contains(pattern) {
            writeln!(writer, "{}", line).expect("");
        }
    }
}

fn main() ->Result<(), Box<dyn std::error::Error>>{
    let bar = indicatif::ProgressBar::new_spinner();
    bar.enable_steady_tick(Duration::from_millis(100));

    let args = Cli::parse();

    let file = std::fs::File::open(&args.path).with_context(|| format!("could not read file '{}'", args.path.display()))?;
    let mut reader = std::io::BufReader::new(file);

    let mut line = String::new();

    find_and_print_matches(&mut line, &args.pattern, &mut reader, &mut std::io::stdout());
    bar.finish_and_clear();
    Ok(())
}
