use crate::evaluate::Evaluatee;

type StdLibFunction = dyn Fn(&[Evaluatee]) -> Evaluatee;

// Would love to have made this a const HashMap. Couldn't figure it out
pub fn functions(name: &str) -> Box<StdLibFunction> {
    match name {
        "add" => add(),
        "subtract" => subtract(),
        "multiply" => multiply(),
        "divide" => divide(),
        // TODO: Not a good solution for an unmatched function
        _ => add()
    } 
}

pub fn constants(name: &str) -> f64 {
    match name {
        "pi" => pi(),
        // TODO: Not a good solution for an unmatched constant
        _ => 0.0
    } 
}
// TODO: WOULD LIKE TO IMPLEMENT PROPER ERROR HANDLING RATHER THAN USING PANIC!

fn all(func: fn(Evaluatee, Evaluatee) -> Evaluatee) -> Box<StdLibFunction> {
    // It feels like there's a way to do this without copying the vec. I don't know enough rust
    // right now to do it.
    Box::new(move |arguments: &[Evaluatee]| {
        // If list is empty reduce will return None. I'd like to convert this to an error so that
        // the user can be notified there weren't any args supplied to the function 
        // Maybe add custom errors
        arguments.iter().cloned().reduce(|a, b| func(a, b)).unwrap()
    })
}

fn add() -> Box<StdLibFunction> {
    let func = |a: Evaluatee, b: Evaluatee| -> Evaluatee {
        return match (a, b) {
            (Evaluatee::Number(a), Evaluatee::Number(b)) => Evaluatee::Number(a + b),
            _ => panic!("add function received an input that was not a number")
        }
    };
    
    all(func)
}

fn subtract() -> Box<StdLibFunction> {
    let func = |a: Evaluatee, b: Evaluatee| -> Evaluatee {
        return match (a, b) {
            (Evaluatee::Number(a), Evaluatee::Number(b)) => Evaluatee::Number(a - b),
            _ => panic!("subtract received an input that was not a number")
        }
    };

    all(func)
}

fn multiply() -> Box<StdLibFunction> {
    let func = |a: Evaluatee, b: Evaluatee| -> Evaluatee {
        return match (a, b) {
            (Evaluatee::Number(a), Evaluatee::Number(b)) => Evaluatee::Number(a * b),
            _ => panic!("multiply received an input that was not a number")
        }
    };

    all(func)
}

fn divide() -> Box<StdLibFunction> {
    let func = |a: Evaluatee, b: Evaluatee| -> Evaluatee {
        return match (a, b) {
            (Evaluatee::Number(a), Evaluatee::Number(b)) => Evaluatee::Number(a / b),
            _ => panic!("divide received an input that was not a number")
        }
    };

    all(func)
}

fn modulo() -> Box<StdLibFunction> { 
    let func = |a: Evaluatee, b: Evaluatee| -> Evaluatee {
        return match (a, b) {
            (Evaluatee::Number(a), Evaluatee::Number(b)) => Evaluatee::Number(a % b),
            _ => panic!("modulo received an input that was not a number")
        }
    };

    all(func)
}

fn pi() -> f64 {
    std::f64::consts::PI
}
//
// fn log() -> Box<StdLibFunction> {
//     |_literal: &str| {
//         // Write::write_fmt(std::fmt::format) 
//         println!("Fix this later")
//     }
//
//     let func = |a: Evaluatee| -> Evaluatee {
//       println!("")  
//     };
// } 
