use crate::parser::python_core_statement_parser::StatementRules;
use crate::parser::python_core_tokenizer::LexerMethods;
use crate::parser::syntax_error::SyntaxError;
use crate::parser::syntax_nodes::SyntaxNode;
use crate::parser::syntax_nodes::SyntaxNode::{CompareEqualExprNode, CompareGreaterEqualExprNode, CompareGreaterExprNode, CompareInEqualExprNode, CompareIsExprNode, CompareIsNotExprNode, CompareLessEqualExprNode, CompareLessExprNode, CompareNotEqualExprNode, CompareNotInExprNode, LambdaExprNode, MulExprNode, NamedExprNode, NotTestExprNode, OrExprNode, OrTestExprNode, StarExprNode, TestExprNode, XorExprNode};
use crate::parser::token_nodes::Token;
use super::python_core_parser::PythonCoreParser;


// Trait for expression grammar rule ///////////////////////////////////////////////////////////////
trait ExpressionRules {
    fn parse_named_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_test_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_test_no_cond_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_lambda_def_expr(&mut self, is_conditional: bool) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_or_test_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_and_test_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_not_test_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_comparison_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_star_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_xor_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_and_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_shift_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_arith_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_term_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_factor_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_power_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_await_atom_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_atom_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_test_list_comp_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_trailer_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_subscript_list_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_subscript_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_expr_list_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_test_list_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_dictionary_set_maker_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_arg_list_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_argument_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_comp_iter_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_sync_comp_for_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_comp_for_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_comp_if_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_yield_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
}

// Implementing all expression grammar rules ///////////////////////////////////////////////////////
impl ExpressionRules for PythonCoreParser {
    fn parse_named_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let left = self.parse_expr()?;

        match &*self.lexer.symbol {
            Token::ColonAssignToken( _ , _ , _ ) => {
                let symbol = self.lexer.symbol.clone();
                self.lexer.advance();

                let right = self.parse_expr()?;
                Ok(Box::new(NamedExprNode(pos, self.lexer.position, left, symbol, right)))
            },
            _ => {
                Ok(left)
            }
        }
    }

    fn parse_test_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        match &*self.lexer.symbol {
            Token::LambdaToken( _ , _ , _ ) => {
                self.parse_lambda_def_expr(true)
            },
            _ => {
                let pos = self.lexer.position;
                let left = self.parse_or_test_expr()?;

                match &*self.lexer.symbol {
                    Token::IfToken( _ , _ , _ ) => {
                        let symbol1 = self.lexer.symbol.clone();
                        self.lexer.advance();
                        let right = self.parse_or_test_expr()?;
                        match &*self.lexer.symbol {
                            Token::ElseToken( _ , _ , _ ) => {
                                let symbol2 = self.lexer.symbol.clone();
                                self.lexer.advance();
                                let next = self.parse_test_expr()?;
                                Ok(Box::new(TestExprNode(pos, self.lexer.position, left, symbol1, right, symbol2, next)))
                            },
                            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting 'else' in test expression!"))))
                        }
                    },
                    _ => Ok(left)
                }
            }
        }
    }

    fn parse_test_no_cond_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        match &*self.lexer.symbol {
            Token::LambdaToken( _ , _ , _ ) => {
                self.parse_lambda_def_expr(false)
            },
            _ => self.parse_or_test_expr()
        }
    }

    fn parse_lambda_def_expr(&mut self, is_conditional: bool) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol1 = self.lexer.symbol.clone();
        self.lexer.advance();

        let left = match &*self.lexer.symbol {
            Token::ColonToken( _ , _ , _ ) => {
                None
            },
            _ => Some(self.parse_var_args_list_stmt()?)
        };

        match &*self.lexer.symbol {
            Token::ColonToken( _ , _ , _ ) => {
                let symbol2 = self.lexer.symbol.clone();
                self.lexer.advance();
                let right = match is_conditional {
                    true => {
                        self.parse_test_expr()?
                    },
                    _ => self.parse_test_no_cond_expr()?
                };

                Ok(Box::new(LambdaExprNode(pos, self.lexer.position, symbol1, left, symbol2, right, is_conditional)))
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting ':' in lambda expression!"))))
        }
    }

    fn parse_or_test_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut left = self.parse_and_test_expr()?;

        loop {
            match &*self.lexer.symbol {
                Token::OrToken( _ , _ , _ ) => {
                    let symbol1 = self.lexer.symbol.clone();
                    self.lexer.advance();

                    let right = self.parse_and_test_expr()?;

                    left = Box::new(OrTestExprNode(pos, self.lexer.position, left, symbol1, right));
                },
                _ => break
            }
        }

        Ok(left)
    }

    fn parse_and_test_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut left = self.parse_not_test_expr()?;

        loop {
            match &*self.lexer.symbol {
                Token::AndToken( _ , _ , _ ) => {
                    let symbol1 = self.lexer.symbol.clone();
                    self.lexer.advance();

                    let right = self.parse_not_test_expr()?;

                    left = Box::new(OrTestExprNode(pos, self.lexer.position, left, symbol1, right));
                },
                _ => break
            }
        }

        Ok(left)
    }

    fn parse_not_test_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;

        match &*self.lexer.symbol {
            Token::NotToken( _ , _ , _ ) => {
                let symbol1 = self.lexer.symbol.clone();
                self.lexer.advance();

                let right = self.parse_not_test_expr()?;
                Ok(Box::new(NotTestExprNode(pos, self.lexer.position, symbol1, right)))
            },
            _ => self.parse_comparison_expr()
        }
    }

    fn parse_comparison_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut left = self.parse_expr()?;

        loop {
            match &*self.lexer.symbol {
                Token::LessThanToken( _ , _ , _ ) |
                Token::LessOrEqualToken( _ , _ , _ ) |
                Token::EqualToken( _ , _ , _ ) |
                Token::GreaterOrEqualToken( _ , _ , _ ) |
                Token::GreaterThanToken( _ , _ , _ ) |
                Token::NotEqualToken( _ , _ , _ ) |
                Token::InToken( _ , _ , _ ) => {
                    let symbol1 = self.lexer.symbol.clone();
                    self.lexer.advance();

                    let right = self.parse_expr()?;

                    left = Box::new(match &*self.lexer.symbol {
                        Token::LessThanToken( _ , _ , _ ) => CompareLessExprNode(pos, self.lexer.position, left, symbol1, right),
                        Token::LessOrEqualToken( _ , _ , _ ) => CompareLessEqualExprNode(pos, self.lexer.position, left, symbol1, right),
                        Token::EqualToken( _ , _ , _ ) => CompareEqualExprNode(pos, self.lexer.position, left, symbol1, right),
                        Token::GreaterOrEqualToken( _ , _ , _ ) => CompareGreaterEqualExprNode(pos, self.lexer.position, left, symbol1, right),
                        Token::GreaterThanToken( _ , _ , _ ) => CompareGreaterExprNode(pos, self.lexer.position, left, symbol1, right),
                        Token::InToken( _ , _ , _ ) => CompareInEqualExprNode(pos, self.lexer.position, left, symbol1, right),
                        _ => CompareNotEqualExprNode(pos, self.lexer.position, left, symbol1, right)
                    })
                },
                Token::IsToken( _ , _ , _ ) => {
                    let symbol1 = self.lexer.symbol.clone();
                    self.lexer.advance();

                    match &*self.lexer.symbol {
                        Token::NotToken( _ , _ , _ ) => {
                            let symbol2 = self.lexer.symbol.clone();
                            self.lexer.advance();

                            let right = self.parse_expr()?;

                            left = Box::new(CompareIsNotExprNode(pos, self.lexer.position, left, symbol1, symbol2, right))
                        },
                        _ => {
                            let right = self.parse_expr()?;
                            left = Box::new(CompareIsExprNode(pos, self.lexer.position, left, symbol1, right))
                        }
                    }
                },
                Token::NotToken( _ , _ , _ ) => {
                    let symbol1 = self.lexer.symbol.clone();
                    self.lexer.advance();

                    match &*self.lexer.symbol {
                        Token::InToken( _ , _ , _ ) => {
                            let symbol2 = self.lexer.symbol.clone();
                            self.lexer.advance();

                            let right = self.parse_expr()?;

                            left = Box::new(CompareNotInExprNode(pos, self.lexer.position, left, symbol1, symbol2, right))
                        },
                        _ => return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting 'in' in 'not in' compare expression!"))))
                    }
                }
                _ => break
            }
        }

        Ok(left)
    }

    fn parse_star_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol1 = self.lexer.symbol.clone();
        self.lexer.advance();

        let right = self.parse_expr()?;

        Ok(Box::new(StarExprNode(pos, self.lexer.position, symbol1, right)))
    }

    fn parse_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut left = self.parse_xor_expr()?;

        loop {
            match &*self.lexer.symbol {
                Token::BitOrToken( _ , _ , _ ) => {
                    let symbol1 = self.lexer.symbol.clone();
                    self.lexer.advance();

                    let right = self.parse_xor_expr()?;

                    left = Box::new(OrExprNode(pos, self.lexer.position, left, symbol1, right));
                },
                _ => break
            }
        }

        Ok(left)
    }

    fn parse_xor_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut left = self.parse_and_expr()?;

        loop {
            match &*self.lexer.symbol {
                Token::BitXorToken( _ , _ , _ ) => {
                    let symbol1 = self.lexer.symbol.clone();
                    self.lexer.advance();

                    let right = self.parse_and_expr()?;

                    left = Box::new(XorExprNode(pos, self.lexer.position, left, symbol1, right));
                },
                _ => break
            }
        }

        Ok(left)
    }

    fn parse_and_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_shift_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_arith_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_term_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_factor_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_power_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_await_atom_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_atom_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_test_list_comp_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_trailer_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_subscript_list_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_subscript_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_expr_list_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_test_list_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_dictionary_set_maker_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_arg_list_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_argument_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_comp_iter_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_sync_comp_for_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_comp_for_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_comp_if_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_yield_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
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