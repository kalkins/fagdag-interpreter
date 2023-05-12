mod scope;
mod function;
mod value;
mod expression;

#[cfg(test)]
mod test;

use crate::parser::ast::Program;
use self::function::run_function;
use self::scope::Scope;
use self::value::Value;

pub fn run(program: &Program) -> Result<i32, String> {
    let mut scope = Scope::new();

    for function in &program.nodes {
        scope.add_function(function)
    }

    if let Some(main) = scope.get_function("main") {
        match run_function(&main, &scope, vec![]) {
            Ok(Some(Value::Int(return_code))) => Ok(return_code),
            Ok(Some(value)) => Err(format!("Illegal non-integer return value from main: {value}")),
            Ok(None) => Err("Expected integer return value from main".into()),
            Err(error) => Err(error),
        }
    } else {
        Err("No main function".into())
    }
}
