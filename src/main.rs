use std::fs::read_to_string;

mod tokenizer;
mod interpreter;
mod ascii_to_in;

use interpreter::interpret;
use tokenizer::tokenize;

fn main() {
    let macrofile = read_to_string("macrofile").unwrap();
    for x in 0..3 {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.5));
        println!("Executing in {}", 3-x);
    }

    let tokens = tokenize(macrofile);

    interpret(
        tokens
    )
}