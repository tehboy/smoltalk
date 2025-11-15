use clap::Parser;
use smol::eval;
use std::io::{self, BufRead, Write};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // The script file to run
    #[arg(index = 1)]
    file_path: Option<String>,

    // Expression to evaluate
    #[arg(short = 'e', long = "eval")]
    expr: Option<String>,
}

fn main() {
    let args = Args::parse();

    // Mode 1: Evaluate expression from command line
    if let Some(expr) = args.expr {
        let result = eval(&expr);
        println!("{}", result);
        return;
    }

    // Mode 2: Evaluate file contents
    if let Some(file_path) = args.file_path {
        let code = std::fs::read_to_string(&file_path).expect("Could not read file");
        let result = eval(&code);
        println!("{}", result);
        return;
    }

    // Mode 3: REPL mode - read from stdin
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        match line {
            Ok(input) => {
                if !input.trim().is_empty() {
                    let result = eval(&input);
                    println!("{}", result);
                }
                // Flush to ensure prompt appears immediately
                stdout.flush().unwrap();
            }
            Err(_) => break, // EOF or error
        }
    }
}
