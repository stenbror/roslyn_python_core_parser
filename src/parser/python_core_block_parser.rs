use crate::parser::syntax_error::SyntaxError;
use crate::parser::syntax_nodes::SyntaxNode;
use crate::parser::token_nodes::Token;
use super::python_core_parser::PythonCoreParser;

pub trait BlockGrammarRules {
    fn parse_single_input(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_file_input(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_eval_input(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_decorator_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_decorators_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_decorated_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_class_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_func_def_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_parameters_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_typed_args_list_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_tfp_def(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_var_args_list_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_func_body_suite_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_func_type_input(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_func_type(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_type_list(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
}

impl BlockGrammarRules for PythonCoreParser {
    fn parse_single_input(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_file_input(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_eval_input(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_decorator_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_decorators_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
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

    fn parse_parameters_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_typed_args_list_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_tfp_def(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_var_args_list_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_func_body_suite_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_func_type_input(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let right = self.parse_func_type()?;

        let mut nodes = Vec::<Box<Token>>::new();

        loop {
            match &*self.lexer.symbol {
                Token::NewlineToken( _ , _ , _ , _ , _ ) => {
                    nodes.push(self.lexer.symbol.clone())
                },_ => break
            }
        }

        nodes.reverse();

        match &*self.lexer.symbol {
            Token::EofToken( _ , _ , _ ) => Ok(Box::new(SyntaxNode::FuncTypeInputStmtNode(pos, self.lexer.position, right, nodes, self.lexer.symbol.clone()))),
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting EOF in type input!"))))
        }
    }

    fn parse_func_type(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_type_list(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }
}

// Unittests for expression grammar rules //////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    #[test]
    fn dummy_test() {
        assert_eq!(1 == 1, true);
    }
}
