## GRRS
This is a command line program that allows for searching a file, line by line, for all instances of a keyword.

## Installing Globally 
With Cargo installed, you may run cargo install chap_grrs
## Example Usage
 ```
~/$ chap_grrs -- let src/main.rs

    let bar = indicatif::ProgressBar::new_spinner();
    
    let args = Cli::parse();
    
    let file = std::fs::File::open(&args.path).with_context(|| format!("could not read file '{}'", args.path.display()))?;
    
    let mut reader = std::io::BufReader::new(file);
    
    let mut line = String::new();
