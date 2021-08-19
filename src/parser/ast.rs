use crate::lexer::token::TokenType;

pub type ModuleAst = Vec<Expr>;
pub type BlockDecl = Vec<Expr>;

#[derive(PartialEq, Debug)]
pub enum Expr {
    Grouping { expr: Box<Expr> },
    Binary { left: Box<Expr>, op: BinaryOperator, right: Box<Expr> },
    Unary { op: UnaryOperator, expr: Box<Expr> },
    LetAssign { ident: Identifier, initializer: Box<Expr> }, // TODO: Make initializer Option.
    LetGet { ident: Identifier },
    LetSet { ident: Identifier, expr: Box<Expr> },
    Fun { ident: Identifier, decl: FunDecl },
    Block { block: BlockDecl },
    Print { expr: Box<Expr> },
    Literal(LiteralExpr),
}

impl Expr {
    pub fn grouping(expr: Expr) -> Expr {
        Expr::Grouping { expr: Box::new(expr) }
    }

    pub fn binary(left: Expr, op: BinaryOperator, right: Expr) -> Expr {
        Expr::Binary { left: Box::new(left), op, right: Box::new(right) }
    }

    pub fn unary(op: UnaryOperator, expr: Expr) -> Expr {
        Expr::Unary { op, expr: Box::new(expr) }
    }

    pub fn let_assign(ident: Identifier, initializer: Expr) -> Expr {
        Expr::LetAssign { ident, initializer: Box::new(initializer) }
    }

    pub fn let_get(ident: Identifier) -> Expr {
        Expr::LetGet { ident }
    }

    pub fn let_set(ident: Identifier, expr: Expr) -> Expr {
        Expr::LetSet { ident, expr: Box::new(expr) }
    }

    pub fn fun(ident: Identifier, decl: FunDecl) -> Expr {
        Expr::Fun { ident, decl }
    }

    pub fn block(block: BlockDecl) -> Expr {
        Expr::Block { block }
    }

    pub fn print(expr: Expr) -> Expr {
        Expr::Print { expr: Box::new(expr) }
    }
}

pub type Identifier = String;

#[derive(PartialEq, Debug)]
pub enum LiteralExpr {
    Number(f64),
    Nil,
}

#[derive(PartialEq, Debug)]
pub enum BinaryOperator {
    Equal,
    BangEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    Subtract,
    Add,
    Divide,
    Multiply,
}

impl BinaryOperator {
    pub fn from_token(token_type: &TokenType) -> Option<BinaryOperator> {
        let op = match token_type {
            TokenType::Minus => BinaryOperator::Subtract,
            TokenType::Plus => BinaryOperator::Add,
            TokenType::Star => BinaryOperator::Multiply,
            TokenType::Slash => BinaryOperator::Divide,
            TokenType::BangEqual => BinaryOperator::BangEqual,
            TokenType::Equal => BinaryOperator::Equal,
            TokenType::EqualEqual => BinaryOperator::Equal,
            TokenType::LessThan => BinaryOperator::LessThan,
            TokenType::LessThanEqual => BinaryOperator::LessThanEqual,
            TokenType::GreaterThan => BinaryOperator::GreaterThan,
            TokenType::GreaterThanEqual => BinaryOperator::GreaterThanEqual,
            _ => return None,
        };

        Some(op)
    }
}

#[derive(PartialEq, Debug)]
pub enum UnaryOperator {
    Negate,
    Not,
}

impl UnaryOperator {
    pub fn from_token(token_type: &TokenType) -> Option<UnaryOperator> {
        Some(match token_type {
            TokenType::Minus => UnaryOperator::Negate,
            TokenType::Bang => UnaryOperator::Not,
            _ => return None,
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct FunDecl {
    params: Vec<Identifier>,
    body: BlockDecl,
}

impl FunDecl {
    pub fn new(params: Vec<Identifier>, body: BlockDecl) -> Self {
        FunDecl { params, body }
    }

    pub fn body(&self) -> &BlockDecl {
        &self.body
    }
}
