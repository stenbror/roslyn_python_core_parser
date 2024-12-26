use crate::parser::python_core_statement_parser::StatementRules;
use crate::parser::python_core_tokenizer::LexerMethods;
use crate::parser::syntax_error::SyntaxError;
use crate::parser::syntax_nodes::SyntaxNode;
use crate::parser::token_nodes::Token;
use super::python_core_parser::{BlockGrammarRules, PythonCoreParser};


// Trait for expression grammar rule ///////////////////////////////////////////////////////////////
pub(crate) trait ExpressionRules {
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
    fn parse_subscript_list_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_subscript_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_expr_list_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_test_list_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_dictionary_set_maker_expr(&mut self, symbol1: Box<Token>, position: u32) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
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
                Ok(Box::new(SyntaxNode::NamedExprNode(pos, self.lexer.position, left, symbol, right)))
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
                                Ok(Box::new(SyntaxNode::TestExprNode(pos, self.lexer.position, left, symbol1, right, symbol2, next)))
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

                Ok(Box::new(SyntaxNode::LambdaExprNode(pos, self.lexer.position, symbol1, left, symbol2, right, is_conditional)))
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

                    left = Box::new(SyntaxNode::OrTestExprNode(pos, self.lexer.position, left, symbol1, right));
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

                    left = Box::new(SyntaxNode::OrTestExprNode(pos, self.lexer.position, left, symbol1, right));
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
                Ok(Box::new(SyntaxNode::NotTestExprNode(pos, self.lexer.position, symbol1, right)))
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
                        Token::LessThanToken( _ , _ , _ ) => SyntaxNode::CompareLessExprNode(pos, self.lexer.position, left, symbol1, right),
                        Token::LessOrEqualToken( _ , _ , _ ) => SyntaxNode::CompareLessEqualExprNode(pos, self.lexer.position, left, symbol1, right),
                        Token::EqualToken( _ , _ , _ ) => SyntaxNode::CompareEqualExprNode(pos, self.lexer.position, left, symbol1, right),
                        Token::GreaterOrEqualToken( _ , _ , _ ) => SyntaxNode::CompareGreaterEqualExprNode(pos, self.lexer.position, left, symbol1, right),
                        Token::GreaterThanToken( _ , _ , _ ) => SyntaxNode::CompareGreaterExprNode(pos, self.lexer.position, left, symbol1, right),
                        Token::InToken( _ , _ , _ ) => SyntaxNode::CompareInEqualExprNode(pos, self.lexer.position, left, symbol1, right),
                        _ => SyntaxNode::CompareNotEqualExprNode(pos, self.lexer.position, left, symbol1, right)
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

                            left = Box::new(SyntaxNode::CompareIsNotExprNode(pos, self.lexer.position, left, symbol1, symbol2, right))
                        },
                        _ => {
                            let right = self.parse_expr()?;
                            left = Box::new(SyntaxNode::CompareIsExprNode(pos, self.lexer.position, left, symbol1, right))
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

                            left = Box::new(SyntaxNode::CompareNotInExprNode(pos, self.lexer.position, left, symbol1, symbol2, right))
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

        Ok(Box::new(SyntaxNode::StarExprNode(pos, self.lexer.position, symbol1, right)))
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

                    left = Box::new(SyntaxNode::OrExprNode(pos, self.lexer.position, left, symbol1, right));
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

                    left = Box::new(SyntaxNode::XorExprNode(pos, self.lexer.position, left, symbol1, right));
                },
                _ => break
            }
        }

        Ok(left)
    }

    fn parse_and_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut left = self.parse_shift_expr()?;

        loop {
            match &*self.lexer.symbol {
                Token::BitAndToken( _ , _ , _ ) => {
                    let symbol1 = self.lexer.symbol.clone();
                    self.lexer.advance();

                    let right = self.parse_shift_expr()?;

                    left = Box::new(SyntaxNode::AndExprNode(pos, self.lexer.position, left, symbol1, right));
                },
                _ => break
            }
        }

        Ok(left)
    }

    fn parse_shift_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut left = self.parse_arith_expr()?;

        loop {
            match &*self.lexer.symbol {
                Token::ShiftLeftToken( _ , _ , _ ) |
                Token::ShiftRightToken( _ , _ , _ ) => {
                    let symbol1 = self.lexer.symbol.clone();
                    self.lexer.advance();

                    let right = self.parse_arith_expr()?;

                    left = Box::new(match &*self.lexer.symbol {
                        Token::ShiftLeftToken( _ , _ , _ ) => SyntaxNode::ShiftLeftExprNode(pos, self.lexer.position, left, symbol1, right),
                        _ => SyntaxNode::ShiftRightExprNode(pos, self.lexer.position, left, symbol1, right)
                    });
                },
                _ => break
            }
        }

        Ok(left)
    }

    fn parse_arith_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut left = self.parse_term_expr()?;

        loop {
            match &*self.lexer.symbol {
                Token::PlusToken( _ , _ , _ ) |
                Token::MinusToken( _ , _ , _ ) => {
                    let symbol1 = self.lexer.symbol.clone();
                    self.lexer.advance();

                    let right = self.parse_term_expr()?;

                    left = Box::new(match &*self.lexer.symbol {
                        Token::PlusToken( _ , _ , _ ) => SyntaxNode::PlusExprNode(pos, self.lexer.position, left, symbol1, right),
                        _ => SyntaxNode::MinusExprNode(pos, self.lexer.position, left, symbol1, right)
                    });
                },
                _ => break
            }
        }

        Ok(left)
    }

    fn parse_term_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut left = self.parse_factor_expr()?;

        loop {
            match &*self.lexer.symbol {
                Token::MultiplyToken( _ , _ , _ ) |
                Token::MatricesToken( _ , _ , _ ) |
                Token::DivideToken( _ , _ , _ ) |
                Token::ModuloToken( _ , _ , _ ) |
                Token::FloorDivideToken( _ , _ , _ ) => {
                    let symbol1 = self.lexer.symbol.clone();
                    self.lexer.advance();

                    let right = self.parse_factor_expr()?;

                    left = Box::new(match &*self.lexer.symbol {
                        Token::MultiplyToken( _ , _ , _ ) => SyntaxNode::MulExprNode(pos, self.lexer.position, left, symbol1, right),
                        Token::MatricesToken( _ , _ , _ ) => SyntaxNode::MatricesExprNode(pos, self.lexer.position, left, symbol1, right),
                        Token::DivideToken( _ , _ , _ ) => SyntaxNode::DivExprNode(pos, self.lexer.position, left, symbol1, right),
                        Token::ModuloToken( _ , _ , _ ) => SyntaxNode::ModuloExprNode(pos, self.lexer.position, left, symbol1, right),
                        _ => SyntaxNode::FloorDivExprNode(pos, self.lexer.position, left, symbol1, right)
                    });
                },
                _ => break
            }
        }

        Ok(left)
    }

    fn parse_factor_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        match &*self.lexer.symbol {
            Token::PlusToken( _ , _ , _ ) |
            Token::MinusToken( _ , _ , _ ) |
            Token::BitInvertToken( _ , _ , _ ) => {
                let pos = self.lexer.position;
                let symbol1 = self.lexer.symbol.clone();
                self.lexer.advance();

                let right = self.parse_factor_expr()?;

                Ok(Box::new(match &*self.lexer.symbol {
                    Token::PlusToken( _ , _ , _ ) => SyntaxNode::UnaryPlusExprNode(pos, self.lexer.position, symbol1, right),
                    Token::MinusToken( _ , _ , _ ) => SyntaxNode::UnaryMinusExprNode(pos, self.lexer.position, symbol1, right),
                    _ => SyntaxNode::UnaryBitInvertExprNode(pos, self.lexer.position, symbol1, right),
                }))
            },
            _ => self.parse_power_expr()
        }
    }

    fn parse_power_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let left = self.parse_await_atom_expr()?;

        match &*self.lexer.symbol {
            Token::PowerToken( _ , _ , _ ) => {
                let symbol1 = self.lexer.symbol.clone();
                self.lexer.advance();
                let right = self.parse_await_atom_expr()?;

                Ok(Box::new(SyntaxNode::PowerExprNode(pos, self.lexer.position, left, symbol1, right)))
            }
            _ => Ok(left)
        }
    }

    fn parse_await_atom_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol1 = match &*self.lexer.symbol {
            Token::AwaitToken( _ , _ , _ ) => {
                let symbol1 = self.lexer.symbol.clone();
                self.lexer.advance();
                Some(symbol1)
            },
            _ => None
        };
        let right = self.parse_atom_expr()?;
        let mut trailers = Vec::<Box<SyntaxNode>>::new();

        loop {
            match &*self.lexer.symbol {
                Token::LeftParenToken( _ , _ , _ ) => {
                    let symbol2 = self.lexer.symbol.clone();
                    self.lexer.advance();

                    let next = match &*self.lexer.symbol {
                        Token::RightParenToken( _ , _ , _ ) => None,
                        _ => Some(self.parse_arg_list_expr()?)
                    };

                    match &*self.lexer.symbol {
                        Token::RightParenToken( _ , _ , _ ) => {
                            let symbol3 = self.lexer.symbol.clone();
                            self.lexer.advance();

                            trailers.push(Box::new(SyntaxNode::TrailerCallExprNode(pos, self.lexer.position, symbol2, next, symbol3)))
                        },
                        _ => return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting ')' in call trailer!"))))
                    }
                },
                Token::LeftSquareBracketToken( _ , _ , _ ) => {
                    let symbol2 = self.lexer.symbol.clone();
                    self.lexer.advance();

                    let next = self.parse_subscript_list_expr()?;

                    match &*self.lexer.symbol {
                        Token::RightSquareBracketToken( _ , _ , _ ) => {
                            let symbol3 = self.lexer.symbol.clone();
                            self.lexer.advance();

                            trailers.push(Box::new(SyntaxNode::TrailerIndexExprNode(pos, self.lexer.position, symbol2, next, symbol3)))
                        },
                        _ => return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting ']' in index trailer!"))))
                    }
                },
                Token::PeriodToken( _ , _ , _ ) => {
                    let symbol2 = self.lexer.symbol.clone();
                    self.lexer.advance();

                    match &*self.lexer.symbol {
                        Token::NameToken( _ , _ , _ , _ ) => {
                            let next = self.parse_atom_expr()?;
                            trailers.push(Box::new(SyntaxNode::TrailerDotNameExprNode(pos, self.lexer.position, symbol2, next)))
                        }
                        _ => return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting NAME literal after '.' in trailer!"))))
                    }
                },
                _ => break
            }
        }

        trailers.reverse();

        match symbol1.is_none() && trailers.is_empty() {
            true => Ok(Box::new(SyntaxNode::AtomExprNode(pos, self.lexer.position, symbol1, right, trailers))),
            _ => Ok(right)
        }
    }

    fn parse_atom_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol1 = self.lexer.symbol.clone();
        self.lexer.advance();

        match &*self.lexer.symbol {
            Token::NameToken( _ , _ , _ , _ ) => Ok(Box::new(SyntaxNode::NameExprNode(pos, self.lexer.position, symbol1))),
            Token::NumberToken( _ , _ , _ , _ ) => Ok(Box::new(SyntaxNode::NumberExprNode(pos, self.lexer.position, symbol1))),
            Token::NoneToken( _ , _ , _ ) => Ok(Box::new(SyntaxNode::NoneExprNode(pos, self.lexer.position, symbol1))),
            Token::FalseToken( _ , _ , _ ) => Ok(Box::new(SyntaxNode::FalseExprNode(pos, self.lexer.position, symbol1))),
            Token::TrueToken( _ , _ , _ ) => Ok(Box::new(SyntaxNode::TrueExprNode(pos, self.lexer.position, symbol1))),
            Token::EllipsisToken( _ , _ , _ ) => Ok(Box::new(SyntaxNode::EllipsisExprNode(pos, self.lexer.position, symbol1))),
            Token::StringToken( _ , _ , _ , _ ) => {
                let mut nodes = Vec::<Box<Token>>::new();
                nodes.push(symbol1);
                loop {
                    match &*self.lexer.symbol {
                        Token::StringToken( _ , _ , _ , _ ) => {
                            let symbol2 = self.lexer.symbol.clone();
                            self.lexer.advance();
                            nodes.push(symbol2)
                        }
                        _ => break
                    }
                }
                nodes.reverse();
                Ok(Box::new(SyntaxNode::StringExprNode(pos, self.lexer.position, nodes)))
            },
            Token::LeftParenToken( _ , _ , _ ) => {
                let right = match &*self.lexer.symbol {
                    Token::YieldToken( _ , _ , _ ) => Some( self.parse_yield_expr()? ),
                    Token::RightParenToken( _ , _ , _ ) => None,
                    _ => Some(self.parse_test_list_comp_expr()?)
                };
                match &*self.lexer.symbol {
                    Token::RightParenToken( _ , _ , _ ) => {
                        let symbol2 = self.lexer.symbol.clone();
                        self.lexer.advance();
                        Ok(Box::new(SyntaxNode::TupleExprNode(pos, self.lexer.position, symbol1, None, symbol2)))
                    }
                    _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting ')' in literal!"))))
                }
            },
            Token::LeftSquareBracketToken( _ , _ , _ ) => {
                let right = match &*self.lexer.symbol {
                    Token::RightParenToken( _ , _ , _ ) => None,
                    _ => Some(self.parse_test_list_comp_expr()?)
                };
                match &*self.lexer.symbol {
                    Token::RightSquareBracketToken( _ , _ , _ ) => {
                        let symbol2 = self.lexer.symbol.clone();
                        self.lexer.advance();
                        Ok(Box::new(SyntaxNode::ListExprNode(pos, self.lexer.position, symbol1, None, symbol2)))
                    }
                    _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting ')' in literal!"))))
                }
            },
            Token::LeftCurlyBracketToken( _ , _ , _ ) => self.parse_dictionary_set_maker_expr(symbol1, pos),
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting valid literal!"))))
        }
    }

    fn parse_test_list_comp_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut nodes = Vec::<Box<SyntaxNode>>::new();
        let mut separators = Vec::<Box<Token>>::new();

        nodes.push(match &*self.lexer.symbol {
            Token::MultiplyToken( _ , _ , _ ) => self.parse_star_expr()?,
            _ => self.parse_named_expr()?
        });

        match &*self.lexer.symbol {
            Token::ForToken( _ , _ , _ ) |
            Token::AsyncToken( _ , _ , _ ) => {
                nodes.push(self.parse_comp_for_expr()?)
            },
            _ => {
                loop {
                    match &*self.lexer.symbol {
                        Token::CommaToken( _ , _ , _ ) => {
                            separators.push(self.lexer.symbol.clone());
                            self.lexer.advance();

                            match &*self.lexer.symbol {
                                Token::RightParenToken( _ , _ , _ ) |
                                Token::RightSquareBracketToken( _ , _ , _ ) => break,
                                _ => {
                                    nodes.push(match &*self.lexer.symbol {
                                        Token::MultiplyToken( _ , _ , _ ) => self.parse_star_expr()?,
                                        _ => self.parse_named_expr()?
                                    })
                                }
                            }
                        },
                        _ => break
                    }
                }
            }
        }

        nodes.reverse();
        separators.reverse();

        Ok(match nodes.len() == 1 && separators.len() == 0 {
            true => nodes.pop().unwrap(),
            _ => Box::new(SyntaxNode::TestListComprehensionExprNode(pos, self.lexer.position, nodes, separators)),
        })
    }

    fn parse_subscript_list_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut nodes = Vec::<Box<SyntaxNode>>::new();
        let mut separators = Vec::<Box<Token>>::new();

        nodes.push(self.parse_subscript_expr()?);

        loop {
            match &*self.lexer.symbol {
                Token::CommaToken( _ , _ , _ ) => {
                    separators.push(self.lexer.symbol.clone());
                    self.lexer.advance();

                    match &*self.lexer.symbol {
                        Token::RightSquareBracketToken( _ , _ , _ ) => break,
                        _ => {
                            nodes.push(self.parse_subscript_expr()?);
                        }
                    }
                },
                _ => break
            }
        }

        nodes.reverse();
        separators.reverse();

        Ok(match nodes.len() == 1 && separators.len() == 0 {
            true => nodes.pop().unwrap(),
            _ => Box::new(SyntaxNode::SubscriptListExprNode(pos, self.lexer.position, nodes, separators)),
        })
    }

    fn parse_subscript_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let first = match &*self.lexer.symbol {
           Token::ColonToken( _ , _ , _ ) => None,
            _ => Some(self.parse_test_expr()?)
        };
        let mut symbol1 : Option<Box<Token>> = None;
        let mut symbol2 : Option<Box<Token>> = None;
        let mut second : Option<Box<SyntaxNode>> = None;
        let mut third : Option<Box<SyntaxNode>> = None;

        match &*self.lexer.symbol {
            Token::ColonToken( _ , _ , _ ) => {
                symbol1 = Some(self.lexer.symbol.clone());
                self.lexer.advance();

                match &*self.lexer.symbol {
                    Token::ColonToken( _ , _ , _ ) |
                    Token::CommaToken( _ , _ , _ ) |
                    Token::RightSquareBracketToken( _ , _ , _ )=> (),
                    _ => {
                        second = Some(self.parse_test_expr()?)
                    }
                }

                match &*self.lexer.symbol {
                    Token::ColonToken( _ , _ , _ ) => {
                        symbol2 = Some(self.lexer.symbol.clone());
                        self.lexer.advance();

                        match &*self.lexer.symbol {
                            Token::CommaToken( _ , _ , _ ) |
                            Token::RightSquareBracketToken( _ , _ , _ ) => (),
                            _ => third = Some(self.parse_test_expr()?)
                        }
                    },
                    _ => ()
                }
            },
            _ => ()
        }

        Ok(Box::new(SyntaxNode::SubscriptExprNode(pos, self.lexer.position, first, symbol1, second, symbol2, third)))
    }

    fn parse_expr_list_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut nodes = Vec::<Box<SyntaxNode>>::new();
        let mut separators = Vec::<Box<Token>>::new();

        nodes.push(match &*self.lexer.symbol {
            Token::MultiplyToken( _ , _ , _ ) => self.parse_star_expr()?,
            _ => self.parse_expr()?
        });

        loop {
            match &*self.lexer.symbol {
                Token::CommaToken( _ , _ , _ ) => {
                    separators.push(self.lexer.symbol.clone());
                    self.lexer.advance();

                    match &*self.lexer.symbol {
                        Token::InToken( _ , _ , _ ) => break,
                        _ => nodes.push(match &*self.lexer.symbol {
                            Token::MultiplyToken( _ , _ , _ ) => self.parse_star_expr()?,
                            _ => self.parse_expr()?
                        })
                    }
                },
                _ => break
            }
        }

        nodes.reverse();
        separators.reverse();

        Ok(match nodes.len() == 1 && separators.len() == 0 {
            true => nodes.pop().unwrap(),
            _ => Box::new(SyntaxNode::ExprListExprNode(pos, self.lexer.position, nodes, separators)),
        })
    }

    fn parse_test_list_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut nodes = Vec::<Box<SyntaxNode>>::new();
        let mut separators = Vec::<Box<Token>>::new();

        nodes.push(self.parse_test_expr()?);

        loop {
            match &*self.lexer.symbol {
                Token::CommaToken( _ , _ , _ ) => {
                    separators.push(self.lexer.symbol.clone());
                    self.lexer.advance();

                    match &*self.lexer.symbol {
                        Token::SemicolonToken( _ , _ , _ )|
                        Token::NewlineToken( _ , _ , _ , _ , _ ) => break,
                        _ => nodes.push(self.parse_test_expr()?)
                    }
                },
                _ => break
            }
        }

        nodes.reverse();
        separators.reverse();

        Ok(match nodes.len() == 1 && separators.len() == 0 {
            true => nodes.pop().unwrap(),
            _ => Box::new(SyntaxNode::TestListExprNode(pos, self.lexer.position, nodes, separators)),
        })
    }

    fn parse_dictionary_set_maker_expr(&mut self, symbol1: Box<Token>, position: u32) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let mut is_dictionary = true;

        match &*self.lexer.symbol {
            Token::RightCurlyBracketToken( _ , _ , _ ) => {
                let symbol2 = self.lexer.symbol.clone();
                self.lexer.advance();

                Ok(Box::new(SyntaxNode::DictionaryExprNode(position, self.lexer.position, symbol1, Vec::new(),Vec::new(), symbol2)))
            },
            _ => {
                let mut nodes = Vec::<Box<SyntaxNode>>::new();
                let mut separators = Vec::<Box<Token>>::new();
                let pos2 = self.lexer.position;

                /* First element */
                match *self.lexer.symbol {
                    Token::MultiplyToken( _ , _ , _ ) => {
                        is_dictionary = false;
                        let symbol = self.lexer.symbol.clone();
                        self.lexer.advance();
                        let right = self.parse_expr()?;

                        nodes.push(Box::new(SyntaxNode::SetReferenceNode(pos2, self.lexer.position, symbol, right)));
                    },
                    Token::PowerToken( _ , _ , _ ) => {
                        let symbol = self.lexer.symbol.clone();
                        self.lexer.advance();
                        let right = self.parse_test_expr()?;

                        nodes.push(Box::new(SyntaxNode::DictionaryReferenceNode(pos2, self.lexer.position, symbol, right)));
                    },
                    _ => {
                        let left = self.parse_test_expr()?;

                        match &*self.lexer.symbol {
                            Token::ColonToken( _ , _ , _ ) => {
                                let symbol2 = self.lexer.symbol.clone();
                                self.lexer.advance();
                                let right = self.parse_test_expr()?;

                                nodes.push(Box::new(SyntaxNode::DictionaryEntryNode(pos2, self.lexer.position, left, symbol2, right)))
                            },
                            _ => {
                                is_dictionary = false;
                                nodes.push(left)
                            }
                        }
                    }
                }

                /* Second and later elements */
                match &*self.lexer.symbol {
                    Token::AsyncToken( _ , _ , _ ) |
                    Token::ForToken( _ , _ , _ ) => nodes.push(self.parse_comp_for_expr()?),
                    _ => {
                        match is_dictionary {
                            true => {
                                loop {
                                    let mut pos2 = self.lexer.position;
                                    match &*self.lexer.symbol {
                                        Token::CommaToken( _ , _ , _ ) => {
                                            separators.push(self.lexer.symbol.clone());
                                            self.lexer.advance();

                                            match &*self.lexer.symbol {
                                                Token::RightCurlyBracketToken( _ , _ , _ ) => break,
                                                Token::PowerToken( _ , _ , _ ) => {
                                                    let symbol3 = self.lexer.symbol.clone();
                                                    self.lexer.advance();

                                                    let right = self.parse_test_expr()?;
                                                    nodes.push(Box::new(SyntaxNode::DictionaryReferenceNode(pos2, self.lexer.position, symbol3, right)))
                                                },
                                                _ => {
                                                    let left = self.parse_test_expr()?;
                                                    match &*self.lexer.symbol {
                                                        Token::SemicolonToken( _ , _ , _ ) => {
                                                            let symbol3 = self.lexer.symbol.clone();
                                                            self.lexer.advance();
                                                            let right = self.parse_test_expr()?;
                                                            nodes.push(Box::new(SyntaxNode::DictionaryEntryNode(pos2, self.lexer.position, left, symbol3, right)))
                                                        },
                                                        _ => return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting ':' in dictionary element!"))))
                                                    }
                                                }
                                            }
                                        },
                                        _ => break
                                    }
                                }
                            },
                            _ => {
                                loop {
                                    match &*self.lexer.symbol {
                                        Token::CommaToken( _ , _ , _ ) => {
                                            separators.push(self.lexer.symbol.clone());
                                            self.lexer.advance();

                                            match &*self.lexer.symbol {
                                                Token::RightCurlyBracketToken( _ , _ , _ ) => break,
                                                Token::MultiplyToken( _ , _ , _ ) => {
                                                    let pos = self.lexer.position;
                                                    let symbol3 = self.lexer.symbol.clone();
                                                    self.lexer.advance();

                                                    let right = self.parse_expr()?;
                                                    nodes.push(Box::new(SyntaxNode::SetReferenceNode(pos, self.lexer.position, symbol3, right)));
                                                },
                                                _ => nodes.push(self.parse_test_expr()?)
                                            }
                                        },
                                        _ => break
                                    }
                                }
                            }
                        }
                    }
                }

                /* End it up */

                let symbol2 = self.lexer.symbol.clone();
                self.lexer.advance();

                nodes.reverse();
                separators.reverse();

                match is_dictionary {
                    true => Ok(Box::new(SyntaxNode::DictionaryExprNode(position, self.lexer.position, symbol1, nodes, separators, symbol2))),
                    _ => Ok(Box::new(SyntaxNode::SetExprNode(position, self.lexer.position, symbol1, nodes, separators, symbol2)))
                }
            }
        }
    }

    fn parse_arg_list_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut nodes = Vec::<Box<SyntaxNode>>::new();
        let mut separators = Vec::<Box<Token>>::new();

        nodes.push(self.parse_argument_expr()?);

        loop {
            match &*self.lexer.symbol {
                Token::CommaToken( _ , _ , _ ) => {
                    separators.push(self.lexer.symbol.clone());
                    self.lexer.advance();

                    match &*self.lexer.symbol {
                        Token::RightParenToken( _ , _ , _ ) => break,
                        _ => nodes.push(self.parse_argument_expr()?)
                    }
                },
                _ => break
            }
        }

        nodes.reverse();
        separators.reverse();

        Ok(match nodes.len() == 1 && separators.len() == 0 {
            true => nodes.pop().unwrap(),
            _ => Box::new(SyntaxNode::ArgListExprNode(pos, self.lexer.position, nodes, separators)),
        })
    }

    fn parse_argument_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;

        match &*self.lexer.symbol {
            Token::MultiplyToken( _ , _ , _ ) |
            Token::PowerToken( _ , _ , _ ) => {
                let symbol = self.lexer.symbol.clone();
                self.lexer.advance();

                match &*self.lexer.symbol {
                    Token::NameToken( _ , _ , _ , _ ) => {
                        let right = self.parse_atom_expr()?;
                        Ok(Box::new(SyntaxNode::ArgumentExprNode(pos, self.lexer.position, None, Some(symbol), Some(right))))
                    },
                    _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting NAME literal in argument after '*' or '**'!"))))
                }
            },
            Token::NameToken( _ , _ , _ , _ ) => {
                let left = self.parse_atom_expr()?;

                match &*self.lexer.symbol {
                    Token::ColonToken( _ , _ , _ ) |
                    Token::AssignToken( _ , _ , _ )=> {
                        let symbol = self.lexer.symbol.clone();
                        self.lexer.advance();
                        let right = self.parse_atom_expr()?;
                        Ok(Box::new(SyntaxNode::ArgumentExprNode(pos, self.lexer.position, Some(left), Some(symbol), Some(right))))
                    },
                    Token::AsyncToken( _ , _ , _ ) |
                    Token::ForToken( _ , _ , _ )=> {
                        let  right = self.parse_comp_for_expr()?;
                        Ok(Box::new(SyntaxNode::ArgumentExprNode(pos, self.lexer.position, Some(left), None, Some(right))))
                    },
                    _ => Ok(left)
                }
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting NAME literal in argument!"))))
        }
    }

    fn parse_comp_iter_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        match &*self.lexer.symbol {
            Token::ForToken( _ , _ , _ ) => self.parse_sync_comp_for_expr(),
            Token::AsyncToken( _ , _ , _ ) => self.parse_comp_for_expr(),
            _ => self.parse_comp_if_expr()
        }
    }

    fn parse_sync_comp_for_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol1 = self.lexer.symbol.clone();
        self.lexer.advance();

        let left = self.parse_expr_list_expr()?;

        match &*self.lexer.symbol {
            Token::InToken( _ , _ , _ ) => {
                Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting 'in' in comprehension 'for' expression!"))))
            },
            _ => {
                let symbol2 = self.lexer.symbol.clone();
                self.lexer.advance();

                let right = self.parse_or_test_expr()?;

                let next = match &*self.lexer.symbol {
                    Token::AsyncToken( _ , _ , _ ) |
                    Token::ForToken( _ , _ , _ ) |
                    Token::IfToken( _ , _ , _ ) => {
                        Some(self.parse_comp_iter_expr()?)
                    },
                    _ => None
                };

                Ok(Box::new(SyntaxNode::SyncCompForExprNode(pos, self.lexer.position, symbol1, left, symbol2, right, next)))
            }
        }
    }

    fn parse_comp_for_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol = self.lexer.symbol.clone();
        self.lexer.advance();

        let right = self.parse_sync_comp_for_expr()?;

        let next = match &*self.lexer.symbol {
            Token::ForToken( _ , _ , _ ) |
            Token::IfToken( _ , _ , _ ) |
            Token::AsyncToken( _ , _ , _ ) => {
                Some(self.parse_comp_iter_expr()?)
            },
            _ => None
        };

        Ok(Box::new(SyntaxNode::CompForExprNode(pos, self.lexer.position, symbol, right, next)))
    }

    fn parse_comp_if_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol = self.lexer.symbol.clone();
        self.lexer.advance();

        let right = self.parse_test_no_cond_expr()?;

        let next = match &*self.lexer.symbol {
            Token::AsyncToken( _ , _ , _ ) |
            Token::ForToken( _ , _ , _ ) |
            Token::IfToken( _ , _ , _ )=> {
                Some(self.parse_comp_iter_expr()?)
            },
            _ => None
        };

        Ok(Box::new(SyntaxNode::CompIfExprNode(pos, self.lexer.position, symbol, right, next)))
    }

    fn parse_yield_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol1 = self.lexer.symbol.clone();
        self.lexer.advance();

        match &*self.lexer.symbol {
            Token::FromToken( _ , _ , _ ) => {
                let symbol2 = self.lexer.symbol.clone();
                self.lexer.advance();

                let right = self.parse_test_expr()?;

                Ok(Box::new(SyntaxNode::YieldFromExprNode(pos, self.lexer.position, symbol1, symbol2, right)))
            },
            _ => {
                let right = self.parse_test_list_star_expr_stmt()?;

                Ok(Box::new(SyntaxNode::YieldExprNode(pos, self.lexer.position, symbol1, right)))
            }
        }
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