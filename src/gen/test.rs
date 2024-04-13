// auto-generated: "lalrpop 0.19.12"
// sha3: e92c2ff083fcd5a8b2db37d011abb0a1eaa5de7c441f6ea5dbfdfee1801c8a83
use crate::ast::{
    Expr, Literal, Ident,
    LogicalOp, ComparisonOp, UnaryOp, AccessOp, FunctionOp
};
use lalrpop_util::ParseError;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::all)]

    use crate::ast::{
    Expr, Literal, Ident,
    LogicalOp, ComparisonOp, UnaryOp, AccessOp, FunctionOp
};
    use lalrpop_util::ParseError;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(Box<Expr>),
        Variant2(alloc::vec::Vec<Box<Expr>>),
        Variant3(AccessOp),
        Variant4(ComparisonOp),
        Variant5(Ident),
        Variant6(Literal),
        Variant7(core::option::Option<Box<Expr>>),
        Variant8(UnaryOp),
        Variant9(Vec<Box<Expr>>),
    }
    const __ACTION: &[i8] = &[
        // State 0
        26, 0, 0, 4, 0, 0, 27, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 5, 0, 29, 30, 31, 32, 0, 33, 34, 35, 0, 36, 37, 38, 0, 0,
        // State 1
        0, 39, -20, 0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 41, 42, 43, 44, 0, 0, 0, 0, 0, 0, 45, 0, 0, 0, -20, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 5, 0, 29, 30, 31, 32, 0, 33, 34, 35, 0, 36, 37, 38, 0, 0,
        // State 3
        26, 0, 0, 4, 0, 0, 27, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 5, 0, 29, 30, 31, 32, 0, 33, 34, 35, 0, 36, 37, 38, 0, 0,
        // State 4
        0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 5, -50, 29, 30, 31, 32, 0, 33, 34, 35, 0, 36, 37, 38, 0, 0,
        // State 5
        26, 0, 0, 4, 0, 0, 27, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 5, 0, 29, 30, 31, 32, 0, 33, 34, 35, 0, 36, 37, 38, 0, 0,
        // State 6
        26, 0, 0, 4, 0, 0, 27, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 5, 0, 29, 30, 31, 32, 0, 33, 34, 35, 0, 36, 37, 38, 0, 0,
        // State 7
        26, 0, 0, 4, 0, 0, 27, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 5, 0, 29, 30, 31, 32, 0, 33, 34, 35, 0, 36, 37, 38, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 30, 31, 0, 0, 33, 0, 35, 0, 0, 0, 38, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 30, 31, 0, 0, 33, 0, 35, 0, 0, 0, 38, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 34, 0, 0, 36, 37, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 30, 31, 0, 0, 33, 0, 35, 0, 0, 0, 38, 0, 0,
        // State 12
        0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 5, -52, 29, 30, 31, 32, 0, 33, 34, 35, 0, 36, 37, 38, 0, 0,
        // State 13
        0, 39, -19, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 41, 42, 43, 44, 0, 0, 0, 0, 0, 0, 45, 0, 0, 0, -19, 0, 0, 0, 0, 0,
        // State 14
        0, 39, -18, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 41, 42, 43, 44, 0, 0, 0, 0, 0, 0, 45, 0, 0, 0, -18, 0, 0, 0, 0, 0,
        // State 15
        26, 0, 0, 4, 0, 0, 27, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 5, 0, 29, 30, 31, 32, 0, 33, 34, 35, 0, 36, 37, 38, 0, 0,
        // State 16
        26, 0, 0, 4, 0, 0, 27, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 5, 0, 29, 30, 31, 32, 0, 33, 34, 35, 0, 36, 37, 38, 0, 0,
        // State 17
        26, 0, 0, 4, 0, 0, 27, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 0, 0, 5, 0, 29, 30, 31, 32, 0, 33, 34, 35, 0, 36, 37, 38, 0, 0,
        // State 18
        0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0,
        // State 19
        0, -38, -38, 0, -38, 0, 0, 0, -38, -38, -38, -38, -38, 0, -38, -38, -38, -38, -38, 0, 0, 0, 0, 0, 0, -38, 0, 0, 0, -38, 0, 0, 0, 0, 0,
        // State 20
        0, -39, -39, 0, -39, -39, 0, 0, -39, -39, -39, -39, -39, 0, -39, -39, -39, -39, -39, 0, -39, 0, 0, 0, 0, -39, 0, 0, 0, -39, 0, 0, 0, 0, 0,
        // State 21
        0, -40, -40, 0, -40, -40, 0, 0, -40, -40, -40, -40, -40, 0, -40, -40, -40, -40, -40, 0, -40, 0, 0, 0, 0, -40, 0, 0, 0, -40, 0, 0, 0, 0, 0,
        // State 22
        0, -46, -46, 0, -46, 0, 0, 0, 9, 10, 11, 46, 12, 0, -46, -46, -46, -46, -46, 0, 0, 0, 0, 0, 0, -46, 0, 0, 0, -46, 0, 0, 0, 0, 0,
        // State 23
        0, -26, -26, 0, -26, 0, 0, 0, -26, -26, -26, -26, -26, 0, -26, -26, -26, -26, -26, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, -26, 0, 0, 0, 0, 0,
        // State 24
        0, -10, -10, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, -10, -10, -10, -10, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0, -10, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, -47, 0, 0, 0, 0, 0, -47, 0, -47, -47, -47, -47, 0, -47, -47, -47, 0, -47, -47, -47, 0, 0,
        // State 26
        0, 0, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, -48, 0, 0, 0, 0, 0, -48, 0, -48, -48, -48, -48, 0, -48, -48, -48, 0, -48, -48, -48, 0, 0,
        // State 27
        0, -35, -35, 0, -35, -35, 0, 0, -35, -35, -35, -35, -35, 0, -35, -35, -35, -35, -35, 0, -35, 0, 0, 0, 0, -35, 0, 0, 0, -35, 0, 0, 0, 0, 0,
        // State 28
        0, -28, -28, 0, -28, -28, 0, 0, -28, -28, -28, -28, -28, 0, -28, -28, -28, -28, -28, 0, -28, 0, 0, 0, 0, -28, 0, 0, 0, -28, 0, 0, 0, 0, 0,
        // State 29
        0, -29, -29, 0, -29, -29, 0, 0, -29, -29, -29, -29, -29, 0, -29, -29, -29, -29, -29, 0, -29, 0, 0, 0, 0, -29, 0, 0, 0, -29, 0, 0, 0, 0, 0,
        // State 30
        0, -32, -32, 0, -32, -32, 0, 0, -32, -32, -32, -32, -32, 0, -32, -32, -32, -32, -32, 0, -32, 0, 0, 0, 0, -32, 0, 0, 0, -32, 0, 0, 0, 0, 0,
        // State 31
        0, -34, -34, 0, -34, -34, 0, 0, -34, -34, -34, -34, -34, 0, -34, -34, -34, -34, -34, 0, -34, 0, 0, 0, 0, -34, 0, 0, 0, -34, 0, 0, 0, 0, 0,
        // State 32
        0, -30, -30, 0, -30, -30, 0, 0, -30, -30, -30, -30, -30, 0, -30, -30, -30, -30, -30, 0, -30, 0, 0, 0, 0, -30, 0, 0, 0, -30, 0, 0, 0, 0, 0,
        // State 33
        0, -33, -33, 0, -33, -33, 0, 0, -33, -33, -33, -33, -33, 0, -33, -33, -33, -33, -33, 0, -33, 0, 0, 0, 0, -33, 0, 0, 0, -33, 0, 0, 0, 0, 0,
        // State 34
        0, -31, -31, 0, -31, -31, 0, 0, -31, -31, -31, -31, -31, 0, -31, -31, -31, -31, -31, 0, -31, 0, 0, 0, 0, -31, 0, 0, 0, -31, 0, 0, 0, 0, 0,
        // State 35
        0, -37, -37, 0, -37, -37, 0, 0, -37, -37, -37, -37, -37, 0, -37, -37, -37, -37, -37, 0, -37, 0, 0, 0, 0, -37, 0, 0, 0, -37, 0, 0, 0, 0, 0,
        // State 36
        0, -36, -36, 0, -36, -36, 0, 0, -36, -36, -36, -36, -36, 0, -36, -36, -36, -36, -36, 0, -36, 0, 0, 0, 0, -36, 0, 0, 0, -36, 0, 0, 0, 0, 0,
        // State 37
        0, -27, -27, 0, -27, -27, 0, 0, -27, -27, -27, -27, -27, 0, -27, -27, -27, -27, -27, 0, -27, 0, 0, 0, 0, -27, 0, 0, 0, -27, 0, 0, 0, 0, 0,
        // State 38
        -16, 0, 0, -16, 0, 0, -16, 0, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0, 0, -16, 0, -16, -16, -16, -16, 0, -16, -16, -16, 0, -16, -16, -16, 0, 0,
        // State 39
        -11, 0, 0, -11, 0, 0, -11, 0, 0, 0, 0, 0, 0, -11, 0, 0, 0, 0, 0, -11, 0, -11, -11, -11, -11, 0, -11, -11, -11, 0, -11, -11, -11, 0, 0,
        // State 40
        -12, 0, 0, -12, 0, 0, -12, 0, 0, 0, 0, 0, 0, -12, 0, 0, 0, 0, 0, -12, 0, -12, -12, -12, -12, 0, -12, -12, -12, 0, -12, -12, -12, 0, 0,
        // State 41
        -15, 0, 0, -15, 0, 0, -15, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0, 0, 0, -15, 0, -15, -15, -15, -15, 0, -15, -15, -15, 0, -15, -15, -15, 0, 0,
        // State 42
        -13, 0, 0, -13, 0, 0, -13, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0, 0, 0, -13, 0, -13, -13, -13, -13, 0, -13, -13, -13, 0, -13, -13, -13, 0, 0,
        // State 43
        -14, 0, 0, -14, 0, 0, -14, 0, 0, 0, 0, 0, 0, -14, 0, 0, 0, 0, 0, -14, 0, -14, -14, -14, -14, 0, -14, -14, -14, 0, -14, -14, -14, 0, 0,
        // State 44
        -17, 0, 0, -17, 0, 0, -17, 0, 0, 0, 0, 0, 0, -17, 0, 0, 0, 0, 0, -17, 0, -17, -17, -17, -17, 0, -17, -17, -17, 0, -17, -17, -17, 0, 0,
        // State 45
        0, -24, -24, 0, -24, 0, 0, 0, -24, -24, -24, -24, -24, 0, -24, -24, -24, -24, -24, 0, 0, 0, 0, 0, 0, -24, 0, 0, 0, -24, 0, 0, 0, 0, 0,
        // State 46
        0, -45, -45, 0, -45, 0, 0, 0, 9, 10, 11, 46, 12, 0, -45, -45, -45, -45, -45, 0, 0, 0, 0, 0, 0, -45, 0, 0, 0, -45, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 7, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, -9, -9, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, -9, -9, -9, -9, 0, 0, 0, 0, 0, 0, -9, 0, 0, 0, -9, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, -42, -42, 0, -42, -42, 0, 0, -42, -42, -42, -42, -42, 0, -42, -42, -42, -42, -42, 0, -42, 0, 0, 0, 0, -42, 0, 0, 0, -42, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0, 0, 0, 0, 0, -4, -4, -4, -4, -4, -4, 0, -4, -4, -4, 0, -4, -4, -4, 0, 0,
        // State 58
        0, -41, -41, 0, -41, -41, 0, 0, -41, -41, -41, -41, -41, 0, -41, -41, -41, -41, -41, 0, -41, 0, 0, 0, 0, -41, 0, 0, 0, -41, 0, 0, 0, 0, 0,
        // State 59
        0, -23, -23, 0, -23, 0, 0, 0, -23, -23, -23, -23, -23, 0, -23, -23, -23, -23, -23, 0, 0, 0, 0, 0, 0, -23, 0, 0, 0, -23, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0, 0, 0, 0, 0, -5, -5, -5, -5, -5, -5, 0, -5, -5, -5, 0, -5, -5, -5, 0, 0,
        // State 61
        0, 0, 7, 0, 65, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0,
        // State 62
        0, 0, 7, 0, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0,
        // State 63
        0, 0, 7, 0, 67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0,
        // State 64
        0, -21, -21, 0, -21, 0, 0, 0, -21, -21, -21, -21, -21, 0, -21, -21, -21, -21, -21, 0, 0, 0, 0, 0, 0, -21, 0, 0, 0, -21, 0, 0, 0, 0, 0,
        // State 65
        0, -22, -22, 0, -22, 0, 0, 0, -22, -22, -22, -22, -22, 0, -22, -22, -22, -22, -22, 0, 0, 0, 0, 0, 0, -22, 0, 0, 0, -22, 0, 0, 0, 0, 0,
        // State 66
        0, -25, -25, 0, -25, 0, 0, 0, -25, -25, -25, -25, -25, 0, -25, -25, -25, -25, -25, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, -25, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 35 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -20,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        -19,
        // State 14
        -18,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        -53,
        // State 19
        -38,
        // State 20
        -39,
        // State 21
        -40,
        // State 22
        -46,
        // State 23
        -26,
        // State 24
        -10,
        // State 25
        0,
        // State 26
        0,
        // State 27
        -35,
        // State 28
        -28,
        // State 29
        -29,
        // State 30
        -32,
        // State 31
        -34,
        // State 32
        -30,
        // State 33
        -33,
        // State 34
        -31,
        // State 35
        -37,
        // State 36
        -36,
        // State 37
        -27,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        -24,
        // State 46
        -45,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        -9,
        // State 51
        0,
        // State 52
        0,
        // State 53
        0,
        // State 54
        0,
        // State 55
        -42,
        // State 56
        0,
        // State 57
        0,
        // State 58
        -41,
        // State 59
        -23,
        // State 60
        0,
        // State 61
        0,
        // State 62
        0,
        // State 63
        0,
        // State 64
        -21,
        // State 65
        -22,
        // State 66
        -25,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            2 => 12,
            4 => match state {
                6 => 13,
                7 => 14,
                _ => 1,
            },
            5 => 5,
            6 => match state {
                3 => 47,
                15 => 61,
                16 => 62,
                17 => 63,
                _ => 18,
            },
            7 => 19,
            8 => match state {
                8 => 51,
                9 => 52,
                11 => 54,
                _ => 20,
            },
            9 => match state {
                10 => 53,
                _ => 21,
            },
            10 => match state {
                2 => 46,
                _ => 22,
            },
            11 => match state {
                4 => 48,
                12 => 56,
                _ => 23,
            },
            13 => match state {
                5 => 50,
                _ => 24,
            },
            14 => 2,
            15 => 49,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""!""###,
            r###""!=""###,
            r###""&&""###,
            r###""(""###,
            r###"")""###,
            r###"",""###,
            r###""-""###,
            r###"".""###,
            r###"".all(""###,
            r###"".any(""###,
            r###"".contains(""###,
            r###"".count()""###,
            r###"".filter(""###,
            r###""0""###,
            r###""<""###,
            r###""<=""###,
            r###""==""###,
            r###"">""###,
            r###"">=""###,
            r###""[""###,
            r###""]""###,
            r###""activity""###,
            r###""approvers""###,
            r###""eth.tx""###,
            r###""false""###,
            r###""in""###,
            r###""private_key""###,
            r###""true""###,
            r###""wallet""###,
            r###""||""###,
            r###"r#"'[^'\\n]*'"#"###,
            r###"r#"[1-9]+[0-9]*"#"###,
            r###"r#"[a-zA-Z][_a-zA-Z0-9]*"#"###,
            r###"r#"\\[[0-9]+\\.\\.[0-9]+\\]"#"###,
            r###"r#"\\[[0-9]+\\]"#"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<'input>
    where 
    {
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input ())>,
    }
    impl<'input> __state_machine::ParserDefinition for __StateMachine<'input>
    where 
    {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = Box<Expr>;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 35 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
        'input,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(5, _) if true => Some(0),
            Token(6, _) if true => Some(1),
            Token(7, _) if true => Some(2),
            Token(8, _) if true => Some(3),
            Token(9, _) if true => Some(4),
            Token(10, _) if true => Some(5),
            Token(11, _) if true => Some(6),
            Token(12, _) if true => Some(7),
            Token(13, _) if true => Some(8),
            Token(14, _) if true => Some(9),
            Token(15, _) if true => Some(10),
            Token(16, _) if true => Some(11),
            Token(17, _) if true => Some(12),
            Token(18, _) if true => Some(13),
            Token(19, _) if true => Some(14),
            Token(20, _) if true => Some(15),
            Token(21, _) if true => Some(16),
            Token(22, _) if true => Some(17),
            Token(23, _) if true => Some(18),
            Token(24, _) if true => Some(19),
            Token(25, _) if true => Some(20),
            Token(26, _) if true => Some(21),
            Token(27, _) if true => Some(22),
            Token(28, _) if true => Some(23),
            Token(29, _) if true => Some(24),
            Token(30, _) if true => Some(25),
            Token(31, _) if true => Some(26),
            Token(32, _) if true => Some(27),
            Token(33, _) if true => Some(28),
            Token(34, _) if true => Some(29),
            Token(0, _) if true => Some(30),
            Token(1, _) if true => Some(31),
            Token(2, _) if true => Some(32),
            Token(3, _) if true => Some(33),
            Token(4, _) if true => Some(34),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 => match __token {
                Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(17, __tok0) | Token(18, __tok0) | Token(19, __tok0) | Token(20, __tok0) | Token(21, __tok0) | Token(22, __tok0) | Token(23, __tok0) | Token(24, __tok0) | Token(25, __tok0) | Token(26, __tok0) | Token(27, __tok0) | Token(28, __tok0) | Token(29, __tok0) | Token(30, __tok0) | Token(31, __tok0) | Token(32, __tok0) | Token(33, __tok0) | Token(34, __tok0) | Token(0, __tok0) | Token(1, __tok0) | Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct ExprParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl ExprParser {
        pub fn new() -> ExprParser {
            let __builder = super::__intern_token::new_builder();
            ExprParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
        >(
            &self,
            input: &'input str,
        ) -> Result<Box<Expr>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    input,
                    __phantom: core::marker::PhantomData::<(&())>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<Result<Box<Expr>,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            1 => {
                __reduce1(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            2 => {
                __reduce2(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            3 => {
                __reduce3(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            4 => {
                __reduce4(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            5 => {
                __reduce5(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            6 => {
                // AccessOp = r#"\\[[0-9]+\\]"# => ActionFn(20);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action20::<>(input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (1, 3)
            }
            7 => {
                // AccessOp = r#"\\[[0-9]+\\.\\.[0-9]+\\]"# => ActionFn(21);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action21::<>(input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant3(__nt), __end));
                (1, 3)
            }
            8 => {
                __reduce8(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            9 => {
                __reduce9(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            10 => {
                __reduce10(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            11 => {
                __reduce11(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            12 => {
                __reduce12(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            13 => {
                __reduce13(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            14 => {
                __reduce14(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            15 => {
                __reduce15(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            16 => {
                __reduce16(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            17 => {
                __reduce17(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            18 => {
                __reduce18(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            19 => {
                __reduce19(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            20 => {
                __reduce20(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            21 => {
                __reduce21(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            22 => {
                __reduce22(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            23 => {
                __reduce23(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            24 => {
                __reduce24(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            25 => {
                __reduce25(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            26 => {
                __reduce26(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            27 => {
                __reduce27(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            28 => {
                __reduce28(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            29 => {
                __reduce29(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            30 => {
                __reduce30(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            31 => {
                __reduce31(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            32 => {
                __reduce32(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            33 => {
                __reduce33(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            34 => {
                __reduce34(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            35 => {
                // Literal = r#"[1-9]+[0-9]*"# => ActionFn(34);
                let __sym0 = __pop_Variant0(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action34::<>(input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant6(__nt), __end));
                (1, 9)
            }
            36 => {
                __reduce36(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            37 => {
                __reduce37(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            38 => {
                __reduce38(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            39 => {
                __reduce39(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            40 => {
                __reduce40(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            41 => {
                __reduce41(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            42 => {
                __reduce42(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            43 => {
                __reduce43(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            44 => {
                __reduce44(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            45 => {
                __reduce45(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            46 => {
                __reduce46(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            47 => {
                __reduce47(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            48 => {
                __reduce48(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            49 => {
                __reduce49(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            50 => {
                __reduce50(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            51 => {
                __reduce51(input, __lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            52 => {
                // __Expr = Expr => ActionFn(0);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, AccessOp, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ComparisonOp, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Ident, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant5(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Literal, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant6(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant8<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, UnaryOp, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant8(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant9<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Box<Expr>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant9(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, alloc::vec::Vec<Box<Expr>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant7<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, core::option::Option<Box<Expr>>, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant7(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Term> ",") = Term, "," => ActionFn(47);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action47::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Term> ",")* =  => ActionFn(45);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action45::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (0, 1)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Term> ",")* = (<Term> ",")+ => ActionFn(46);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action46::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Term> ",")+ = Term, "," => ActionFn(50);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action50::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 2)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // (<Term> ",")+ = (<Term> ",")+, Term, "," => ActionFn(51);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action51::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 2)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // AccessOp = ".", Ident => ActionFn(19);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action19::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 3)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Calc = Calc, ComparisonOp, Unary => ActionFn(4);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action4::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 4)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Calc = Unary => ActionFn(5);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ComparisonOp = "<" => ActionFn(24);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ComparisonOp = "<=" => ActionFn(25);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ComparisonOp = ">" => ActionFn(26);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ComparisonOp = ">=" => ActionFn(27);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ComparisonOp = "==" => ActionFn(28);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ComparisonOp = "!=" => ActionFn(29);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ComparisonOp = "in" => ActionFn(30);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = Expr, "||", Calc => ActionFn(1);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action1::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 6)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = Expr, "&&", Calc => ActionFn(2);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action2::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 6)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expr = Calc => ActionFn(3);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action3::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Function = Member, ".all(", Ident, ",", Expr, ")" => ActionFn(9);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant1(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action9::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (6, 7)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Function = Member, ".any(", Ident, ",", Expr, ")" => ActionFn(10);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant1(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action10::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (6, 7)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Function = Member, ".contains(", Literal, ")" => ActionFn(11);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant6(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action11::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 7)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Function = Member, ".count()" => ActionFn(12);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action12::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 7)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Function = Member, ".filter(", Ident, ",", Expr, ")" => ActionFn(13);
        assert!(__symbols.len() >= 6);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant1(__symbols);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym5.2.clone();
        let __nt = super::__action13::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (6, 7)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Function = Term => ActionFn(14);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 7)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Ident = r#"[a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(36);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action36::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Ident = "activity" => ActionFn(37);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action37::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Ident = "approvers" => ActionFn(38);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Ident = "private_key" => ActionFn(39);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action39::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce30<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Ident = "wallet" => ActionFn(40);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce31<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Ident = "eth.tx" => ActionFn(41);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce32<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Literal = "true" => ActionFn(31);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action31::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce33<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Literal = "false" => ActionFn(32);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action32::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce34<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Literal = "0" => ActionFn(33);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce36<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Literal = r#"'[^'\\n]*'"# => ActionFn(35);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (1, 9)
    }
    pub(crate) fn __reduce37<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Member = Function => ActionFn(8);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 10)
    }
    pub(crate) fn __reduce38<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Term = Ident => ActionFn(15);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce39<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Term = Literal => ActionFn(16);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 11)
    }
    pub(crate) fn __reduce40<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Term = "[", WithCommas<Term>, "]" => ActionFn(17);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant9(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action17::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce41<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Term = "(", Expr, ")" => ActionFn(18);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action18::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 11)
    }
    pub(crate) fn __reduce42<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Term? = Term => ActionFn(43);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (1, 12)
    }
    pub(crate) fn __reduce43<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Term? =  => ActionFn(44);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action44::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant7(__nt), __end));
        (0, 12)
    }
    pub(crate) fn __reduce44<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Unary = UnaryOp, Member => ActionFn(6);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant8(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action6::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 13)
    }
    pub(crate) fn __reduce45<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Unary = Member => ActionFn(7);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action7::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 13)
    }
    pub(crate) fn __reduce46<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // UnaryOp = "!" => ActionFn(22);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce47<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // UnaryOp = "-" => ActionFn(23);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant8(__nt), __end));
        (1, 14)
    }
    pub(crate) fn __reduce48<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // WithCommas<Term> = Term => ActionFn(54);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action54::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 15)
    }
    pub(crate) fn __reduce49<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // WithCommas<Term> =  => ActionFn(55);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action55::<>(input, &__start, &__end);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (0, 15)
    }
    pub(crate) fn __reduce50<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // WithCommas<Term> = (<Term> ",")+, Term => ActionFn(56);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action56::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (2, 15)
    }
    pub(crate) fn __reduce51<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // WithCommas<Term> = (<Term> ",")+ => ActionFn(57);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action57::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant9(__nt), __end));
        (1, 15)
    }
}
pub use self::__parse__Expr::ExprParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use crate::ast::{
    Expr, Literal, Ident,
    LogicalOp, ComparisonOp, UnaryOp, AccessOp, FunctionOp
};
    use lalrpop_util::ParseError;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
        let __strs: &[(&str, bool)] = &[
            ("^('[\0-\t\u{b}-\\&\\(-\u{10ffff}]*')", false),
            ("^([1-9]+[0-9]*)", false),
            ("^([A-Za-z][0-9A-Z_a-z]*)", false),
            ("^(\\[[0-9]+\\.\\.[0-9]+\\])", false),
            ("^(\\[[0-9]+\\])", false),
            ("^(!)", false),
            ("^(!=)", false),
            ("^(\\&\\&)", false),
            ("^(\\()", false),
            ("^(\\))", false),
            ("^(,)", false),
            ("^(\\-)", false),
            ("^(\\.)", false),
            ("^(\\.all\\()", false),
            ("^(\\.any\\()", false),
            ("^(\\.contains\\()", false),
            ("^(\\.count\\(\\))", false),
            ("^(\\.filter\\()", false),
            ("^(0)", false),
            ("^(<)", false),
            ("^(<=)", false),
            ("^(==)", false),
            ("^(>)", false),
            ("^(>=)", false),
            ("^(\\[)", false),
            ("^(\\])", false),
            ("^(activity)", false),
            ("^(approvers)", false),
            ("^(eth\\.tx)", false),
            ("^(false)", false),
            ("^(in)", false),
            ("^(private_key)", false),
            ("^(true)", false),
            ("^(wallet)", false),
            ("^(\\|\\|)", false),
            (r"^(\s*)", true),
        ];
        __lalrpop_util::lexer::MatcherBuilder::new(__strs.iter().copied()).unwrap()
    }
}
pub(crate) use self::__lalrpop_util::lexer::Token;

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    __0
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, left, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, right, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::Logical(left, LogicalOp::Or, right))
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, left, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, right, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::Logical(left, LogicalOp::And, right))
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    __0
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, left, _): (usize, Box<Expr>, usize),
    (_, op, _): (usize, ComparisonOp, usize),
    (_, right, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::Comparison(left, op, right))
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    __0
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, op, _): (usize, UnaryOp, usize),
    (_, right, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::Unary(op, right))
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    __0
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    __0
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, left, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, ident, _): (usize, Ident, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, right, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Expr>
{
    Box::new(Expr::Function(FunctionOp::All(left, ident, right)))
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, left, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, ident, _): (usize, Ident, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, right, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Expr>
{
    Box::new(Expr::Function(FunctionOp::Any(left, ident, right)))
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, left, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, right, _): (usize, Literal, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Expr>
{
    Box::new(Expr::Function(FunctionOp::Contains(left, right)))
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    input: &'input str,
    (_, left, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Expr>
{
    Box::new(Expr::Function(FunctionOp::Count(left)))
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    input: &'input str,
    (_, left, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, ident, _): (usize, Ident, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, right, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Expr>
{
    Box::new(Expr::Function(FunctionOp::Filter(left, ident, right)))
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    __0
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Ident, usize),
) -> Box<Expr>
{
    Box::new(Expr::Ident(__0))
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Literal, usize),
) -> Box<Expr>
{
    Box::new(Expr::Literal(__0))
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Vec<Box<Expr>>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Expr>
{
    Box::new(Expr::List(__0))
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Expr>
{
    __0
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, field, _): (usize, Ident, usize),
) -> AccessOp
{
    AccessOp::Field(field)
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Result<AccessOp,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    {
        let parts = &__0[1..(__0.len() - 1)];
        let index = parts.parse::<u32>().map_err(|e| ParseError::User{
            error: "unable to parse access operation to u32",
        })?;

        Ok(AccessOp::Index(index))
    }
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Result<AccessOp,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    {
        let parts: Vec<String> = __0[1..(__0.len() - 1)].split("..").map(|s| s.to_string()).collect();
        let start = parts[0].parse::<u32>().map_err(|e| ParseError::User{
            error: "unable to parse access operation to u32",
        })?;
        let end = parts[1].parse::<u32>().map_err(|e| ParseError::User{
            error: "unable to parse access operation to u32",
        })?;

        Ok(AccessOp::Slice(start, end))
    }
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> UnaryOp
{
    UnaryOp::Not
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> UnaryOp
{
    UnaryOp::Negative
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ComparisonOp
{
    ComparisonOp::LessThan
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ComparisonOp
{
    ComparisonOp::LessThanOrEqual
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ComparisonOp
{
    ComparisonOp::GreaterThan
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ComparisonOp
{
    ComparisonOp::GreaterThanOrEqual
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ComparisonOp
{
    ComparisonOp::Equals
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ComparisonOp
{
    ComparisonOp::NotEquals
}

#[allow(unused_variables)]
fn __action30<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ComparisonOp
{
    ComparisonOp::In
}

#[allow(unused_variables)]
fn __action31<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Literal
{
    Literal::Bool(true)
}

#[allow(unused_variables)]
fn __action32<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Literal
{
    Literal::Bool(false)
}

#[allow(unused_variables)]
fn __action33<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Literal
{
    Literal::Int(0)
}

#[allow(unused_variables)]
fn __action34<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Result<Literal,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    {
        let i = __0.parse::<i128>().map_err(|e| ParseError::User{
            error: "unable to parse int to i128",
        })?;

        Ok(Literal::Int(i))
    }
}

#[allow(unused_variables)]
fn __action35<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Literal
{
    Literal::String(__0.replace("\'", ""))
}

#[allow(unused_variables)]
fn __action36<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Ident
{
    Ident::Variable(__0.to_string())
}

#[allow(unused_variables)]
fn __action37<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Ident
{
    Ident::Keyword(__0.into())
}

#[allow(unused_variables)]
fn __action38<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Ident
{
    Ident::Keyword(__0.into())
}

#[allow(unused_variables)]
fn __action39<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Ident
{
    Ident::Keyword(__0.into())
}

#[allow(unused_variables)]
fn __action40<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Ident
{
    Ident::Keyword(__0.into())
}

#[allow(unused_variables)]
fn __action41<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Ident
{
    Ident::Keyword(__0.into())
}

#[allow(unused_variables)]
fn __action42<
    'input,
>(
    input: &'input str,
    (_, mut v, _): (usize, alloc::vec::Vec<Box<Expr>>, usize),
    (_, e, _): (usize, core::option::Option<Box<Expr>>, usize),
) -> Vec<Box<Expr>>
{
    match e {
        None => v,
        Some(e) => {
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
fn __action43<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> core::option::Option<Box<Expr>>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action44<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> core::option::Option<Box<Expr>>
{
    None
}

#[allow(unused_variables)]
fn __action45<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> alloc::vec::Vec<Box<Expr>>
{
    alloc::vec![]
}

#[allow(unused_variables)]
fn __action46<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Box<Expr>>, usize),
) -> alloc::vec::Vec<Box<Expr>>
{
    v
}

#[allow(unused_variables)]
fn __action47<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Expr>
{
    __0
}

#[allow(unused_variables)]
fn __action48<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> alloc::vec::Vec<Box<Expr>>
{
    alloc::vec![__0]
}

#[allow(unused_variables)]
fn __action49<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, alloc::vec::Vec<Box<Expr>>, usize),
    (_, e, _): (usize, Box<Expr>, usize),
) -> alloc::vec::Vec<Box<Expr>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action50<
    'input,
>(
    input: &'input str,
    __0: (usize, Box<Expr>, usize),
    __1: (usize, &'input str, usize),
) -> alloc::vec::Vec<Box<Expr>>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action47(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action48(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action51<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Box<Expr>>, usize),
    __1: (usize, Box<Expr>, usize),
    __2: (usize, &'input str, usize),
) -> alloc::vec::Vec<Box<Expr>>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action47(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action49(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action52<
    'input,
>(
    input: &'input str,
    __0: (usize, core::option::Option<Box<Expr>>, usize),
) -> Vec<Box<Expr>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action45(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action42(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action53<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Box<Expr>>, usize),
    __1: (usize, core::option::Option<Box<Expr>>, usize),
) -> Vec<Box<Expr>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action46(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action42(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action54<
    'input,
>(
    input: &'input str,
    __0: (usize, Box<Expr>, usize),
) -> Vec<Box<Expr>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action43(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action52(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action55<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Box<Expr>>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action52(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action56<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Box<Expr>>, usize),
    __1: (usize, Box<Expr>, usize),
) -> Vec<Box<Expr>>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action43(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action53(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action57<
    'input,
>(
    input: &'input str,
    __0: (usize, alloc::vec::Vec<Box<Expr>>, usize),
) -> Vec<Box<Expr>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action44(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action53(
        input,
        __0,
        __temp0,
    )
}

pub trait __ToTriple<'input, >
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>;
}

impl<'input, > __ToTriple<'input, > for (usize, Token<'input>, usize)
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, Token<'input>, usize), &'static str>
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
