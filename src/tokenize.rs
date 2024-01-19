use crate::identify;

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = vec![];

    let mut input_chars = input.chars().peekable();

    while let Some(character) = input_chars.next() {
        // let character = match input_chars.next() {
        //     Some(char) => char,
        //     None => return tokens
        // };
    
        if identify::is_parenthesis(&character) {
            tokens.push(Token::parenthesis(character));
            continue;
        }

        if identify::is_whitespace(&character) {
            continue;
        }
         
        if identify::is_number(&character) {
            let mut number = character.to_string();

            loop {
                let character = match input_chars.next_if(identify::is_number) {
                    Some(character) => { character },
                    None => break
                };
                number.push_str(&character.to_string());
            }

            let number = number.parse::<u32>().expect(&format!("Character {number} was identified as a number but failed to be parsed as such."));
            tokens.push(Token::number(number));
            continue;
        }

        if identify::is_quote(&character) {
            let mut string = match input_chars.next() {
                Some(character) => character.to_string(),
                None => return tokens
            };
            
            loop {
                let character = match input_chars.next() {
                    Some(character) if identify::is_quote(&character) => break,
                    Some(character) => character,
                    None => return tokens
                };
                
                string.push(character);
            }

            tokens.push(Token::string(string));
            continue;
        }

        if identify::is_letter(&character) {
            let mut name = character.to_string();

            loop {
                let character = match input_chars.next_if(identify::is_letter) {
                    Some(character) => character,
                    _ => break
                };

                name.push(character);
            }

            tokens.push(Token::name(name));
            continue;
        }
    }

    tokens
}

#[derive(Debug, PartialEq)]
enum Token {
    Parenthesis(char),
    Whitespace,
    Name(String),
    Number(u32),
    String(String)
}

impl Token {
    fn string(s: impl Into<String>) -> Self {
        Token::String(s.into())
    }

    fn name(s: impl Into<String>) -> Self {
        Token::Name(s.into())
    }

    fn parenthesis(s: impl Into<char>) -> Self {
        Token::Parenthesis(s.into())
    }

    fn number(s: impl Into<u32>) -> Self {
        Token::Number(s.into())
    }
}
// TODO: I feel like the parsing in the tokenize loop could be done with an into
// impl Into<Option<Token>> for char {
//     fn into(self) -> Option<Token> {
//         
//     }
// }

#[test]
fn should_tokenize_parentheses() {
    let input: &str = "()";
    let result: [Token; 2] = [
        Token::parenthesis('('),
        Token::parenthesis(')')
    ];

    assert!(tokenize(input).iter().eq(result.iter()))
}

#[test]
fn should_ignore_whitespace() {
    let input: &str = "              ";
    let result: [Token; 0] = [];

    assert!(tokenize(input).iter().eq(result.iter()))
}

#[test]
fn should_tokenize_single_digit() {
    let input: &str = "5";
    let result: [Token; 1] = [
        Token::number(5 as u32)
    ];

    assert!(tokenize(input).iter().eq(result.iter()))
}

#[test]
fn should_tokenize_single_numbers_in_expressions() {
    let input: &str = "(1 2)";
    let result: [Token; 4] = [
        Token::parenthesis('('),
        Token::number(1 as u32),
        Token::number(2 as u32),
        Token::parenthesis(')')
    ];

    assert!(tokenize(input).iter().eq(result.iter()))
}

#[test]
fn should_tokenize_single_letters_in_expressions() {
    let input: &str = "(a b)";
    let result: [Token; 4] = [
        Token::parenthesis('('),
        Token::name("a"),
        Token::name("b"),
        Token::parenthesis(')')
    ];

    assert!(tokenize(input).iter().eq(result.iter()))
}

#[test]
fn should_tokenize_multi_digit_numbers() {
    let input: &str = "(11 22)";
    let result: [Token; 4] = [
        Token::parenthesis('('),
        Token::number(11 as u32),
        Token::number(22 as u32),
        Token::parenthesis(')')
    ];

    assert!(tokenize(input).iter().eq(result.iter()))
}

#[test]
fn should_tokenize_a_simple_expression() {
    let input: &str = "(add 2 3)";
    let result: [Token; 5] = [
        Token::parenthesis('('),
        Token::name("add"),
        Token::number(2 as u8),
        Token::number(3 as u8),
        Token::parenthesis(')')
    ];

    assert!(tokenize(input).iter().eq(result.iter()))
}

#[test]
fn should_ignore_more_whitespace() {
    let input: &str = "   (add   2 3)";
    let result: [Token; 5] = [
        Token::parenthesis('('),
        Token::name("add"),
        Token::number(2 as u8),
        Token::number(3 as u8),
        Token::parenthesis(')')
    ];

    assert!(tokenize(input).iter().eq(result.iter()))
}

#[test]
fn should_tokenize_strings() {
    let input: &str = "(log \"hello\" \"world\")";
    let result: [Token; 5]= [
        Token::parenthesis('('),
        Token::name("log"),
        Token::string("hello"),
        Token::string("world"),
        Token::parenthesis(')')
    ];


    assert!(tokenize(input).iter().eq(result.iter()))
}
