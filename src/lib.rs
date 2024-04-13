/// Abstract Syntax Trees
pub mod ast;

/// Generated parsers that convert a String to an AST
#[cfg(not(feature = "gen"))]
pub mod gen {
    pub mod test;
}
