// auto-generated: "lalrpop 0.19.0"
// sha256: 5d7892ea18743d22727a2abc3b1f79e74492311e6aee34f2a1248a7bb6cd11f
use super::lexer::{LexError, Token};
use super::parse_tree;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Term {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use super::super::lexer::{LexError, Token};
    use super::super::parse_tree;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<>
     {
        Variant0(Token),
        Variant1(u64),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 48 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -2,
        // State 2
        -1,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            0 => 1,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> Vec<::std::string::String> {
        const __TERMINAL: &[&str] = &[
            r###"":""###,
            r###""=""###,
            r###""add""###,
            r###""addr""###,
            r###""args""###,
            r###""callfunc""###,
            r###""callvirt""###,
            r###""define""###,
            r###""div""###,
            r###""f32""###,
            r###""f64""###,
            r###""func""###,
            r###""halt""###,
            r###""i16""###,
            r###""i32""###,
            r###""i64""###,
            r###""i8""###,
            r###""jmp""###,
            r###""jmpf""###,
            r###""jmpt""###,
            r###""ldconst""###,
            r###""ldfield""###,
            r###""ldlocal""###,
            r###""locals""###,
            r###""mod""###,
            r###""mul""###,
            r###""newarray""###,
            r###""newstruct""###,
            r###""nop""###,
            r###""pop""###,
            r###""ret""###,
            r###""stfield""###,
            r###""stlocal""###,
            r###""str""###,
            r###""sub""###,
            r###""testeq""###,
            r###""testgt""###,
            r###""testlt""###,
            r###""testne""###,
            r###""u16""###,
            r###""u32""###,
            r###""u64""###,
            r###""u8""###,
            r###""void""###,
            r###"Error"###,
            r###"FloatLit"###,
            r###"IntegerLit"###,
            r###"StringLit"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct __StateMachine<>
    where 
    {
        __phantom: ::std::marker::PhantomData<()>,
    }
    impl<> __state_machine::ParserDefinition for __StateMachine<>
    where 
    {
        type Location = usize;
        type Error = LexError;
        type Token = Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = u64;
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
            __token_to_integer(token, ::std::marker::PhantomData::<()>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 48 - 1)
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
            __token_to_symbol(token_index, token, ::std::marker::PhantomData::<()>)
        }

        fn expected_tokens(&self, state: i8) -> Vec<String> {
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
            states: &mut Vec<i8>,
            symbols: &mut Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                action,
                start_location,
                states,
                symbols,
                ::std::marker::PhantomData::<()>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
    >(
        __token: &Token,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<usize>
    {
        match *__token {
            Token::Colon if true => Some(0),
            Token::Equal if true => Some(1),
            Token::Add if true => Some(2),
            Token::AddressType if true => Some(3),
            Token::Args if true => Some(4),
            Token::CallFunction if true => Some(5),
            Token::CallVirtual if true => Some(6),
            Token::Define if true => Some(7),
            Token::Div if true => Some(8),
            Token::Float32Type if true => Some(9),
            Token::Float64Type if true => Some(10),
            Token::Func if true => Some(11),
            Token::Halt if true => Some(12),
            Token::Integer16Type if true => Some(13),
            Token::Integer32Type if true => Some(14),
            Token::Integer64Type if true => Some(15),
            Token::Integer8Type if true => Some(16),
            Token::Jump if true => Some(17),
            Token::JmpFalse if true => Some(18),
            Token::JmpTrue if true => Some(19),
            Token::LoadConst if true => Some(20),
            Token::LoadField if true => Some(21),
            Token::LoadLocal if true => Some(22),
            Token::Locals if true => Some(23),
            Token::Mod if true => Some(24),
            Token::Mul if true => Some(25),
            Token::NewArray if true => Some(26),
            Token::NewStruct if true => Some(27),
            Token::Noop if true => Some(28),
            Token::Pop if true => Some(29),
            Token::Return if true => Some(30),
            Token::StoreField if true => Some(31),
            Token::StoreLocal if true => Some(32),
            Token::StringType if true => Some(33),
            Token::Sub if true => Some(34),
            Token::TestEqual if true => Some(35),
            Token::TestGreaterThan if true => Some(36),
            Token::TestLessThan if true => Some(37),
            Token::TestNotEqual if true => Some(38),
            Token::UInteger16Type if true => Some(39),
            Token::UInteger32Type if true => Some(40),
            Token::UInteger64Type if true => Some(41),
            Token::UInteger8Type if true => Some(42),
            Token::VoidType if true => Some(43),
            Token::Error if true => Some(44),
            Token::Float(f64) if true => Some(45),
            Token::Integer(u64) if true => Some(46),
            Token::String(String) if true => Some(47),
            _ => None,
        }
    }
    fn __token_to_symbol<
    >(
        __token_index: usize,
        __token: Token,
        _: ::std::marker::PhantomData<()>,
    ) -> __Symbol<>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 => __Symbol::Variant0(__token),
            _ => unreachable!(),
        }
    }
    pub struct TermParser {
        _priv: (),
    }

    impl TermParser {
        pub fn new() -> TermParser {
            TermParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            __TOKEN: __ToTriple<>,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<u64, __lalrpop_util::ParseError<usize, Token, LexError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    __phantom: ::std::marker::PhantomData::<()>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
    >(
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<u64,__lalrpop_util::ParseError<usize, Token, LexError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(__lookahead_start, __symbols, ::std::marker::PhantomData::<()>)
            }
            1 => {
                // __Term = Term => ActionFn(0);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
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
    fn __pop_Variant0<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, Token, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>
    ) -> (usize, u64, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
    >(
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> (usize, usize)
    {
        // Term = IntegerLit => ActionFn(1);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 0)
    }
}
pub use self::__parse__Term::TermParser;

fn __action0<
>(
    (_, __0, _): (usize, u64, usize),
) -> u64
{
    __0
}

fn __action1<
>(
    (_, __0, _): (usize, Token, usize),
) -> u64
{
    parse_tree::Literal::UInteger64(__0 as u64)
}

pub trait __ToTriple<> {
    fn to_triple(value: Self) -> Result<(usize,Token,usize), __lalrpop_util::ParseError<usize, Token, LexError>>;
}

impl<> __ToTriple<> for (usize, Token, usize) {
    fn to_triple(value: Self) -> Result<(usize,Token,usize), __lalrpop_util::ParseError<usize, Token, LexError>> {
        Ok(value)
    }
}
impl<> __ToTriple<> for Result<(usize, Token, usize), LexError> {
    fn to_triple(value: Self) -> Result<(usize,Token,usize), __lalrpop_util::ParseError<usize, Token, LexError>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
