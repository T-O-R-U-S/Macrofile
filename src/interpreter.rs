use crate::tokenizer::Token;
use std::collections::HashMap;
use enigo::*;

pub fn interpret(input: Vec<Token>) {
	let mut input = input.into_iter();

	let mut actions: HashMap<String, Vec<Token>> = HashMap::new();

	let mut keyboard = enigo::Enigo::new();

	while let Some(token) = input.next() {
		println!("{:?}", token);
		let action: bool;
		let name = match token {
			Token::Id(id) => {
				action = false;
				id
			},
			Token::ActionTilde => {
				action = true;
				match input.next() {
					Some(Token::Id(id)) => id,
					any => panic!("Unexpected action: {:?}", any)
				}
			}
			_ => panic!("Unexpected tok: {:?}", token)
		};

		if action {
			interpret(actions
				.get(&name)
				.unwrap()
				.clone()
			);
			continue;
		}

		match name.as_str() {
			"inp" => {
				match input.next().unwrap() {
					Token::Str(input) => {
						keyboard.key_sequence(&input)
					}
					any => panic!("Unexpected tok: {:?}", any)
				}
			}
			"action" => {
				let name = match input.next().unwrap() {
					Token::Id(id) => id,
					any => panic!("Unexpected tok: {:?}", any)
				};
				let scope = match input.next().unwrap() {
					Token::Scope(scope) => scope,
					any => panic!("Unexpected tok: {:?}", any)
				};
				actions.insert(name, scope);
			}
			unknown => panic!("Unknown function called: {:?}", unknown)
		}
	}
}