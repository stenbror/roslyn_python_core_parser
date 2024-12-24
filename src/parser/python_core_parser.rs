use crate::parser::python_core_tokenizer::{LexerMethods, PythonCoreTokenizer};

pub struct PythonCoreParser
{
    pub lexer: Box<PythonCoreTokenizer>
}

impl PythonCoreParser {
    pub fn new() -> PythonCoreParser { PythonCoreParser{
        lexer: Box::new(PythonCoreTokenizer::new())
    } }
}