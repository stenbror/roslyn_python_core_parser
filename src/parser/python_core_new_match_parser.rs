use crate::parser::python_core_parser::PythonCoreParser;
use crate::parser::syntax_error::SyntaxError;
use crate::parser::syntax_nodes::SyntaxNode;



// Refactor match parser to a LL(1) parser. New rules not based on Python grammar 3.13 but will parse equal 3.13





pub(crate) trait MatchPatternRulesNew {
    fn parse_match_stmt_new(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
}

impl MatchPatternRulesNew for PythonCoreParser {
    fn parse_match_stmt_new(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        unimplemented!()
    }
}