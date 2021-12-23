use std::fs::read_to_string;

mod tokenizer;
mod interpreter;

use interpreter::interpret;
use tokenizer::tokenize;

fn main() {
    let macrofile = read_to_string("macrofile").unwrap();
    for x in 0..3 {
        println!("Executing in {}", 3-x);
        std::thread::sleep(std::time::Duration::from_secs(1))
    }
    interpret(
        tokenize(macrofile)
    )
}