use crate::tokenize::Token;

use std::{
    iter::Peekable,
    slice::Iter,
};

#[derive(PartialEq)]
pub enum TreeElement {
    NumericLiteral(u32),
    StringLiteral(String),
    Identifier(String),
    CallExpression { name: String, arguments: Vec<TreeElement> },
    Empty
}

#[derive(Clone,Debug)]
enum Item<T> {
    Collection(Vec<Item<T>>),
    Value(T)
}

impl TreeElement {
    pub fn numeric_literal(value: impl Into<u32>) -> Self {
        Self::NumericLiteral(value.into())
    }

    pub fn string_literal(value: impl Into<String>) -> TreeElement {
        Self::StringLiteral(value.into())
    }

    pub fn identifier(value: impl Into<String>) -> TreeElement {
        Self::Identifier(value.into())
    }

    pub fn call_expression(name: impl Into<String>, arguments: Vec<TreeElement>) -> TreeElement {
        Self::CallExpression { name: name.into(), arguments }
    }
}

fn parse(tokens: Vec<Token>) -> Option<TreeElement> {
    let mut iter = tokens.iter().peekable();
    let parens = parenthesize(&mut iter)?;
    
    return classify(&parens)
}

fn parenthesize(tokens: &mut Peekable<Iter<Token>>) -> Option<Item<Token>> {
    let token = match tokens.next() {
        None => return None,
        Some(value) => value
    };

    match token {
        Token::Parenthesis(token) if token.eq(&'(') => {
            let mut expression: Item<Token> = Item::Collection(vec!());

            loop {
                let next_token = match tokens.peek() {
                    Some(next_token) => next_token,
                    None => return Some(expression)
                };

                match next_token {
                    Token::Parenthesis(next_token) if next_token.eq(&')') => break,
                    _ => {
                        let parens = match parenthesize(tokens) {
                            None => return Some(expression),
                            Some(parens) => parens
                        };
                        expression = match expression {
                            Item::Collection(expression_values) => {
                                let mut new_expression_values = expression_values.clone();
                                new_expression_values.push(parens);
                                Item::Collection(new_expression_values)
                            },
                            _ => { println!("Somehow expression was not a Collection"); break; }
                        }
                    }
                }
            }

            // Due to our loop conditions this next value is a closing parenthesis which we don't
            // care about
            tokens.next();

            return Some(expression)
        },
        // Feels like there should be a better option than cloning here
        _ => Some(Item::Value(token.clone()))
    }
}

fn classify(tokens: &Item<Token>) -> Option<TreeElement> {
    return match tokens {
        Item::Value(token) => match token {
            Token::Number(value) => Some(TreeElement::numeric_literal(*value)),
            Token::Name(value) => Some(TreeElement::identifier(value)),
            Token::String(value) => Some(TreeElement::string_literal(value)),
            _ => None
        },
        Item::Collection(tokens) => {
            let mut tokens = tokens.iter();
            let first_token = match tokens.next() {
                Some(Item::Value(Token::Name(token))) => token,
                _ => return None
            };

            return Some(TreeElement::call_expression(first_token, tokens.filter_map(classify).collect()))
        }
    }
}


#[test]
fn should_return_element_of_type_numeric_literal_for_number() {
    let value = 2 as u32;
    let input: Vec<Token>  = vec!(
        Token::number(value)
    );
    let result = TreeElement::numeric_literal(value);

    assert!(parse(input).unwrap().eq(&result))
}

#[test]
fn should_return_element_of_type_string_literal_for_string() {
    let value = "hello";
    let input: Vec<Token> = vec!(
        Token::string(value)
    );
    let result = TreeElement::string_literal(value);

    assert!(parse(input).unwrap().eq(&result))
}

#[test]
fn should_return_element_of_type_identifier_for_names() {
    let value = 'x';
    let input: Vec<Token> = vec!(
        Token::name(value)
    );
    let result = TreeElement::identifier(value);

    assert!(parse(input).unwrap().eq(&result))
}

#[test]
fn should_return_an_ast_for_a_basic_data_structure() {
    let input: Vec<Token> = vec!(
        Token::parenthesis('('),
        Token::name("add"),
        Token::number(2 as u32),
        Token::number(3 as u32),
        Token::parenthesis(')')
    );
    let result = TreeElement::call_expression("add", vec!(
            TreeElement::numeric_literal(2 as u32),
            TreeElement::numeric_literal(3 as u32)
    ));

    assert!(parse(input).unwrap().eq(&result))
}

#[test]
fn should_return_an_ast_for_a_nested_data_structure () {
    let input: Vec<Token> = vec!(
        Token::parenthesis('('),
        Token::name("add"),
        Token::number(2 as u32),
        Token::number(3 as u32),
        Token::parenthesis('('),
        Token::name("subtract"),
        Token::number(4 as u32),
        Token::number(2 as u32),
        Token::parenthesis(')'),
        Token::parenthesis(')')
    );
    let result = TreeElement::call_expression("add", vec!(
        TreeElement::numeric_literal(2 as u32),
        TreeElement::numeric_literal(3 as u32),
        TreeElement::call_expression("subtract", vec!(
            TreeElement::numeric_literal(4 as u32),
            TreeElement::numeric_literal(2 as u32)
        ))
    ));

    assert!(parse(input).unwrap().eq(&result))
}
