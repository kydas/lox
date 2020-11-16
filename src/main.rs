use std::env;
use std::process;
use std::fs;
use std::io::stdin;
use std::io;
use std::io::Write;

pub mod token_type;
pub mod token;

fn main() {
    let args: Vec<String> =  env::args().collect();
    let args_len = args.len();

    if args_len > 2 {
        println!("Usage: jlox [script]");
        process::exit(0);
    } else if args_len == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &String) {
    let contents = fs::read_to_string(path)
    .expect("Something went wrong reading the file");

    run(&contents);
    
    // if hadError { process::exit(0) }
}

fn run_prompt() {
    let _line = &mut String::new();
    let stdin = stdin();
    loop {
        print!("> ");
        let mut input = String::new();
        io::stdout().flush().unwrap();
        let _gar = stdin.read_line(&mut input).expect("Failed to read line");
        let line = input.trim_end();
        run(& String::from(line));
        // hadError = false;
    }
}

fn run(source: & str) {
    let tokens: Vec<&str> = source.split(' ').collect();

    for t in tokens {
        println!("token: {}", t);
    }
}

struct Lox {
    had_error: bool
}

impl Lox {
    fn error(self, line: u32, message: &str) {
        self.report(line, "", message);
    }

    fn report(mut self, line: u32, from: &str, message: &str) {
        eprintln!("[line {}] Error {}: {}", line, from, message);
        self.had_error = true; 
    }
}











