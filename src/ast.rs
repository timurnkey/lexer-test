//! This file contains all the possible AST nodes created by our grammar

/// An expression: (root node)
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    /// An expression representing an access operator applied to a expression
    Access(Box<Expr>, AccessOp),
    /// An expression representing a list of expressions
    List(Vec<Box<Expr>>),
    /// An expression representing a literal expression
    Literal(Literal),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = match self {
            Expr::Access(left, op) => format!("{left}{op}"),
            Expr::List(list) => format!(
                "[{}]",
                list.iter().map(|i| format!("{i}")).collect::<Vec<String>>().join(", ")
            ),
            Expr::Literal(literal) => format!("{literal}"),
        };

        write!(f, "{}", message)
    }
}

/// An access operator
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AccessOp {
    /// The access operator: \[i\]
    Index(u32),
}

impl std::fmt::Display for AccessOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = match self {
            AccessOp::Index(index) => format!("[{index}]"),
        };

        write!(f, "{}", message)
    }
}

/// A literal value (leaf node)
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Literal {
    /// The literal value: i128
    Int(i128),
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = match self {
            Literal::Int(i) => format!("{i}"),
        };

        write!(f, "{}", message)
    }
}
