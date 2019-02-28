use std::env;
use fumie::repl as REPL;
use fumie::options::Options;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut options = Options::new();
    options.load_options(args);

    if options.get("input") == Some(&String::from("repl")) {
        if options.get("--repl") == Some(&String::from("per_block")) {
            REPL::open("per_block");
        }
        else {
            REPL::open("per_line");
        }
    }
}
