#[derive(Debug, Clone)]
pub enum Token {
    Scope(Vec<Token>),
    Id(String),
    Str(String),
    Int(f32),
    ActionTilde,
    VarToken,
}

pub fn tokenize(input: String) -> Vec<Token> {
    let mut out: Vec<Token> = vec![];

    let mut input = input.chars().peekable();

    while let Some(chr) = input.next() {
        match chr {
            chr if chr.is_alphabetic() => {
                let mut str_out = String::from(chr);

                while let Some(alpha) = input.peek() {
                    if alpha.is_alphanumeric() || *alpha == '-' {
                        str_out.push(input.next().unwrap())
                    } else {
                        break;
                    }
                }
                out.push(Token::Id(str_out))
            }
            chr if chr.is_numeric() => {
                let mut int_out = String::from(chr);

                while let Some(int) = input.peek() {
                    if int.is_numeric() || *int == '.' {
                        int_out.push(input.next().unwrap())
                    } else {
                        break;
                    }
                }
                out.push(Token::Int(int_out.parse::<f32>().unwrap()));
            }
            '#' => while Some('\n') != input.next() {},
            '[' => {
                let mut to_parse = String::new();
                while let Some(chr) = input.next() {
                    if chr == ']' {
                        break;
                    }
                    to_parse.push(chr)
                }
                out.push(Token::Scope(tokenize(to_parse)));
            }
            '~' => {
                out.push(Token::ActionTilde);
            }
            '$' => out.push(Token::VarToken),
            '"' => {
                let mut text = String::new();
                while let Some(chr) = input.next() {
                    if chr == '"' {
                        break;
                    }
                    if chr == '\\' {
                        text.push(input.next().unwrap())
                    }
                    text.push(chr)
                }
                out.push(Token::Str(text))
            }
            any if any.is_whitespace() => {}
            unexpected => panic!("Unexpected char: {}", unexpected),
        }
    }

    out
}
