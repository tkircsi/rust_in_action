use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// Simple grep-lit program
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Pattern to search for
    #[arg(short, long)]
    pattern: String,

    /// Input file
    #[arg(short, long, default_value_t = String::from("-"))]
    input: String,
}

fn main() {
    let args = Args::parse();
    let input = &args.input;
    let re = Regex::new(&args.pattern).unwrap();

    if input == "-" {
        let stdin = std::io::stdin();
        let reader = stdin.lock();
        grep(reader, re);
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        grep(reader, re);
    }
}

fn grep<R: BufRead>(r: R, re: Regex) {
    for line_ in r.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}
