use crate::parser::syntax_error::SyntaxError;
use crate::parser::syntax_nodes::SyntaxNode;
use super::python_core_parser::PythonCoreParser;

pub trait MatchPatternRules {
    fn parse_match_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
}

impl MatchPatternRules for PythonCoreParser {
    fn parse_match_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }
}