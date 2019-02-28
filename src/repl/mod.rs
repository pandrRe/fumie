use std::io::{self, BufRead, Read, Write};
use super::lexer::Lexer;

pub fn open(method: &str) {
    let stdin = io::stdin();
    let mut buffer = String::new();

    if method == "per_line" {
        repl_per_line(&stdin, &mut buffer);
    }
    else if method == "per_block" {
        repl_per_block(&stdin, &mut buffer);
    }
}

fn repl_per_block(stdin: &std::io::Stdin, mut buffer: &mut String) {
    loop {
        buffer.clear();
        print!(">> ");
        io::stdout().flush().expect("Error when flushing.");

        stdin.lock().read_to_string(&mut buffer).expect("Input error.");

        let lexer = Lexer::new(&buffer);

        for token in lexer {
            println!("{:?}", token);
        }
    }
}

fn repl_per_line(stdin: &std::io::Stdin, mut buffer: &mut String) {
    loop {
        buffer.clear();
        print!(">> ");
        io::stdout().flush().expect("Error when flushing.");

        stdin.lock().read_line(&mut buffer).expect("Input error.");

        let lexer = Lexer::new(&buffer);

        for token in lexer {
            println!("{:?}", token);
        }
    }
}