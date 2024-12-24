use crate::parser::token_nodes::Token::Invalid;
use super::token_nodes::Token;

pub trait LexerMethods {
    fn new() -> PythonCoreTokenizer;
    fn advance(&mut self) -> ();
}

pub struct PythonCoreTokenizer
{
    pub symbol: Box<Token>,
    pub position: u32,
}

impl LexerMethods for PythonCoreTokenizer {
    fn new() -> PythonCoreTokenizer {
        PythonCoreTokenizer {
            symbol: Box::new(Invalid),
            position: 0,
        }
    }

    fn advance(&mut self) {

    }
}