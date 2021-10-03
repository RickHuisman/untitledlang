use crate::lexer::token::TokenType;
use crate::parser::error::{ParseResult, ParserError};

pub type ModuleAst = Vec<Expr>;
pub type BlockDecl = Vec<Expr>;

#[derive(PartialEq, Debug)]
pub enum Expr {
    Grouping {
        expr: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOperator,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOperator,
        expr: Box<Expr>,
    },
    LetAssign {
        ident: Identifier,
        initializer: Box<Expr>,
    },
    LetGet {
        ident: Identifier,
    },
    LetSet {
        ident: Identifier,
        expr: Box<Expr>,
    },
    Fun {
        ident: Identifier,
        decl: FunDecl,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    While {
        condition: Box<Expr>,
        body: Box<Expr>,
    },
    IfElse {
        condition: Box<Expr>,
        then: BlockDecl,
        else_: Option<BlockDecl>,
    },
    Block {
        block: Box<BlockDecl>,
    },
    Print {
        expr: Box<Expr>,
    },
    Return {
        expr: Option<Box<Expr>>,
    },
    Literal(LiteralExpr),
}

impl Expr {
    pub fn grouping(expr: Expr) -> Self {
        Expr::Grouping {
            expr: Box::new(expr),
        }
    }

    pub fn binary(left: Expr, op: BinaryOperator, right: Expr) -> Self {
        Expr::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }

    pub fn unary(op: UnaryOperator, expr: Expr) -> Self {
        Expr::Unary {
            op,
            expr: Box::new(expr),
        }
    }

    pub fn let_assign(ident: Identifier, initializer: Expr) -> Self {
        Expr::LetAssign {
            ident,
            initializer: Box::new(initializer),
        }
    }

    pub fn let_get(ident: Identifier) -> Self {
        Expr::LetGet { ident }
    }

    pub fn let_set(ident: Identifier, expr: Expr) -> Self {
        Expr::LetSet {
            ident,
            expr: Box::new(expr),
        }
    }

    pub fn fun(ident: Identifier, decl: FunDecl) -> Self {
        Expr::Fun { ident, decl }
    }

    pub fn call(callee: Expr, args: Vec<Expr>) -> Self {
        Expr::Call {
            callee: Box::new(callee),
            args,
        }
    }

    pub fn while_(condition: Expr, body: Expr) -> Self {
        Expr::While {
            condition: Box::new(condition),
            body: Box::new(body),
        }
    }

    pub fn if_else(condition: Expr, then: BlockDecl, else_: Option<BlockDecl>) -> Self {
        Expr::IfElse {
            condition: Box::new(condition),
            then,
            else_,
        }
    }

    pub fn block(block: BlockDecl) -> Self {
        Expr::Block {
            block: Box::new(block),
        }
    }

    pub fn print(expr: Expr) -> Self {
        Expr::Print {
            expr: Box::new(expr),
        }
    }

    pub fn return_(expr: Option<Expr>) -> Self {
        // TODO: Clean up.
        let foo = match expr {
            Some(e) => Some(Box::new(e)),
            None => None,
        };

        Expr::Return { expr: foo }
    }
}

pub type Identifier = String;

#[derive(PartialEq, Debug)]
pub enum LiteralExpr {
    Number(f64),
    String(String),
    True,
    False,
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
    pub fn from_token(token_type: &TokenType) -> ParseResult<BinaryOperator> {
        Ok(match token_type {
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
            _ => return Err(ParserError::ExpectedBinaryOperator(token_type.clone())),
        })
    }
}

#[derive(PartialEq, Debug)]
pub enum UnaryOperator {
    Negate,
    Not,
}

impl UnaryOperator {
    pub fn from_token(token_type: &TokenType) -> ParseResult<UnaryOperator> {
        Ok(match token_type {
            TokenType::Minus => UnaryOperator::Negate,
            TokenType::Bang => UnaryOperator::Not,
            _ => return Err(ParserError::ExpectedUnaryOperator(token_type.clone())),
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct FunDecl {
    args: Vec<Identifier>,
    body: BlockDecl,
}

impl FunDecl {
    pub fn new(args: Vec<Identifier>, body: BlockDecl) -> Self {
        FunDecl { args, body }
    }

    pub fn args(&self) -> &Vec<Identifier> {
        &self.args
    }

    pub fn body(self) -> BlockDecl {
        self.body
    }
}
