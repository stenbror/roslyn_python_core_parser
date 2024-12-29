
use crate::parser::python_core_tokenizer::{LexerMethods, PythonCoreTokenizer};


pub struct PythonCoreParser
{
    pub lexer: Box<PythonCoreTokenizer>
}

impl PythonCoreParser {
    pub(crate) fn new() -> PythonCoreParser { PythonCoreParser{
        lexer: Box::new(PythonCoreTokenizer::new())
    } }
}
