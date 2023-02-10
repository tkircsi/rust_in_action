use clap::Parser;
use regex::Regex;

/// Simple grep-lit program
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Pattern to search for
    #[arg(short, long)]
    pattern: String,
}

fn main() {
    let args = Args::parse();
    let search_term = "picture";
    let quote = "Every face, every shop, bedroom window, public-house, 
    and dark square is a picture feverishly turned--in search of what? 
    It is the same with books. What do we seek through millions of pages?";

    println!("=== Contains ===");
    contains(quote, search_term);
    println!("=== Regex find ===");
    search_regex(quote, search_term);
    println!("=== Cli find ===");
    cli_search(quote, &args);
}

/// Search for `search_term` in `quote`'s lines by using the `contains` method of str
fn contains(quote: &str, search_term: &str) {
    for line in quote.lines() {
        if line.contains(search_term) {
            println!("{}", line);
        }
    }
}

fn search_regex(quote: &str, search_term: &str) {
    let re = Regex::new(search_term).unwrap();

    for line in quote.lines() {
        match re.find(line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn cli_search(quote: &str, args: &Args) {
    let re = Regex::new(&args.pattern).unwrap();
    for line in quote.lines() {
        match re.find(line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}
