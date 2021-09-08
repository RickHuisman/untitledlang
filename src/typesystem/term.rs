enum Term {
    Let { name: String, rhs: Term, body: Term},
    Apply { fun: Term, arg: Term },
    Lambda { var_name: String, body: Term },
    Variable { name: String },
    Literal(Literal),
}

enum Literal {
    Int,
    Bool,
}