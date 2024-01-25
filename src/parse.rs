use crate::tokenize::Token;

#[derive(PartialEq)]
pub enum TreeElement {
    NumericLiteral(u32),
    StringLiteral(String),
    Identifier(String),
    CallExpression { name: String, arguments: Vec<TreeElement> },
    Empty
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

fn parenthesize() {

}

fn parse(tokens: Vec<Token>) -> Option<TreeElement> {
    let mut iter = tokens.iter().peekable();
    let first_token = iter.next();

    if first_token.is_none() {
        return None
    }

    return match first_token.unwrap() {
        Token::Number(value) => Some(TreeElement::numeric_literal(*value)),
        Token::Name(value) => Some(TreeElement::identifier(value)),
        Token::String(value) => Some(TreeElement::string_literal(value)),
        _ => None
    };

    // TODO: Remove this unwrap when this is further implemented
    let token = iter.next().unwrap();
    match token {
        Token::Number(value) => Some(TreeElement::numeric_literal(*value)),
        Token::Name(value) => Some(TreeElement::identifier(value)),
        Token::String(value) => Some(TreeElement::string_literal(value)),
        _ => None
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
        Token::parenthesis(')'),
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
