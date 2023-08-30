use std::collections::HashMap;
use crate::parser::ast::FunctionNode;
use crate::vm::function::run_function;
use super::value::Value;

struct ScopeLayer {
    variables: HashMap<String, Value>,
}

impl ScopeLayer {
    fn new() -> Self {
        ScopeLayer {
            variables: HashMap::new(),
        }
    }
}

pub struct Scope {
    layers: Vec<ScopeLayer>,
    functions: HashMap<String, FunctionNode>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            layers: vec![ScopeLayer::new()],
            functions: HashMap::new(),
        }
    }

    fn push_layer(&mut self) {
        self.layers.push(ScopeLayer::new())
    }

    fn pop_layer(&mut self) {
        if self.layers.len() > 1 {
            self.layers.pop();
        } else {
            panic!("Tried to remove last layer of scope. This should not be possible.")
        }
    }

    pub fn subscope<T, F>(&mut self, f: F) -> T
        where F: FnOnce(&mut Scope) -> T
    {
        self.push_layer();
        let result = f(self);
        self.pop_layer();
        result
    }

    pub fn add_variable(&mut self, name: impl ToString, value: Value) {
        self.layers.last_mut().expect("Scope has no layers").variables.insert(name.to_string(), value);
    }

    pub fn set_variable(&mut self, name: impl ToString, value: Value) -> Result<(), String> {
        let key = name.to_string();

        self.layers
            .iter_mut()
            .rev()
            .find_map(|layer| layer.variables.get_mut(&key))
            .map(|v| *v = value)
            .ok_or(format!("Could not find variable {key}"))
    }

    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.layers
            .iter()
            .rev()
            .find_map(|layer| layer.variables.get(name))
    }

    pub fn clone_variable(&self, name: &str) -> Option<Value> {
        self.get_variable(name).map(Clone::clone)
    }

    pub fn add_function(&mut self, function: &FunctionNode) {
        self.functions.insert(function.name.clone(), function.clone());
    }

    pub fn call_function(&mut self, name: &str, args: Vec<Value>) -> Result<Option<Value>, String> {
        self.subscope(|scope| {
            scope.functions
                .get(name)
                .cloned()
                .ok_or(format!("No function '{name}'"))
                .and_then(|f| {
                    run_function(&f, scope, args)
                })
        })
    }
}

#[cfg(test)]
mod test {
    use crate::vm::scope::Scope;
    use crate::vm::value::Value;

    #[test]
    fn test_single_scope() {
        let mut scope = Scope::new();

        scope.add_variable("x", Value::Int(5));

        assert_eq!(
            scope.get_variable("x").expect("Could not find x"),
            &Value::Int(5),
        );

        scope.add_variable("y", Value::Bool(true));

        assert_eq!(
            scope.get_variable("y").expect("Could not find y"),
            &Value::Bool(true),
        );

        assert_eq!(
            scope.get_variable("x").expect("Could not find x"),
            &Value::Int(5),
        );

        scope.add_variable("x", Value::Int(0));
        assert_eq!(
            scope.get_variable("x").expect("Could not find x"),
            &Value::Int(0),
        );
    }

    #[test]
    fn test_read_parent() {
        let mut parent = Scope::new();
        parent.add_variable("x", Value::Int(5));

        parent.subscope(|s| {
            s.add_variable("y", Value::Int(0));

            assert_eq!(
                s.get_variable("x").expect("Could not find x in child"),
                &Value::Int(5),
            );

            assert_eq!(
                s.get_variable("y").expect("Could not find y in child"),
                &Value::Int(0),
            );
        });

        assert_eq!(
            parent.get_variable("x").expect("Could not find x in parent"),
            &Value::Int(5),
        );

        assert_eq!(
            parent.get_variable("y"),
            None,
        );
    }

    #[test]
    fn test_write_parent() {
        let mut parent = Scope::new();
        parent.add_variable("x", Value::Int(5));

        parent.subscope(|s| {
            s.set_variable("x", Value::Int(0)).unwrap();

            assert_eq!(
                s.get_variable("x").expect("Could not find x in child"),
                &Value::Int(0),
            );
        });

        assert_eq!(
            parent.get_variable("x").expect("Could not find x in parent"),
            &Value::Int(0),
        );
    }
}
