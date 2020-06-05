use lalrpop_util::lalrpop_mod;

pub mod lexer;
pub mod parse_tree;
lalrpop_mod!(parser, "/assembler/parser.rs");
