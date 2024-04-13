//! This file contains all the possible AST nodes created by our grammar

/// An expression: (root node)
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    /// An expression representing a logical operator applied to two expressions
    Logical(Box<Expr>, LogicalOp, Box<Expr>),
    /// An expression representing a function
    Function(FunctionOp),
    /// An expression representing a comparison operator applied to two expressions
    Comparison(Box<Expr>, ComparisonOp, Box<Expr>),
    /// An expression representing a unary operator applied to a expression
    Unary(UnaryOp, Box<Expr>),
    /// An expression representing an access operator applied to a expression
    Access(Box<Expr>, AccessOp),
    /// An expression representing a list of expressions
    List(Vec<Box<Expr>>),
    /// An expression representing a literal expression
    Literal(Literal),
    /// An expression representing an identifier
    Ident(Ident),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = match self {
            Expr::Logical(left, op, right) => format!("{left} {op} {right}"),
            Expr::Function(op) => format!("{op}"),
            Expr::Comparison(left, op, right) => format!("{left} {op} {right}"),
            Expr::Unary(op, right) => format!("{op}{right}"),
            Expr::Access(left, op) => format!("{left}{op}"),
            Expr::List(list) => format!(
                "[{}]",
                list.iter().map(|i| format!("{i}")).collect::<Vec<String>>().join(", ")
            ),
            Expr::Literal(literal) => format!("{literal}"),
            Expr::Ident(ident) => format!("{ident}"),
        };

        write!(f, "{}", message)
    }
}

/// A logical operator
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LogicalOp {
    /// The logical operator: &&
    And,
    /// The logical operator: ||
    Or,
}

impl std::fmt::Display for LogicalOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = match self {
            LogicalOp::And => "&&",
            LogicalOp::Or => "||",
        };

        write!(f, "{}", message)
    }
}

/// A function operator
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FunctionOp {
    /// The all operation: x.all(y, predicate)
    All(Box<Expr>, Ident, Box<Expr>),
    /// The any operation: x.any(y, predicate)
    Any(Box<Expr>, Ident, Box<Expr>),
    /// The has operation: x.contains(<literal>)
    Contains(Box<Expr>, Literal),
    /// The count operation: x.count()
    Count(Box<Expr>),
    /// The filter operation: x.filter(y, predicate)
    Filter(Box<Expr>, Ident, Box<Expr>),
}

impl std::fmt::Display for FunctionOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = match self {
            FunctionOp::All(left, var, right) => {
                format!("{left}.all({var}, {right})")
            }
            FunctionOp::Any(left, var, right) => {
                format!("{left}.any({var}, {right})")
            }
            FunctionOp::Contains(left, right) => {
                format!("{left}.contains({right})")
            }
            FunctionOp::Count(left) => format!("{left}.count()"),
            FunctionOp::Filter(left, var, right) => {
                format!("{left}.filter({var}, {right})")
            }
        };

        write!(f, "{}", message)
    }
}

/// A comparison operator
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ComparisonOp {
    /// The comparison operator: <
    LessThan,
    /// The comparison operator: <=
    LessThanOrEqual,
    /// The comparison operator: >
    GreaterThan,
    /// The comparison operator: >=
    GreaterThanOrEqual,
    /// The comparison operator: ==
    Equals,
    /// The comparison operator: !=
    NotEquals,
    /// The comparison operator: a in {a,b,c}
    In,
}

impl std::fmt::Display for ComparisonOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = match self {
            ComparisonOp::LessThan => "<",
            ComparisonOp::LessThanOrEqual => "<=",
            ComparisonOp::GreaterThan => ">",
            ComparisonOp::GreaterThanOrEqual => ">=",
            ComparisonOp::Equals => "==",
            ComparisonOp::NotEquals => "!=",
            ComparisonOp::In => "in",
        };

        write!(f, "{}", message)
    }
}

/// An access operator
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AccessOp {
    /// The access operator: x.field
    Field(Ident),
    /// The access operator: \[i\]
    Index(u32),
    /// The access operator: \[i..j\]
    Slice(u32, u32),
}

impl std::fmt::Display for AccessOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = match self {
            AccessOp::Field(field) => format!(".{field}"),
            AccessOp::Index(index) => format!("[{index}]"),
            AccessOp::Slice(start, end) => format!("[{start}..{end}]"),
        };

        write!(f, "{}", message)
    }
}

/// A unary operator
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UnaryOp {
    /// The unary operator: !
    Not,
    /// The unary operator: -
    Negative,
}

impl std::fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = match self {
            UnaryOp::Not => "!",
            UnaryOp::Negative => "-",
        };

        write!(f, "{}", message)
    }
}

/// A literal value (leaf node)
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Literal {
    /// The literal value: i128
    Int(i128),
    /// The literal value: bool
    Bool(bool),
    /// The literal value: String
    String(String),
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = match self {
            Literal::Int(i) => format!("{i}"),
            Literal::Bool(b) => format!("{b}"),
            Literal::String(s) => format!("'{s}'"),
        };

        write!(f, "{}", message)
    }
}

/// An identifier
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Ident {
    /// A variable identifier
    Variable(String),
    /// A reserved, keyword identifier
    Keyword(String),
}

impl std::fmt::Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = match self {
            Ident::Variable(v) => v.to_string(),
            Ident::Keyword(k) => k.to_string(),
        };

        write!(f, "{}", message)
    }
}
