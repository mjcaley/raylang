mod assembler;
use assembler::lexer;

fn main() {
    let mut lex = lexer::lex_string("halt 0 123 0.0 123.123 \"This is a string\"");
    let halt = lex.next();
    println!("{:?} - {}", halt.unwrap(), lex.slice());

    let zero = lex.next();
    println!("{:?} - {}", zero.unwrap(), lex.slice());

    let n123 = lex.next();
    println!("{:?} - {}", n123.unwrap(), lex.slice());

    let n123 = lex.next();
    println!("{:?} - {}", n123.unwrap(), lex.slice());

    let n123 = lex.next();
    println!("{:?} - {}", n123.unwrap(), lex.slice());

    let n123 = lex.next();
    println!("{:?} - {}", n123.unwrap(), lex.slice());
}
