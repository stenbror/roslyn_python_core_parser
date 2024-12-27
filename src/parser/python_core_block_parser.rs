use crate::parser::syntax_error::SyntaxError;
use crate::parser::syntax_nodes::SyntaxNode;
use super::python_core_parser::PythonCoreParser;

pub trait BlockGrammarRules {
    fn parse_var_args_list_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;

    fn parse_decorated_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_class_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_func_def_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
}

impl BlockGrammarRules for PythonCoreParser {
    fn parse_var_args_list_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_decorated_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_class_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_func_def_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }
}