use std::{fmt::Display, collections::HashMap};

use crate::{parse::TreeElement, standard_library};

fn apply(name: &str,  arguments: &Vec<TreeElement>) -> Result<Evaluatee, String> {
    let function = standard_library::functions(name);
    let arguments: Vec<Result<Evaluatee, String>> = arguments.into_iter().map(evaluate).collect();
    if !arguments.iter().all(Result::is_ok) {
        // TODO: This should probably return all the failing evaluatees
        return Err("An evaluatee could not be parsed".to_string())
    }
    
    let arguments: Vec<Evaluatee> = arguments.into_iter().map(|arg| arg.unwrap()).collect();
    return Ok(function(&arguments))
}

fn evaluate(node: &TreeElement) -> Result<Evaluatee, String> {
    match node {
        TreeElement::StringLiteral(value) => return Ok(Evaluatee::String(value.to_string())),
        TreeElement::Empty => return Err("Given element was empty".to_string()),
        TreeElement::NumericLiteral(value) => return Ok(Evaluatee::Number(*value)),
        TreeElement::CallExpression { name, arguments } => apply(name, arguments),
        TreeElement::Identifier(name) => return Ok(Evaluatee::Number(standard_library::constants(name)))
     }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Evaluatee {
    Number(f64),
    String(String)
}


#[test]
fn should_fall_back_to_returning_a_primitive_numeric() {
    let input = TreeElement::numeric_literal(2);
    
    assert_eq!(evaluate(&input), Ok(Evaluatee::Number(2.0)))
}

#[test]
fn should_fall_back_to_returning_a_primitive_string() {
    let input = TreeElement::string_literal("Hello");

    assert_eq!(evaluate(&input), Ok(Evaluatee::String("Hello".to_string())))
}

#[test]
fn should_be_able_to_evaluate_a_single_expression() {
    let input = TreeElement::call_expression("add", vec!(
        TreeElement::numeric_literal(2),
        TreeElement::numeric_literal(3)
    ));

    assert_eq!(evaluate(&input), Ok(Evaluatee::Number(5.0)))
}

#[test]
fn should_be_able_to_evaluate_a_nested_expression() {
    let input = TreeElement::call_expression("add", vec!(
        TreeElement::numeric_literal(2),
        TreeElement::numeric_literal(3),
        TreeElement::call_expression("subtract", vec!(
            TreeElement::numeric_literal(5),
            TreeElement::numeric_literal(4)
        ))
    ));

    assert_eq!(evaluate(&input), Ok(Evaluatee::Number(6.0)))
}

#[test]
fn should_be_able_to_lookup_identifiers_in_the_environment() {
    let input = TreeElement::identifier("pi");

    assert_eq!(evaluate(&input), Ok(Evaluatee::Number(std::f64::consts::PI)))
}

#[test]
fn should_be_able_to_get_the_highest_number_in_a_range() {
    let input = TreeElement::call_expression("max", vec!(
        TreeElement::numeric_literal(2),
        TreeElement::numeric_literal(3),
        TreeElement::numeric_literal(10)
    ));

    assert_eq!(evaluate(&input), Ok(Evaluatee::Number(10.0)))
}
