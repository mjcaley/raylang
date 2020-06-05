use logos::{Lexer, Logos};

pub fn lex_string(source: &str) -> Lexer<Token> {
    return Token::lexer(source);
}

fn from_hex_string(lex: &mut Lexer<Token>) -> Option<u64> {
    let slice = lex.slice();
    match u64::from_str_radix(slice.trim_start_matches("0x"), 16) {
        Ok(number) => Some(number),
        Err(_) => None,
    }
}

fn from_bin_string(lex: &mut Lexer<Token>) -> Option<u64> {
    let slice = lex.slice();
    match u64::from_str_radix(slice.trim_start_matches("0b"), 2) {
        Ok(number) => Some(number),
        Err(_) => None,
    }
}

#[derive(Debug)]
pub struct LexError(std::ops::Range<usize>, &'static str);

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    #[token("define")]
    Define,

    #[token("func")]
    Func,

    #[token("args")]
    Args,

    #[token("locals")]
    Locals,

    // Instructions
    #[token("halt")]
    Halt,

    #[token("nop")]
    Noop,

    #[token("add")]
    Add,

    #[token("sub")]
    Sub,

    #[token("mul")]
    Mul,

    #[token("div")]
    Div,

    #[token("mod")]
    Mod,

    #[token("ldconst")]
    LoadConst,

    #[token("stlocal")]
    StoreLocal,

    #[token("ldlocal")]
    LoadLocal,

    #[token("pop")]
    Pop,

    #[token("testeq")]
    TestEqual,

    #[token("testne")]
    TestNotEqual,

    #[token("testgt")]
    TestGreaterThan,

    #[token("testlt")]
    TestLessThan,

    #[token("jmp")]
    Jump,

    #[token("jmpt")]
    JmpTrue,

    #[token("jmpf")]
    JmpFalse,

    #[token("callfunc")]
    CallFunction,

    #[token("callvirt")]
    CallVirtual,

    #[token("ret")]
    Return,

    #[token("newstruct")]
    NewStruct,

    #[token("ldfield")]
    LoadField,

    #[token("stfield")]
    StoreField,

    #[token("newarray")]
    NewArray,

    // Types
    #[token("void")]
    VoidType,

    #[token("addr")]
    AddressType,

    #[token("i8")]
    Integer8Type,

    #[token("i16")]
    Integer16Type,

    #[token("i32")]
    Integer32Type,

    #[token("i64")]
    Integer64Type,

    #[token("u8")]
    UInteger8Type,

    #[token("u16")]
    UInteger16Type,

    #[token("u32")]
    UInteger32Type,

    #[token("u64")]
    UInteger64Type,

    #[token("f32")]
    Float32Type,

    #[token("f64")]
    Float64Type,

    #[token("str")]
    StringType,

    // Operators
    #[token("=")]
    Equal,

    #[token(":")]
    Colon,

    // Literals
    #[regex(r"0x[0-9a-fA-F]+", from_hex_string)]
    #[regex(r"0b[01]+", from_bin_string)]
    #[regex(r"[1-9][0-9]*", |lex| lex.slice().parse())]
    #[token("0", |lex| lex.slice().parse())]
    Integer(u64),

    #[regex(r"0\.[0-9]+", |lex| lex.slice().parse())]
    #[token("0.0", |lex| lex.slice().parse())]
    #[regex(r"[1-9][0-9]*\.[0-9]+", |lex| lex.slice().parse())]
    Float(f64),

    #[regex(r#""[^\n\r"]*""#, |lex| lex.slice().trim_matches('"').to_string())]
    String(String),

    #[error]
    #[regex(r"[ \t\r\n\f]+", logos::skip)]
    Error
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn halt_instruction() {
        let mut lexer = lex_string("halt");
        let result = lexer.next();

        assert_eq!(Some(Token::Halt), result);
    }

    #[test]
    fn nop_instruction() {
        let mut lexer = lex_string("nop");
        let result = lexer.next();

        assert_eq!(Some(Token::Noop), result);
    }

    #[test]
    fn add_instruction() {
        let mut lexer = lex_string("add");
        let result = lexer.next();

        assert_eq!(Some(Token::Add), result);
    }

    #[test]
    fn sub_instruction() {
        let mut lexer = lex_string("sub");
        let result = lexer.next();

        assert_eq!(Some(Token::Sub), result);
    }

    #[test]
    fn mul_instruction() {
        let mut lexer = lex_string("mul");
        let result = lexer.next();

        assert_eq!(Some(Token::Mul), result);
    }

    #[test]
    fn div_instruction() {
        let mut lexer = lex_string("div");
        let result = lexer.next();

        assert_eq!(Some(Token::Div), result);
    }

    #[test]
    fn mod_instruction() {
        let mut lexer = lex_string("mod");
        let result = lexer.next();

        assert_eq!(Some(Token::Mod), result);
    }

    #[test]
    fn ldconst_instruction() {
        let mut lexer = lex_string("ldconst");
        let result = lexer.next();

        assert_eq!(Some(Token::LoadConst), result);
    }

    #[test]
    fn ldlocal_instruction() {
        let mut lexer = lex_string("ldlocal");
        let result = lexer.next();

        assert_eq!(Some(Token::LoadLocal), result);
    }

    #[test]
    fn stlocal_instruction() {
        let mut lexer = lex_string("stlocal");
        let result = lexer.next();

        assert_eq!(Some(Token::StoreLocal), result);
    }

    #[test]
    fn pop_instruction() {
        let mut lexer = lex_string("pop");
        let result = lexer.next();

        assert_eq!(Some(Token::Pop), result);
    }

    #[test]
    fn testeq_instruction() {
        let mut lexer = lex_string("testeq");
        let result = lexer.next();

        assert_eq!(Some(Token::TestEqual), result);
    }

    #[test]
    fn testne_instruction() {
        let mut lexer = lex_string("testne");
        let result = lexer.next();

        assert_eq!(Some(Token::TestNotEqual), result);
    }

    #[test]
    fn testgt_instruction() {
        let mut lexer = lex_string("testgt");
        let result = lexer.next();

        assert_eq!(Some(Token::TestGreaterThan), result);
    }

    #[test]
    fn testlt_instruction() {
        let mut lexer = lex_string("testlt");
        let result = lexer.next();

        assert_eq!(Some(Token::TestLessThan), result);
    }

    #[test]
    fn jmp_instruction() {
        let mut lexer = lex_string("jmp");
        let result = lexer.next();

        assert_eq!(Some(Token::Jump), result);
    }

    #[test]
    fn jmpt_instruction() {
        let mut lexer = lex_string("jmpt");
        let result = lexer.next();

        assert_eq!(Some(Token::JmpTrue), result);
    }

    #[test]
    fn jmpf_instruction() {
        let mut lexer = lex_string("jmpf");
        let result = lexer.next();

        assert_eq!(Some(Token::JmpFalse), result);
    }

    #[test]
    fn callfunc_instruction() {
        let mut lexer = lex_string("callfunc");
        let result = lexer.next();

        assert_eq!(Some(Token::CallFunction), result);
    }

    #[test]
    fn callvirt_instruction() {
        let mut lexer = lex_string("callvirt");
        let result = lexer.next();

        assert_eq!(Some(Token::CallVirtual), result);
    }

    #[test]
    fn ret_instruction() {
        let mut lexer = lex_string("ret");
        let result = lexer.next();

        assert_eq!(Some(Token::Return), result);
    }

    #[test]
    fn newstruct_instruction() {
        let mut lexer = lex_string("newstruct");
        let result = lexer.next();

        assert_eq!(Some(Token::NewStruct), result);
    }

    #[test]
    fn ldfield_instruction() {
        let mut lexer = lex_string("ldfield");
        let result = lexer.next();

        assert_eq!(Some(Token::LoadField), result);
    }

    #[test]
    fn stfield_instruction() {
        let mut lexer = lex_string("stfield");
        let result = lexer.next();

        assert_eq!(Some(Token::StoreField), result);
    }

    #[test]
    fn newarray_instruction() {
        let mut lexer = lex_string("newarray");
        let result = lexer.next();

        assert_eq!(Some(Token::NewArray), result);
    }

    #[test]
    fn void_type() {
        let mut lexer = lex_string("void");
        let result = lexer.next();

        assert_eq!(Some(Token::VoidType), result);
    }

    #[test]
    fn addr_type() {
        let mut lexer = lex_string("addr");
        let result = lexer.next();

        assert_eq!(Some(Token::AddressType), result);
    }

    #[test]
    fn i8_type() {
        let mut lexer = lex_string("i8");
        let result = lexer.next();

        assert_eq!(Some(Token::Integer8Type), result);
    }

    #[test]
    fn i16_type() {
        let mut lexer = lex_string("i16");
        let result = lexer.next();

        assert_eq!(Some(Token::Integer16Type), result);
    }

    #[test]
    fn i32_type() {
        let mut lexer = lex_string("i32");
        let result = lexer.next();

        assert_eq!(Some(Token::Integer32Type), result);
    }

    #[test]
    fn i64_type() {
        let mut lexer = lex_string("i64");
        let result = lexer.next();

        assert_eq!(Some(Token::Integer64Type), result);
    }

    #[test]
    fn u8_type() {
        let mut lexer = lex_string("u8");
        let result = lexer.next();

        assert_eq!(Some(Token::UInteger8Type), result);
    }

    #[test]
    fn u16_type() {
        let mut lexer = lex_string("u16");
        let result = lexer.next();

        assert_eq!(Some(Token::UInteger16Type), result);
    }

    #[test]
    fn u32_type() {
        let mut lexer = lex_string("u32");
        let result = lexer.next();

        assert_eq!(Some(Token::UInteger32Type), result);
    }

    #[test]
    fn u64_type() {
        let mut lexer = lex_string("u64");
        let result = lexer.next();

        assert_eq!(Some(Token::UInteger64Type), result);
    }

    #[test]
    fn f32_type() {
        let mut lexer = lex_string("f32");
        let result = lexer.next();

        assert_eq!(Some(Token::Float32Type), result);
    }

    #[test]
    fn f64_type() {
        let mut lexer = lex_string("f64");
        let result = lexer.next();

        assert_eq!(Some(Token::Float64Type), result);
    }

    #[test]
    fn str_type() {
        let mut lexer = lex_string("str");
        let result = lexer.next();

        assert_eq!(Some(Token::StringType), result);
    }

    #[test]
    fn equal_operator() {
        let mut lexer = lex_string("=");
        let result = lexer.next();

        assert_eq!(Some(Token::Equal), result);
    }

    #[test]
    fn colon_operator() {
        let mut lexer = lex_string(":");
        let result = lexer.next();

        assert_eq!(Some(Token::Colon), result);
    }

    #[test]
    fn zero_integer() {
        let mut lexer = lex_string("0");
        let result = lexer.next();

        assert_eq!(Some(Token::Integer(0)), result);
    }

    #[test]
    fn non_zero_integer() {
        let mut lexer = lex_string("42");
        let result = lexer.next();

        assert_eq!(Some(Token::Integer(42)), result);
    }

    #[test]
    fn hex_integer() {
        let mut lexer = lex_string("0x2a");
        let result = lexer.next();

        assert_eq!(Some(Token::Integer(42)), result);
    }

    #[test]
    fn bin_integer() {
        let mut lexer = lex_string("0b101010");
        let result = lexer.next();

        assert_eq!(Some(Token::Integer(42)), result);
    }

    #[test]
    fn zero_float() {
        let mut lexer = lex_string("0.0");
        let result = lexer.next();

        assert_eq!(Some(Token::Float(0.0)), result);
    }

    #[test]
    fn non_zero_float() {
        let mut lexer = lex_string("4.2");
        let result = lexer.next();

        assert_eq!(Some(Token::Float(4.2)), result);
    }

    #[test]
    fn string_literal() {
        let mut lexer = lex_string(r#""String value""#);
        let result = lexer.next();

        assert_eq!(Some(Token::String("String value".to_string())), result);
    }

    #[test]
    fn skip_whitespace() {
        let mut lexer = lex_string(" \t\r\n\x0c42");
        let result = lexer.next();

        assert_eq!(Some(Token::Integer(42)), result);
    }
}
