use regex::Regex;

const OPERATORS: [char; 5] = [ '+', '-', '*', '/', '%' ];

pub fn is_letter(c: &char) -> bool {
    let letter: Regex = Regex::new(r"[a-zA-Z]").unwrap();

    letter.is_match(&c.to_string())
}

pub fn is_whitespace(c: &char) -> bool {
    let whitespace: Regex = Regex::new(r"\s+").unwrap();
    
    whitespace.is_match(&c.to_string())
}

pub fn is_number(c: &char) -> bool {
    let number: Regex = Regex::new(r"^[0-9]+$").unwrap();

    number.is_match(&c.to_string())
}

pub fn is_opening_parenthesis(c: &char) -> bool {
    c.eq(&'(')
}

pub fn is_closing_parenthesis(c: &char) -> bool {
    c.eq(&')')
}

pub fn is_parenthesis(c: &char) -> bool {
   is_opening_parenthesis(c) || is_closing_parenthesis(c) 
}

pub fn is_quote(c: &char) -> bool {
    c.eq(&'"')
}

pub fn is_operator(c: &char) -> bool {
    OPERATORS.contains(c)
}
