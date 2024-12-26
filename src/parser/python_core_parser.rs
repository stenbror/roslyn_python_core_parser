use crate::parser::python_core_tokenizer::{LexerMethods, PythonCoreTokenizer};
use crate::parser::syntax_error::SyntaxError;
use crate::parser::syntax_nodes::SyntaxNode;

pub trait BlockGrammarRules {
    fn new() -> PythonCoreParser;

    fn parse_var_args_list_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
}

pub struct PythonCoreParser
{
    pub lexer: Box<PythonCoreTokenizer>
}

impl BlockGrammarRules for PythonCoreParser {
    fn new() -> PythonCoreParser { PythonCoreParser{
        lexer: Box::new(PythonCoreTokenizer::new())
    } }

    fn parse_var_args_list_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }
}