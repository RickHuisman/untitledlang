use crate::hm::syntax::Syntax;
use crate::hm::syntax::Syntax::*;
use std::collections::{HashSet, HashMap};
use crate::hm::env::*;
use crate::hm::types::*;

pub fn analyse(
    node: &Syntax,
    arena: &mut Vec<Type>,
    env: &Env,
    non_generic: &HashSet<ArenaType>,
) -> ArenaType {
    match node {
        &Identifier { ref name } => {
            get_type(arena, name, env, non_generic)
        }
        &Apply { ref func, ref arg } => {
            let fun_type = analyse(func, arena, env, non_generic);
            let arg_type = analyse(arg, arena, env, non_generic);
            let result_type = new_variable(arena);
            let first = new_function(arena, arg_type, result_type.clone());
            unify(arena, first, fun_type);
            result_type
        }
        &Lambda { ref v, ref body } => {
            let arg_type = new_variable(arena);
            let mut new_env = env.clone();
            new_env.0.insert(v.clone(), arg_type);
            let mut new_non_generic = non_generic.clone();
            new_non_generic.insert(arg_type.clone());
            let result_type = analyse(body, arena, &new_env, &new_non_generic);
            new_function(arena, arg_type, result_type)
        }
        &Let { ref defn, ref v, ref body } => {
            let defn_type = analyse(defn, arena, env, non_generic);
            let mut new_env = env.clone();
            new_env.0.insert(v.clone(), defn_type);
            analyse(body, arena, &new_env, non_generic)
        }
        &Letrec { ref defn, ref v, ref body } => {
            let new_type = new_variable(arena);
            let mut new_env = env.clone();
            new_env.0.insert(v.clone(), new_type.clone());
            let mut new_non_generic = non_generic.clone();
            new_non_generic.insert(new_type.clone());
            let defn_type = analyse(defn, arena, &new_env, &new_non_generic);
            unify(arena, new_type, defn_type);
            analyse(body, arena, &new_env, non_generic)
        }
    }
}

fn get_type(a: &mut Vec<Type>, name: &str, env: &Env, non_generic: &HashSet<ArenaType>) -> ArenaType {
    if let Some(value) = env.0.get(name) {
        let mat = non_generic.iter().cloned().collect::<Vec<_>>();
        fresh(a, *value, &mat)
    } else if is_integer_literal(name) {
        0 //INTEGER.id
    } else {
        //raise ParseError("Undefined symbol {0}".format(name))
        panic!("Undefined symbol {:?}", name);
    }
}

fn fresh(a: &mut Vec<Type>, t: ArenaType, non_generic: &[ArenaType]) -> ArenaType {
    // A mapping of TypeVariables to TypeVariables
    let mut mappings = HashMap::new();

    fn freshrec(a: &mut Vec<Type>, tp: ArenaType, mappings: &mut HashMap<ArenaType, ArenaType>, non_generic: &[ArenaType]) -> ArenaType {
        let p = prune(a, tp);
        match a.get(p).unwrap().clone() {
            Type::Variable { .. } => {
                if is_generic(a, p, non_generic) {
                    mappings.entry(p)
                        .or_insert(new_variable(a))
                        .clone()
                } else {
                    p
                }
            }
            Type::Operator { ref name, ref types, .. } => {
                let b = types.iter().map(|x| freshrec(a, *x, mappings, non_generic)).collect::<Vec<_>>();
                new_operator(a, name, &b)
            }
        }
    }

    freshrec(a, t, &mut mappings, non_generic)
}

fn unify(alloc: &mut Vec<Type>, t1: ArenaType, t2: ArenaType) {
    let a = prune(alloc, t1);
    let b = prune(alloc, t2);
    match (alloc.get(a).unwrap().clone(), alloc.get(b).unwrap().clone()) {
        (Type::Variable { .. }, _) => {
            if a != b {
                if occurs_in_type(alloc, a, b) {
                    // raise InferenceError("recursive unification")
                    panic!("recursive unification");
                }
                alloc.get_mut(a).unwrap().set_instance(b);
            }
        }
        (Type::Operator { .. }, Type::Variable { .. }) => {
            unify(alloc, b, a)
        }
        (Type::Operator { name: ref a_name, types: ref a_types, .. },
            Type::Operator { name: ref b_name, types: ref b_types, .. }) => {
            if a_name != b_name || a_types.len() != b_types.len() {
                //raise InferenceError("Type mismatch: {0} != {1}".format(str(a), str(b)))
                panic!("type mismatch");
            }
            for (p, q) in a_types.iter().zip(b_types.iter()) {
                unify(alloc, *p, *q);
            }
        }
    }
}

fn prune(a: &mut Vec<Type>, t: ArenaType) -> ArenaType {
    let v2 = match a.get(t).unwrap() {
        //TODO screwed up
        &Type::Variable { instance, .. } => {
            if let Some(value) = instance {
                value
            } else {
                return t;
            }
        }
        _ => {
            return t;
        }
    };

    let value = prune(a, v2);
    match a.get_mut(t).unwrap() {
        //TODO screwed up
        &mut Type::Variable { ref mut instance, .. } => {
            *instance = Some(value);
        }
        _ => {
            return t;
        }
    }
    value
}

fn is_generic(a: &mut Vec<Type>, v: ArenaType, non_generic: &[ArenaType]) -> bool {
    !occurs_in(a, v, non_generic)
}

fn occurs_in_type(a: &mut Vec<Type>, v: ArenaType, type2: ArenaType) -> bool {
    let pruned_type2 = prune(a, type2);
    if pruned_type2 == v {
        return true;
    }
    match a.get(pruned_type2).unwrap().clone() {
        Type::Operator { ref types, .. } => {
            occurs_in(a, v, types)
        }
        _ => false
    }
}

fn occurs_in(a: &mut Vec<Type>, t: ArenaType, types: &[ArenaType]) -> bool {
    for t2 in types.iter() {
        if occurs_in_type(a, t, *t2) {
            return true;
        }
    }
    return false;
}

fn is_integer_literal(name: &str) -> bool {
    name.parse::<isize>().is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hm::syntax::*;

    #[test]
    fn test_factorial() {
        let (mut a, my_env) = Env::new();

        // factorial
        let syntax = letrec("factorial",  // letrec factorial =
                            lambda("n",  // fn n =>
                                   apply(
                                       apply(  // cond (zero n) 1
                                               apply(ident("cond"),  // cond (zero n)
                                                     apply(ident("zero"), ident("n"))),
                                               ident("1")),
                                       apply(  // times n
                                               apply(ident("times"), ident("n")),
                                               apply(ident("factorial"),
                                                     apply(ident("pred"), ident("n"))),
                                       ),
                                   ),
                            ),  // in
                            apply(ident("factorial"), ident("5")),
        );

        let t = analyse(&syntax, &mut a, &my_env, &HashSet::new());
        assert_eq!(a[t].as_string(&a, &mut TypeVarGenerator {
            value: 'a',
            set: HashMap::new(),
        }), r#"int"#);
    }

    #[should_panic]
    #[test]
    fn test_mismatch() {
        let (mut a, my_env) = Env::new();

        // fn x => (pair(x(3) (x(true)))
        let syntax = lambda("x",
                            apply(
                                apply(ident("pair"),
                                      apply(ident("x"), ident("3"))),
                                apply(ident("x"), ident("true"))));

        let _ = analyse(&syntax, &mut a, &my_env, &HashSet::new());
    }

    #[should_panic]
    #[test]
    fn test_pair() {
        let (mut a, my_env) = Env::new();

        // pair(f(3), f(true))
        let syntax = apply(
            apply(ident("pair"), apply(ident("f"), ident("4"))),
            apply(ident("f"), ident("true")));

        let _ = analyse(&syntax, &mut a, &my_env, &HashSet::new());
    }

    #[test]
    fn test_mul() {
        let (mut a, my_env) = Env::new();

        let pair = apply(apply(ident("pair"),
                               apply(ident("f"),
                                     ident("4"))),
                         apply(ident("f"),
                               ident("true")));

        // let f = (fn x => x) in ((pair (f 4)) (f true))
        let syntax = let_("f", lambda("x", ident("x")), pair);

        let t = analyse(&syntax, &mut a, &my_env, &HashSet::new());
        assert_eq!(a[t].as_string(&a, &mut TypeVarGenerator {
            value: 'a',
            set: HashMap::new(),
        }), r#"(int * bool)"#);
    }

    #[should_panic]
    #[test]
    fn test_recursive() {
        let (mut a, my_env) = Env::new();

        // fn f => f f (fail)
        let syntax = lambda("f", apply(ident("f"), ident("f")));

        let t = analyse(&syntax, &mut a, &my_env, &HashSet::new());
        assert_eq!(a[t].as_string(&a, &mut TypeVarGenerator {
            value: 'a',
            set: HashMap::new(),
        }), r#"int"#);
    }

    #[test]
    fn test_int() {
        let (mut a, my_env) = Env::new();

        // let g = fn f => 5 in g g
        let syntax = let_("g",
                          lambda("f", ident("5")),
                          apply(ident("g"), ident("g")));

        let t = analyse(&syntax, &mut a, &my_env, &HashSet::new());
        assert_eq!(a[t].as_string(&a, &mut TypeVarGenerator {
            value: 'a',
            set: HashMap::new(),
        }), r#"int"#);
    }


    #[test]
    fn test_generic_nongeneric() {
        let (mut a, my_env) = Env::new();

        // example that demonstrates generic and non-generic variables:
        // fn g => let f = fn x => g in pair (f 3, f true)
        let syntax = lambda("g",
                            let_("f",
                                 lambda("x", ident("g")),
                                 apply(
                                     apply(ident("pair"),
                                           apply(ident("f"), ident("3")),
                                     ),
                                     apply(ident("f"), ident("true")))));

        let t = analyse(&syntax, &mut a, &my_env, &HashSet::new());
        assert_eq!(a[t].as_string(&a, &mut TypeVarGenerator {
            value: 'a',
            set: HashMap::new(),
        }), r#"(a -> (a * a))"#);
    }


    #[test]
    fn test_composition() {
        let (mut a, my_env) = Env::new();

        // Function composition
        // fn f (fn g (fn arg (f g arg)))
        let syntax = lambda("f", lambda("g", lambda("arg", apply(ident("g"), apply(ident("f"), ident("arg"))))));

        let t = analyse(&syntax, &mut a, &my_env, &HashSet::new());
        assert_eq!(a[t].as_string(&a, &mut TypeVarGenerator::new()),
                   r#"((a -> b) -> ((b -> c) -> (a -> c)))"#);
    }


    #[test]
    fn test_fun() {
        let (mut a, my_env) = Env::new();

        // Function composition
        // fn f (fn g (fn arg (f g arg)))
        let syntax = lambda("f", lambda("g", lambda("arg", apply(ident("g"), apply(ident("f"), ident("arg"))))));

        let t = analyse(&syntax, &mut a, &my_env, &HashSet::new());
        assert_eq!(a[t].as_string(&a, &mut TypeVarGenerator {
            value: 'a',
            set: HashMap::new(),
        }), r#"((a -> b) -> ((b -> c) -> (a -> c)))"#);
    }
}