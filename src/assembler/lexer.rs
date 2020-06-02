use logos::{Lexer, Logos};

pub fn lex_string(source: &str) -> Lexer<Token> {
    return Token::lexer(source);
}

#[derive(Logos, Debug, PartialEq)]
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
    #[regex(r"0x[0-9a-fA-F]+")]
    #[regex(r"0b[01]+")]
    #[regex(r"[1-9][0-9]*")]
    #[token("0")]
    Integer,

    #[regex(r"0\.[0-9]+")]
    #[token("0.0")]
    #[regex(r"[1-9][0-9]*\.[0-9]+")]
    Float,

    #[regex(r#""[^\n\r"]*""#)]
    String,

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
}
