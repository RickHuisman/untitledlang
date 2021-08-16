use crate::lexer::token::TokenType;

// TODO: Slice?
pub type ModuleAst = Vec<Expr>;

#[derive(PartialEq, Debug)]
pub enum Expr {
    Grouping(GroupingExpr),
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    LetAssign(LetAssignExpr),
    LetGet(LetGetExpr),
    LetSet(LetSetExpr),
    Block(BlockExpr),
    Literal(LiteralExpr),
}

#[derive(PartialEq, Debug)]
pub enum LiteralExpr {
    Number(f64),
    Nil,
}

#[derive(PartialEq, Debug)]
pub struct GroupingExpr {
    expr: Box<Expr>,
}

impl GroupingExpr {
    pub fn new(expr: Box<Expr>) -> Self {
        GroupingExpr { expr }
    }
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
pub struct BinaryExpr {
    left: Box<Expr>,
    op: BinaryOperator,
    right: Box<Expr>,
}

impl BinaryExpr {
    pub fn new(left: Box<Expr>, op: BinaryOperator, right: Box<Expr>) -> BinaryExpr {
        BinaryExpr { left, op, right }
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
pub struct UnaryExpr {
    operator: UnaryOperator,
    expr: Box<Expr>,
}

impl UnaryExpr {
    pub fn new(operator: UnaryOperator, expr: Box<Expr>) -> UnaryExpr {
        UnaryExpr { operator, expr }
    }
}

#[derive(PartialEq, Debug)]
pub struct LetAssignExpr {
    pub ident: Identifier,
    pub initializer: Box<Expr>,
}

impl LetAssignExpr {
    pub fn new(ident: Identifier, initializer: Box<Expr>) -> Self {
        LetAssignExpr {
            ident,
            initializer,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct LetGetExpr {
    pub ident: Identifier,
}

impl LetGetExpr {
    pub fn new(ident: Identifier) -> Self {
        LetGetExpr { ident }
    }
}

#[derive(PartialEq, Debug)]
pub struct LetSetExpr {
    pub ident: Identifier,
    pub initializer: Box<Expr>,
}

impl LetSetExpr {
    pub fn new(ident: Identifier, initializer: Box<Expr>) -> Self {
        LetSetExpr {
            ident,
            initializer,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct BlockExpr {
    pub exprs: Vec<Expr>,
}

impl BlockExpr {
    pub fn new(exprs: Vec<Expr>) -> Self {
        BlockExpr { exprs }
    }
}

#[derive(PartialEq, Debug)]
pub struct ReturnExpr {
    pub expr: Option<Box<Expr>>,
}

impl ReturnExpr {
    pub fn new(expr: Option<Box<Expr>>) -> Self {
        ReturnExpr { expr }
    }
}

pub type Identifier = String;
