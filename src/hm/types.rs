use std::collections::HashMap;

pub type ArenaType = usize;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Variable {
        id: ArenaType,
        instance: Option<ArenaType>,
    },
    Operator {
        id: ArenaType,
        name: String,
        types: Vec<ArenaType>,
    },
}

pub struct TypeVarGenerator {
    pub value: char,
    pub set: HashMap<ArenaType, String>,
}

impl TypeVarGenerator {
    pub fn new() -> TypeVarGenerator {
        TypeVarGenerator { value: 'a', set: HashMap::new() }
    }

    fn next(&mut self) -> String {
        let v = self.value;
        self.value = ((self.value as u8) + 1) as char;
        format!("{}", v)
    }

    fn name(&mut self, t: ArenaType) -> String {
        let k = {
            self.set.get(&t).map(|x| x.clone())
        };
        if let Some(val) = k {
            val.clone()
        } else {
            let v = self.next();
            self.set.insert(t, v.clone());
            v
        }
    }
}

impl Type {
    pub fn new_variable(idx: ArenaType) -> Type {
        Type::Variable {
            id: idx,
            instance: None,
        }
    }

    pub fn new_operator(idx: ArenaType, name: &str, types: &[ArenaType]) -> Type {
        Type::Operator {
            id: idx,
            name: name.to_string(),
            types: types.to_vec(),
        }
    }

    pub fn id(&self) -> usize {
        match self {
            &Type::Variable { id, .. } => { id }
            &Type::Operator { id, .. } => { id }
        }
    }

    pub fn set_instance(&mut self, instance: ArenaType) {
        match self {
            &mut Type::Variable { instance: ref mut inst, .. } => {
                *inst = Some(instance);
            }
            _ => unimplemented!(), // TODO:
        }
    }

    pub fn as_string(&self, a: &Vec<Type>, namer: &mut TypeVarGenerator) -> String {
        match self {
            &Type::Variable { instance: Some(inst), .. } => {
                a[inst].as_string(a, namer)
            }
            &Type::Variable { .. } => {
                namer.name(self.id())
            }
            &Type::Operator { ref types, ref name, .. } => {
                match types.len() {
                    0 => name.clone(),
                    2 => {
                        let l = a[types[0]].as_string(a, namer);
                        let r = a[types[1]].as_string(a, namer);
                        format!("({} {} {})", l, name, r)
                    }
                    _ => {
                        let mut coll = vec![];
                        for v in types {
                            coll.push(a[*v].as_string(a, namer));
                        }
                        format!("{} {}", name, coll.join(" "))
                    }
                }
            }
        }
    }
}
