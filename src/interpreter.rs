use crate::tokenizer::Token;
use enigo::*;
use std::collections::HashMap;

pub fn interpret(input: Vec<Token>) {
    let mut input = input.into_iter();

    let mut actions: HashMap<String, Vec<Token>> = HashMap::new();

    let mut variables: HashMap<String, Token> = HashMap::new();

    let mut keyboard = enigo::Enigo::new();

    while let Some(token) = input.next() {
        let action: bool;
        let name = match token {
            Token::Id(id) => {
                action = false;
                id
            }
            Token::ActionTilde => {
                action = true;
                match input.next().unwrap() {
                    Token::Id(id) => id,
                    any => panic!("Unexpected action: {:?}", any),
                }
            }
            _ => panic!("Unexpected tok: {:?}", token),
        };

        if action {
            interpret(actions.get(&name).unwrap().clone());
            continue;
        }

        use crate::ascii_to_in::input_to_keycode;

        // Probably faster than a hashmap?
        match name.as_str() {
            "inp" => match input.next().unwrap() {
                Token::Str(input) => keyboard.key_sequence(&input),
                any => panic!("Unexpected tok: {:?}", any),
            },
            "action" => {
                let name = match input.next().unwrap() {
                    Token::Id(id) => id,
                    any => panic!("Unexpected tok: {:?}", any),
                };
                let scope = match input.next().unwrap() {
                    Token::Scope(scope) => scope,
                    any => panic!("Unexpected tok: {:?}", any),
                };
                actions.insert(name, scope);
            }
            "kdn" => {
                let code = match input.next().unwrap() {
                    Token::Id(id) => id,
                    any => panic!("Err unexpected tok: {:?}", any),
                };
                keyboard.key_down(input_to_keycode(code))
            }
            "kup" => {
                let code = match input.next().unwrap() {
                    Token::Id(id) => id,
                    any => panic!("Err unexpected tok: {:?}", any),
                };
                keyboard.key_up(input_to_keycode(code))
            }
            "kbd" => {
                let code = match input.next().unwrap() {
                    Token::Id(id) => id,
                    any => panic!("Err unexpected tok: {:?}", any),
                };
                keyboard.key_click(input_to_keycode(code))
            }
            "seq" => {
                let code = match input.next().unwrap() {
                    Token::Str(str) => str,
                    any => panic!("Err unexpected tok: {:?}", any),
                };
                keyboard.key_sequence_parse(&code)
            }
            "wait" => {
                let time = match input.next().unwrap() {
                    // Just realized that Token::Int is not an integer.
                    // I might be slightly stupid..?
                    Token::Int(int) => std::time::Duration::from_secs_f32(int),
                    any => panic!("Err unexpected tok: {:?}", any),
                };
                std::thread::sleep(time)
            }
            "loop" => match input.next().unwrap() {
                Token::Scope(scope) => loop {
                    interpret(scope.clone());
                    std::thread::sleep(std::time::Duration::from_millis(20))
                },
                Token::Int(num) => {
                    let scope = match input.next().unwrap() {
                        Token::Scope(scope) => scope,
                        any => panic!("Err unexpected tok: {:?}", any),
                    };
                    for _ in 0..(num as i32) {
                        interpret(scope.clone())
                    }
                }
                any => panic!("Err unexpected tok: {:?}", any),
            },
            unknown => panic!("Unknown function called: {:?}", unknown),
        }
    }
}
