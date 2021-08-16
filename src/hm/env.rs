use std::collections::HashMap;
use crate::hm::types::*;

#[derive(Clone, Debug)]
pub struct Env(pub HashMap<String, ArenaType>);

impl Env {
    pub fn new() -> (Vec<Type>, Env) {
        let mut types = vec![
            // Integer
            Type::new_operator(0, "int", &[]),
            // Basic bool
            Type::new_operator(1, "bool", &[]),
        ];

        let var1 = new_variable(&mut types);
        let var2 = new_variable(&mut types);
        let pair_type = new_operator(&mut types, "*", &[var1, var2]);

        let var3 = new_variable(&mut types);

        let mut map = HashMap::new();
        map.insert("pair".to_string(), {
            let right = new_function(&mut types, var2, pair_type);
            new_function(&mut types, var1, right)
        });
        map.insert("true".to_string(), 1);
        map.insert("false".to_string(), 1);
        map.insert("cond".to_string(), {
            let right = new_function(&mut types, var3, var3);
            let right = new_function(&mut types, var3, right);
            new_function(&mut types, 1, right)
        });

        map.insert("zero".to_string(), new_function(&mut types, 0, 1));
        map.insert("pred".to_string(), new_function(&mut types, 0, 0));
        map.insert("times".to_string(), {
            let right = new_function(&mut types, 0, 0);
            new_function(&mut types, 0, right)
        });

        (types, Env(map))
    }
}

/// A binary type constructor which builds function types
pub fn new_function(a: &mut Vec<Type>, from_type: ArenaType, to_type: ArenaType) -> ArenaType {
    let t = Type::new_operator(a.len(), "->", &[from_type, to_type]);
    a.push(t);
    a.len() - 1
}

/// A binary type constructor which builds function types
pub fn new_variable(a: &mut Vec<Type>) -> ArenaType {
    let t = Type::new_variable(a.len());
    a.push(t);
    a.len() - 1
}

/// A binary type constructor which builds function types
pub fn new_operator(a: &mut Vec<Type>, name: &str, types: &[ArenaType]) -> ArenaType {
    let t = Type::new_operator(a.len(), name, types);
    a.push(t);
    a.len() - 1
}
