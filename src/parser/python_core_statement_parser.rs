
use crate::parser::python_core_tokenizer::LexerMethods;
use crate::parser::python_core_expression_parser::ExpressionRules;
use crate::parser::syntax_error::SyntaxError;
use crate::parser::syntax_nodes::SyntaxNode;
use crate::parser::token_nodes::Token;
use crate::parser::python_core_block_parser::BlockGrammarRules;
use crate::parser::python_core_match_parser::MatchPatternRules;
use super::python_core_parser::PythonCoreParser;


pub trait StatementRules {
    fn parse_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_simple_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_small_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_expr_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_test_list_star_expr_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_del_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_pass_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_flow_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_break_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_continue_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_return_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_raise_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_import_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_import_name_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_import_from_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_import_as_name_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_dotted_as_name_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_import_as_names_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_dotted_as_names_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_dotted_name_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_global_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_nonlocal_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_assert_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_compound_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_async_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_if_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_elif_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_else_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_while_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_for_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_try_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_except_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_with_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_with_item_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_except_clause_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_suite_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;

}

impl StatementRules for PythonCoreParser {
    fn parse_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        match &*self.lexer.symbol {
            Token::IfToken( _ , _ , _ ) |
            Token::WhileToken( _ , _ , _ ) |
            Token::AsyncToken( _ , _ , _ ) |
            Token::ForToken( _ , _ , _ ) |
            Token::TryToken( _ , _ , _ ) |
            Token::WithToken( _ , _ , _ ) |
            Token::MatricesToken( _ , _ , _ ) |
            Token::DefToken( _ , _ , _ ) |
            Token::ClassToken( _ , _ , _ ) => self.parse_compound_stmt(),
            Token::NameToken( _ , _ , kw , _ ) => {
                match &*kw.as_str() {
                    "match" => self.parse_match_stmt(),
                    _ => self.parse_simple_stmt()
                }
            },
            _ => self.parse_simple_stmt()
        }
    }

    fn parse_simple_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut nodes = Vec::<Box<SyntaxNode>>::new();
        let mut separators = Vec::<Box<Token>>::new();

        nodes.push(self.parse_small_stmt()?);

        loop {
            match &*self.lexer.symbol {
                Token::SemicolonToken( _ , _ , _ ) => {
                    separators.push(self.lexer.symbol.clone());
                    self.lexer.advance();

                    match &*self.lexer.symbol {
                        Token::NewlineToken( _ , _ , _ , _ , _ ) => break,
                        _ => nodes.push(self.parse_small_stmt()?)
                    }
                },
                _ => break
            }
        }

        match &*self.lexer.symbol {
            Token::NewlineToken( _ , _ , _ , _ , _ ) => {
                let symbol = self.lexer.symbol.clone();
                self.lexer.advance();

                nodes.reverse();
                separators.reverse();

                Ok(Box::new(SyntaxNode::SimpleStmtNode(pos, self.lexer.position, nodes, separators, symbol)))
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting NEWLINE in statement list!"))))
        }
    }

    fn parse_small_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        match &*self.lexer.symbol {
            Token::DelToken( _ , _ , _ ) => self.parse_del_stmt(),
            Token::PassToken( _ , _ , _ ) => self.parse_pass_stmt(),
            Token::BreakToken( _ , _ , _ ) |
            Token::ContinueToken( _ , _ , _ ) |
            Token::ReturnToken( _ , _ , _ ) |
            Token::RaiseToken( _ , _ , _ ) |
            Token::YieldToken( _ , _ , _ ) => self.parse_flow_stmt(),
            Token::ImportToken( _ , _ , _ ) |
            Token::FromToken( _ , _ , _ ) => self.parse_import_stmt(),
            Token::GlobalToken( _ , _ , _ ) => self.parse_global_stmt(),
            Token::NonlocalToken( _ , _ , _ ) => self.parse_nonlocal_stmt(),
            Token::AssertToken( _ , _ , _ ) => self.parse_assert_stmt(),
            _ => self.parse_expr_stmt()
        }
    }

    fn parse_expr_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut left = self.parse_test_list_star_expr_stmt()?;

        match &*self.lexer.symbol {
            Token::PlusAssignToken( _ , _ , _ ) |
            Token::MinusAssignToken( _ , _ , _ ) |
            Token::MultiplyAssignToken( _ , _ , _ ) |
            Token::MatricesAssignToken( _ , _ , _ ) |
            Token::DivideAssignToken( _ , _ , _ ) |
            Token::ModuloAssignToken( _ , _ , _ ) |
            Token::AndAssignToken( _ , _ , _ ) |
            Token::OrAssignToken( _ , _ , _ ) |
            Token::XorAssignToken( _ , _ , _ ) |
            Token::ShiftLeftAssignToken( _ , _ , _ ) |
            Token::ShiftRightAssignToken( _ , _ , _ ) |
            Token::FloorDivideAssignToken( _ , _ , _ ) |
            Token::PowerAssignToken( _ , _ , _ ) => {
                let symbol = self.lexer.symbol.clone();
                self.lexer.advance();

                let right = match &*self.lexer.symbol {
                    Token::YieldToken( _ , _ , _ ) => self.parse_yield_expr()?,
                    _ => self.parse_test_list_expr()?
                };

                match &*symbol {
                    Token::PlusAssignToken( _ , _ , _ ) => Ok(Box::new(SyntaxNode::PlusAssignStmtNode(pos, self.lexer.position, left, symbol, right))),
                    Token::MinusAssignToken( _ , _ , _ ) => Ok(Box::new(SyntaxNode::MinusAssignStmtNode(pos, self.lexer.position, left, symbol, right))),
                    Token::MultiplyAssignToken( _ , _ , _ ) => Ok(Box::new(SyntaxNode::MulAssignStmtNode(pos, self.lexer.position, left, symbol, right))),
                    Token::MatricesAssignToken( _ , _ , _ ) => Ok(Box::new(SyntaxNode::MatricesAssignStmtNode(pos, self.lexer.position, left, symbol, right))),
                    Token::DivideAssignToken( _ , _ , _ ) => Ok(Box::new(SyntaxNode::DivAssignStmtNode(pos, self.lexer.position, left, symbol, right))),
                    Token::ModuloAssignToken( _ , _ , _ ) => Ok(Box::new(SyntaxNode::ModuloAssignStmtNode(pos, self.lexer.position, left, symbol, right))),
                    Token::AndAssignToken( _ , _ , _ ) => Ok(Box::new(SyntaxNode::BitAndAssignStmtNode(pos, self.lexer.position, left, symbol, right))),
                    Token::OrAssignToken( _ , _ , _ ) => Ok(Box::new(SyntaxNode::BitOrAssignStmtNode(pos, self.lexer.position, left, symbol, right))),
                    Token::XorAssignToken( _ , _ , _ ) => Ok(Box::new(SyntaxNode::BitXorAssignStmtNode(pos, self.lexer.position, left, symbol, right))),
                    Token::ShiftLeftAssignToken( _ , _ , _ ) => Ok(Box::new(SyntaxNode::ShiftLeftAssignStmtNode(pos, self.lexer.position, left, symbol, right))),
                    Token::ShiftRightAssignToken( _ , _ , _ ) => Ok(Box::new(SyntaxNode::ShiftRightAssignStmtNode(pos, self.lexer.position, left, symbol, right))),
                    Token::FloorDivideAssignToken( _ , _ , _ ) => Ok(Box::new(SyntaxNode::FloorDivAssignStmtNode(pos, self.lexer.position, left, symbol, right))),
                    _ => Ok(Box::new(SyntaxNode::PowerAssignStmtNode(pos, self.lexer.position, left, symbol, right)))
                }
            },
            Token::ColonToken( _ , _ , _ ) => {
                let symbol = self.lexer.symbol.clone();
                self.lexer.advance();

                let right = self.parse_test_expr()?;

                match &*self.lexer.symbol {
                    Token::AssignToken( _ , _ , _ ) => {
                        let symbol2 = self.lexer.symbol.clone();
                        self.lexer.advance();

                        let next = match &*self.lexer.symbol {
                            Token::YieldToken( _ , _ , _ ) => self.parse_yield_expr()?,
                            _ => self.parse_test_list_star_expr_stmt()?
                        };

                        Ok(Box::new(SyntaxNode::AnnAssignStmtNode(pos, self.lexer.position, left, symbol, right, Some(symbol2), Some(next))))
                    },
                    _ => Ok(Box::new(SyntaxNode::AnnAssignStmtNode(pos, self.lexer.position, left, symbol, right, None, None)))
                }
            },
            Token::AssignToken( _ , _ , _ ) => {
                let mut nodes = Vec::<Box<SyntaxNode>>::new();

                let mut symbol = self.lexer.symbol.clone();
                self.lexer.advance();

                let right = match &*self.lexer.symbol {
                    Token::YieldToken( _ , _ , _ ) => self.parse_yield_expr()?,
                    _ => self.parse_test_list_star_expr_stmt()?
                };

                nodes.push(Box::new(SyntaxNode::AssignmentElementStmtNode(pos, self.lexer.position, symbol, right)));

                loop {
                    match &*self.lexer.symbol {
                        Token::AssertToken( _ , _ , _ ) => {
                            symbol = self.lexer.symbol.clone();
                            self.lexer.advance();

                            let right = match &*self.lexer.symbol {
                                Token::YieldToken( _ , _ , _ ) => self.parse_yield_expr()?,
                                _ => self.parse_test_list_star_expr_stmt()?
                            };

                            nodes.push(Box::new(SyntaxNode::AssignmentElementStmtNode(pos, self.lexer.position, symbol, right)));
                        },
                        _ => break
                    }
                }

                let tc = match &*self.lexer.symbol {
                    Token::TypeCommentToken( _ , _ , _ , _ ) => {
                        let symbol2 = self.lexer.symbol.clone();
                        self.lexer.advance();
                        Some(symbol2)
                    },
                    _ => None
                };

                nodes.reverse();

                Ok(Box::new(SyntaxNode::AssignmentStmtNode(pos, self.lexer.position, left, nodes, tc)))
            },
            _ => Ok(left)
        }
    }

    fn parse_test_list_star_expr_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut nodes: Vec<Box<SyntaxNode>> = Vec::new();
        let mut separators: Vec<Box<Token>> = Vec::new();

        nodes.push(match &*self.lexer.symbol {
            Token::MultiplyToken( _ , _ , _ ) => self.parse_star_expr()?,
            _ => self.parse_test_expr()?
        });

        loop {
            match &*self.lexer.symbol {
                Token::CommaToken( _ , _ , _ ) => {
                    separators.push(self.lexer.symbol.clone());
                    self.lexer.advance();

                    match &*self.lexer.symbol {
                        Token::PlusAssignToken( _ , _ , _ ) |
                        Token::MinusAssignToken( _ , _ , _ ) |
                        Token::MultiplyAssignToken( _ , _ , _ ) |
                        Token::MatricesAssignToken( _ , _ , _ ) |
                        Token::DivideAssignToken( _ , _ , _ ) |
                        Token::ModuloAssignToken( _ , _ , _ ) |
                        Token::AndAssignToken( _ , _ , _ ) |
                        Token::OrAssignToken( _ , _ , _ ) |
                        Token::XorAssignToken( _ , _ , _ ) |
                        Token::ShiftLeftAssignToken( _ , _ , _ ) |
                        Token::ShiftRightAssignToken( _ , _ , _ ) |
                        Token::PowerAssignToken( _ , _ , _ ) |
                        Token::FloorDivideAssignToken( _ , _ , _ ) |
                        Token::SemicolonToken( _ , _ , _ ) |
                        Token::NewlineToken( _ , _ , _ , _ , _ ) |
                        Token::AssignToken( _ , _ , _ ) |
                        Token::ColonToken( _ , _ , _ ) => break,
                        _ => nodes.push(match &*self.lexer.symbol {
                                Token::MultiplyToken( _ , _ , _ ) => self.parse_star_expr()?,
                                _ => self.parse_test_expr()?
                            })
                    }
                },
                _ => break
            }
        }

        nodes.reverse();
        separators.reverse();

        Ok(Box::new(SyntaxNode::TestListStarExprStmtNode(pos, self.lexer.position, nodes, separators)))
    }

    fn parse_del_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol = self.lexer.symbol.clone();
        self.lexer.advance();

        let right = self.parse_expr_list_expr()?;

        Ok(Box::new(SyntaxNode::DelStmtNode(pos, self.lexer.position, symbol, right)))
    }

    fn parse_pass_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol = self.lexer.symbol.clone();
        self.lexer.advance();

        Ok(Box::new(SyntaxNode::PassStmtNode(pos, self.lexer.position, symbol)))
    }

    fn parse_flow_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;

        let right = match &*self.lexer.symbol {
            Token::BreakToken( _ , _ , _ ) => self.parse_break_stmt()?,
            Token::ContinueToken( _ , _ , _ ) => self.parse_continue_stmt()?,
            Token::ReturnToken( _ , _ , _ ) => self.parse_return_stmt()?,
            Token::RaiseToken( _ , _ , _ ) => self.parse_raise_stmt()?,
            _ => self.parse_yield_expr()?
        };

        Ok(right)
    }

    fn parse_break_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol = self.lexer.symbol.clone();
        self.lexer.advance();

        Ok(Box::new(SyntaxNode::BreakStmtNode(pos, self.lexer.position, symbol)))
    }

    fn parse_continue_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol = self.lexer.symbol.clone();
        self.lexer.advance();

        Ok(Box::new(SyntaxNode::ContinueStmtNode(pos, self.lexer.position, symbol)))
    }

    fn parse_return_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol = self.lexer.symbol.clone();
        self.lexer.advance();

        let right = match &*self.lexer.symbol {
            Token::NewlineToken( _ , _ , _ , _ , _ ) |
            Token::SemicolonToken( _ , _ , _ ) => None,
            _ => Some(self.parse_test_list_star_expr_stmt()?)
        };

        Ok(Box::new(SyntaxNode::ReturnStmtNode(pos, self.lexer.position, symbol, right)))
    }

    fn parse_raise_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol = self.lexer.symbol.clone();
        self.lexer.advance();

        match &*self.lexer.symbol {
            Token::NewlineToken( _ , _ , _ , _ , _ ) |
            Token::SemicolonToken( _ , _ , _ ) => {
                Ok(Box::new(SyntaxNode::RaiseStmtNode(pos, self.lexer.position, symbol, None, None, None)))
            },
            _ => {
                let left = self.parse_test_expr()?;

                match &*self.lexer.symbol {
                    Token::FromToken( _ , _ , _ ) => {
                        let symbol2 = self.lexer.symbol.clone();
                        self.lexer.advance();

                        let right = self.parse_test_expr()?;

                        Ok(Box::new(SyntaxNode::RaiseStmtNode(pos, self.lexer.position, symbol, Some(left), Some(symbol2), Some(right))))
                    },
                    _  => Ok(Box::new(SyntaxNode::RaiseStmtNode(pos, self.lexer.position, symbol, Some(left), None, None)))
                }
            }
        }
    }

    fn parse_import_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        match &*self.lexer.symbol {
            Token::ImportToken( _ , _ , _ ) => self.parse_import_stmt(),
            _ => self.parse_import_from_stmt()
        }
    }

    fn parse_import_name_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol = self.lexer.symbol.clone();
        self.lexer.advance();

        let right = self.parse_dotted_as_names_stmt()?;

        Ok(Box::new(SyntaxNode::ImportNameStmtNode(pos, self.lexer.position, symbol, right)))
    }

    fn parse_import_from_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol = self.lexer.symbol.clone(); /* 'from' */
        self.lexer.advance();

        let mut dots = Vec::<Box<Token>>::new();

        loop {
            match &*self.lexer.symbol {
                Token::PeriodToken( _ , _ , _ ) |
                Token::EllipsisToken( _ , _ , _ )=> {
                    dots.push(self.lexer.symbol.clone());
                    self.lexer.advance();
                },
                _ => break
            }
        }

        dots.reverse();

        let left = match (&*self.lexer.symbol, dots.len()) {
            ( Token::ImportToken( _ , _ , _ ), 0 ) => {
                return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting NAME literal or dot(s) before 'import' in import statement!"))))
            },
            ( Token::ImportToken( _ , _ , _ ), _  ) => None,
            ( _ , _ ) => Some(self.parse_dotted_name_stmt()?)
        };

        match &*self.lexer.symbol {
            Token::ImportToken( _ , _ , _ ) => {
                let symbol2 = self.lexer.symbol.clone();
                self.lexer.advance();

                match &*self.lexer.symbol {
                    Token::MultiplyToken( _ , _ , _ ) => { /* '*' */
                        let symbol3 = self.lexer.symbol.clone();
                        self.lexer.advance();

                        Ok(Box::new(SyntaxNode::ImportFromStmtNode(pos, self.lexer.position, symbol, dots, left, symbol2, Some(symbol3), None, None)))
                    },
                    Token::LeftParenToken( _ , _ , _ ) => {
                        let symbol3 = self.lexer.symbol.clone();
                        self.lexer.advance();

                        let right = self.parse_import_as_names_stmt()?;

                        match &*self.lexer.symbol {
                            Token::RightParenToken( _ , _ , _ ) => {
                                let symbol4 = self.lexer.symbol.clone();
                                self.lexer.advance();

                                Ok(Box::new(SyntaxNode::ImportFromStmtNode(pos, self.lexer.position, symbol, dots, left, symbol2, Some(symbol3), Some(right), Some(symbol4))))
                            },
                            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting ')' in from import statement!"))))
                        }
                    },
                    _ => {
                        let right = self.parse_import_as_names_stmt()?;

                        Ok(Box::new(SyntaxNode::ImportFromStmtNode(pos, self.lexer.position, symbol, dots, left, symbol2, None, Some(right), None)))
                    }
                }
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting 'import' in from import statement!"))))
        }
    }

    fn parse_import_as_name_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;

        match &*self.lexer.symbol {
            Token::NameToken( _ , _ , _ , _ ) => {
                let symbol1 = self.lexer.symbol.clone();
                self.lexer.advance();

                match &*self.lexer.symbol {
                    Token::AsToken( _ , _ , _ ) => {
                        let symbol2 = self.lexer.symbol.clone();
                        self.lexer.advance();

                        match &*self.lexer.symbol {
                            Token::NameToken( _ , _ , _ , _ ) => {
                                let symbol3 = self.lexer.symbol.clone();
                                self.lexer.advance();

                                Ok(Box::new(SyntaxNode::ImportAsNameStmtNode(pos, self.lexer.position, symbol1, Some(symbol2), Some(symbol3))))
                            },
                            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting NAME literal after 'as' in import statement!"))))
                        }
                    },
                    _ => Ok(Box::new(SyntaxNode::ImportAsNameStmtNode(pos, self.lexer.position, symbol1, None, None)))
                }
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting NAME literal in import statement!"))))
        }
    }

    fn parse_dotted_as_name_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let left = self.parse_dotted_name_stmt()?;

        match &*self.lexer.symbol {
            Token::AsToken( _ , _ , _ ) => {
                let symbol = self.lexer.symbol.clone();
                self.lexer.advance();

                match &*self.lexer.symbol {
                    Token::NameToken( _ , _ , _ , _ ) => {
                        let right = self.parse_atom_expr()?;

                        Ok(Box::new(SyntaxNode::DottedAsNameStmtNode(pos, self.lexer.position, left, symbol, right)))
                    },
                    _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting NAME literal after 'as' in import statement!"))))
                }
            },
            _ => Ok(left)
        }
    }

    fn parse_import_as_names_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut nodes = Vec::<Box<SyntaxNode>>::new();
        let mut separators = Vec::<Box<Token>>::new();

        nodes.push(self.parse_import_as_name_stmt()?);

        loop {
            match &*self.lexer.symbol {
                Token::CommaToken( _ , _ , _ ) => {
                    separators.push(self.lexer.symbol.clone());
                    self.lexer.advance();

                    nodes.push(self.parse_import_as_name_stmt()?);
                },
                _ => break
            }
        }

        Ok(match nodes.len() == 1 {
            true => nodes.pop().unwrap(),
            _ => Box::new(SyntaxNode::ImportAsNamesStmtNode(pos, self.lexer.position, nodes, separators))
        })
    }

    fn parse_dotted_as_names_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut nodes = Vec::<Box<SyntaxNode>>::new();
        let mut separators = Vec::<Box<Token>>::new();

        nodes.push(self.parse_dotted_as_name_stmt()?);

        loop {
            match &*self.lexer.symbol {
                Token::CommaToken( _ , _ , _ ) => {
                    separators.push(self.lexer.symbol.clone());
                    self.lexer.advance();

                    nodes.push(self.parse_dotted_as_name_stmt()?);
                },
                _ => break
            }
        }

        Ok(match nodes.len() == 1 {
            true => nodes.pop().unwrap(),
            _ => Box::new(SyntaxNode::DottedAsNamesStmtNode(pos, self.lexer.position, nodes, separators))
        })
    }

    fn parse_dotted_name_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
       let pos = self.lexer.position;
        let mut nodes = Vec::<Box<SyntaxNode>>::new();
        let mut sepators = Vec::<Box<Token>>::new();

        match &*self.lexer.symbol {
            Token::NameToken( _ , _ , _ , _ ) => {
                nodes.push(self.parse_atom_expr()?);

                loop {
                    match &*self.lexer.symbol {
                        Token::PeriodToken( _ , _ , _ ) => {
                            sepators.push(self.lexer.symbol.clone());
                            self.lexer.advance();

                            match &*self.lexer.symbol {
                                Token::NameToken( _ , _ , _ , _ ) => {
                                    nodes.push(self.parse_atom_expr()?);
                                },
                                _ => return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting NAME literal in import statement after '.'!"))))
                            }
                        },
                        _ => break
                    }
                }

                nodes.reverse();
                sepators.reverse();

                Ok(Box::new(SyntaxNode::DottedNameStmtNode(pos, self.lexer.position, nodes, sepators)))
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting NAME literal in import statement!"))))
        }
    }

    fn parse_global_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut nodes = Vec::<Box<SyntaxNode>>::new();
        let mut separators = Vec::<Box<Token>>::new();

        let symbol = self.lexer.symbol.clone();
        self.lexer.advance();

        match &*self.lexer.symbol {
            Token::NameToken( _ , _ , _ , _ ) => {
                nodes.push(self.parse_atom_expr()?);

                loop {
                    match &*self.lexer.symbol {
                        Token::CommaToken( _ , _ , _ ) => {
                            separators.push(self.lexer.symbol.clone());
                            self.lexer.advance();

                            match &*self.lexer.symbol {
                                Token::NameToken( _ , _ , _ , _ ) => {
                                    nodes.push(self.parse_atom_expr()?);
                                },
                                _ => return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting NAME literal in 'global' statement!"))))
                            }
                        },
                        _ => break
                    }
                }

                nodes.reverse();
                separators.reverse();

                Ok(Box::new(SyntaxNode::GlobalStmtNode(pos, self.lexer.position, symbol, nodes, separators)))
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting at least one NAME literal in 'global' statement!"))))
        }
    }

    fn parse_nonlocal_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut nodes = Vec::<Box<SyntaxNode>>::new();
        let mut separators = Vec::<Box<Token>>::new();

        let symbol = self.lexer.symbol.clone();
        self.lexer.advance();

        match &*self.lexer.symbol {
            Token::NameToken( _ , _ , _ , _ ) => {
                nodes.push(self.parse_atom_expr()?);

                loop {
                    match &*self.lexer.symbol {
                        Token::CommaToken( _ , _ , _ ) => {
                            separators.push(self.lexer.symbol.clone());
                            self.lexer.advance();

                            match &*self.lexer.symbol {
                                Token::NameToken( _ , _ , _ , _ ) => {
                                    nodes.push(self.parse_atom_expr()?);
                                },
                                _ => return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting NAME literal in 'nonlocal' statement!"))))
                            }
                        },
                        _ => break
                    }
                }

                nodes.reverse();
                separators.reverse();

                Ok(Box::new(SyntaxNode::NonlocalStmtNode(pos, self.lexer.position, symbol, nodes, separators)))
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting at least one NAME literal in 'nonlocal' statement!"))))
        }
    }

    fn parse_assert_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol = self.lexer.symbol.clone();
        self.lexer.advance();

        let left = self.parse_test_expr()?;

        match &*self.lexer.symbol {
            Token::CommaToken( _ , _ , _ ) => {
                let symbol2 = self.lexer.symbol.clone();
                self.lexer.advance();

                let right = self.parse_test_expr()?;

                Ok(Box::new(SyntaxNode::AssertStmtNode(pos, self.lexer.position, symbol, left, Some(symbol2), Some(right))))
            },
            _ => Ok(Box::new(SyntaxNode::AssertStmtNode(pos, self.lexer.position, symbol, left, None, None)))
        }
    }

    fn parse_compound_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        match &*self.lexer.symbol {
            Token::IfToken( _ , _ , _ ) => self.parse_if_stmt(),
            Token::ForToken( _ , _ , _ ) => self.parse_for_stmt(),
            Token::WhileToken( _ , _ , _ ) => self.parse_while_stmt(),
            Token::TryToken( _ , _ , _ ) => self.parse_try_stmt(),
            Token::WithToken( _ , _ , _ ) => self.parse_with_stmt(),
            Token::DefToken( _ , _ , _ ) => self.parse_func_def_stmt(),
            Token::ClassToken( _ , _ , _ ) => self.parse_class_stmt(),
            Token::AsyncToken( _ , _ , _ ) => self.parse_async_stmt(),
            Token::MatricesToken( _ , _ , _ ) => self.parse_decorated_stmt(),
            _ => self.parse_match_stmt()
        }
    }

    fn parse_async_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol = self.lexer.symbol.clone();
        self.lexer.advance();

        let right = match &*self.lexer.symbol {
            Token::DefToken( _ , _ , _ ) => self.parse_func_def_stmt()?,
            Token::ClassToken( _ , _ , _ ) => self.parse_class_stmt()?,
            Token::WithToken( _ , _ , _ ) => self.parse_with_stmt()?,
            _ => return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting 'def', 'class' or 'with' after 'async' statement!"))))
        };

        Ok(Box::new(SyntaxNode::AsyncStmtNode(pos, self.lexer.position, symbol, right)))
    }

    fn parse_if_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol = self.lexer.symbol.clone();
        self.lexer.advance();

        let left = self.parse_named_expr()?;

        match &*self.lexer.symbol {
            Token::ColonToken( _ , _ , _ ) => {
                let symbol2 = self.lexer.symbol.clone();
                self.lexer.advance();

                let right = self.parse_suite_stmt()?;

                let mut nodes = Vec::<Box<SyntaxNode>>::new();

                loop {
                    match &*self.lexer.symbol {
                        Token::ElseToken( _ , _ , _ ) => {
                            nodes.push(self.parse_elif_stmt()?);
                        },
                        _ => break
                    }
                }

                let else_part = match &*self.lexer.symbol {
                    Token::ElseToken( _, _ , _ ) => {
                        Some(self.parse_else_stmt()?)
                    },
                    _ => None
                };

                nodes.reverse();

                Ok(Box::new(SyntaxNode::IfStmtNode(pos, self.lexer.position, symbol, left, symbol2, right, nodes, else_part)))
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting ':' in 'if' statement!"))))
        }
    }

    fn parse_elif_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol = self.lexer.symbol.clone();
        self.lexer.advance();

        let left = self.parse_named_expr()?;

        match &*self.lexer.symbol {
            Token::ColonToken( _ , _ , _ ) => {
                let symbol2 = self.lexer.symbol.clone();
                self.lexer.advance();

                let right = self.parse_suite_stmt()?;

                Ok(Box::new(SyntaxNode::ElifStmtNode(pos, self.lexer.position, symbol, left, symbol2, right)))
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting ':' in 'elif' statement!"))))
        }
    }

    fn parse_else_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol = self.lexer.symbol.clone();
        self.lexer.advance();

        match &*self.lexer.symbol {
            Token::ColonToken( _ , _ , _ ) => {
                let symbol2 = self.lexer.symbol.clone();
                self.lexer.advance();

                let right = self.parse_suite_stmt()?;

                Ok(Box::new(SyntaxNode::ElseStmtNode(pos, self.lexer.position, symbol, symbol2, right)))
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting ':' in 'else' statement!"))))
        }
    }

    fn parse_while_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol = self.lexer.symbol.clone();
        self.lexer.advance();

        let left = self.parse_named_expr()?;

        match &*self.lexer.symbol {
            Token::ColonToken( _ , _ , _ ) => {
                let symbol2 = self.lexer.symbol.clone();
                self.lexer.advance();

                let right = self.parse_suite_stmt()?;

                let else_part = match &*self.lexer.symbol {
                    Token::ElseToken( _, _ , _ ) => Some(self.parse_else_stmt()?),
                    _ => None
                };

                Ok(Box::new(SyntaxNode::WhileStmtNode(pos, self.lexer.position, symbol, left, symbol2, right, else_part)))
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting ':' in 'while' statement!"))))
        }
    }

    fn parse_for_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol = self.lexer.symbol.clone();
        self.lexer.advance();

        let left = self.parse_expr_list_expr()?;

        match &*self.lexer.symbol {
            Token::InToken( _ , _ , _ ) => {
                let symbol2 = self.lexer.symbol.clone();
                self.lexer.advance();

                let right = self.parse_test_list_expr()?;

                match &*self.lexer.symbol {
                    Token::ColonToken( _ , _ , _ ) => {
                        let symbol3 = self.lexer.symbol.clone();
                        self.lexer.advance();

                        let tc = match &*self.lexer.symbol {
                            Token::TypeCommentToken( _ , _ , _ , _ ) => {
                                let symbol5 = self.lexer.symbol.clone();
                                self.lexer.advance();
                                Some(symbol5)
                            },
                            _ => None
                        };

                        let next = self.parse_suite_stmt()?;

                        let else_part = match &*self.lexer.symbol {
                            Token::ElseToken( _, _ , _ ) => Some(self.parse_else_stmt()?),
                            _ => None
                        };

                        Ok(Box::new(SyntaxNode::ForStmtNode(pos, self.lexer.position, symbol, left, symbol2, right, symbol3, tc, next, else_part)))
                    },
                    _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting ':' in 'for' statement!"))))
                }
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting 'in' in 'for' statement!"))))
        }
    }

    fn parse_try_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol = self.lexer.symbol.clone();
        self.lexer.advance();

        match &*self.lexer.symbol {
            Token::ColonToken( _ , _ , _ ) => {
                let symbol2 = self.lexer.symbol.clone();
                self.lexer.advance();

                let left = self.parse_suite_stmt()?;

                match &*self.lexer.symbol {
                    Token::FinallyToken( _ , _ , _ ) => {
                        let symbol3 = self.lexer.symbol.clone();
                        self.lexer.advance();

                        match &*self.lexer.symbol {
                            Token::ColonToken( _ , _ , _ ) => {
                                let symbol4 = self.lexer.symbol.clone();
                                self.lexer.advance();
                                let next = self.parse_suite_stmt()?;

                                Ok(Box::new(SyntaxNode::TryStmtNode(pos, self.lexer.position, symbol, symbol2, left, Vec::new(), None, Some(symbol3), Some(symbol4), Some(next))))
                            },
                            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting ':' in 'finally' statement!"))))
                        }
                    },
                    _ => {
                        let mut nodes = Vec::<Box<SyntaxNode>>::new();

                        match &*self.lexer.symbol {
                            Token::ExceptToken( _ , _ , _ ) => {
                                nodes.push(self.parse_except_stmt()?);

                                loop {
                                    match &*self.lexer.symbol {
                                        Token::ExceptToken( _ , _ , _ ) => nodes.push(self.parse_except_stmt()?),
                                        _ => break
                                    }
                                }

                                nodes.reverse();

                                let else_part = match &*self.lexer.symbol {
                                    Token::ElseToken( _ , _ , _ ) => Some(self.parse_else_stmt()?),
                                    _ => None
                                };

                                match &*self.lexer.symbol {
                                    Token::FinallyToken( _ , _ , _ ) => {
                                        let symbol3 = self.lexer.symbol.clone();
                                        self.lexer.advance();

                                        match &*self.lexer.symbol {
                                            Token::ColonToken( _ , _ , _ ) => {
                                                let symbol4 = self.lexer.symbol.clone();
                                                self.lexer.advance();
                                                let next = self.parse_suite_stmt()?;

                                                Ok(Box::new(SyntaxNode::TryStmtNode(pos, self.lexer.position, symbol, symbol2, left, Vec::new(), else_part, Some(symbol3), Some(symbol4), Some(next))))
                                            },
                                            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting ':' in 'finally' statement!"))))
                                        }
                                    },
                                    _ => Ok(Box::new(SyntaxNode::TryStmtNode(pos, self.lexer.position, symbol, symbol2, left, nodes, else_part, None, None, None)))
                                }
                            },
                            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting 'except' in 'try' statement!"))))
                        }
                    }
                }
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting ':' in 'try' statement!"))))
        }
    }

    fn parse_except_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let left = self.parse_except_clause_stmt()?;

        match &*self.lexer.symbol {
            Token::ColonToken( _ , _ , _ ) => {
                let symbol = self.lexer.symbol.clone();
                self.lexer.advance();

                let right = self.parse_suite_stmt()?;

                Ok(Box::new(SyntaxNode::ExceptStmtNode(pos, self.lexer.position, left, symbol, right)))
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting ':' in 'with' statement!"))))
        }
    }

    fn parse_with_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol = self.lexer.symbol.clone();
        self.lexer.advance();

        let mut nodes = Vec::<Box<SyntaxNode>>::new();
        let mut separators = Vec::<Box<Token>>::new();

        nodes.push(self.parse_with_item_stmt()?);

        loop {
            match &*self.lexer.symbol {
                Token::CommaToken( _ , _ , _ ) => {
                    separators.push(self.lexer.symbol.clone());
                    self.lexer.advance();

                    nodes.push(self.parse_with_item_stmt()?)
                },
                _ => break
            }
        }

        nodes.reverse();
        separators.reverse();

        match &*self.lexer.symbol {
            Token::ColonToken( _ , _ , _ ) => {
                let symbol2 = self.lexer.symbol.clone();
                self.lexer.advance();

                let tc = match &*self.lexer.symbol {
                    Token::TypeCommentToken( _ , _ , _ , _ ) => {
                        let symbol3 = self.lexer.symbol.clone();
                        self.lexer.advance();

                        Some(symbol3)
                    },
                    _ => None
                };

                let next = self.parse_suite_stmt()?;

                Ok(Box::new(SyntaxNode::WithStmtNode(pos, self.lexer.position, symbol, nodes, separators, symbol2, tc, next)))
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting ':' in 'with' statement!"))))
        }
    }

    fn parse_with_item_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let left = self.parse_test_expr()?;

        match &*self.lexer.symbol {
            Token::AsToken( _ , _ , _ ) => {
                let symbol = self.lexer.symbol.clone();
                self.lexer.advance();

                let right = self.parse_expr()?;

                Ok(Box::new(SyntaxNode::WithItemStmtNode(pos, self.lexer.position, left, Some(symbol), Some(right))))
            },
            _ => Ok(Box::new(SyntaxNode::WithItemStmtNode(pos, self.lexer.position, left, None, None)))
        }
    }

    fn parse_except_clause_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol = self.lexer.symbol.clone();
        self.lexer.advance();

        match &*self.lexer.symbol {
            Token::ColonToken( _ , _ , _ ) => Ok(Box::new(SyntaxNode::ExceptClauseStmtNode(pos, self.lexer.position, symbol, None, None, None))),
            _ => {
                let left = self.parse_test_expr()?;

                match &*self.lexer.symbol {
                    Token::AsToken( _ , _ , _ ) => {
                        let symbol2 = self.lexer.symbol.clone();
                        self.lexer.advance();

                        match &*self.lexer.symbol {
                            Token::NameToken( _ , _ , _ , _ ) => {
                                let right = self.parse_suite_stmt()?;

                                Ok(Box::new(SyntaxNode::ExceptClauseStmtNode(pos, self.lexer.position, symbol, Some(left), Some(symbol2), Some(right))))
                            },
                            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting NAME literal in except clause in 'try' statement!"))))
                        }
                    },
                    _ => Ok(Box::new(SyntaxNode::ExceptClauseStmtNode(pos, self.lexer.position, symbol, Some(left), None, None)))
                }
            }
        }
    }

    fn parse_suite_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        match &*self.lexer.symbol {
            Token::NewlineToken( _ , _ , _ , _ , _ ) => {
                let symbol1 = self.lexer.symbol.clone();
                self.lexer.advance();

                match &*self.lexer.symbol {
                    Token::IndentToken( _ , _ , _ ) => {
                        let symbol2 = self.lexer.symbol.clone();
                        self.lexer.advance();

                        let mut nodes = Vec::<Box<SyntaxNode>>::new();

                        nodes.push(self.parse_stmt()?);

                        loop {
                            match &*self.lexer.symbol {
                                Token::DedentToken( _ , _ , _ ) => break,
                                _ => nodes.push(self.parse_stmt()?)
                            }
                        }

                        nodes.reverse();

                        let symbol3 = self.lexer.symbol.clone();
                        self.lexer.advance();

                        Ok(Box::new(SyntaxNode::SuiteStmtNode(pos, self.lexer.position, symbol1, symbol2, nodes, symbol3)))
                    },
                    _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting 'indent' statement!"))))
                }
            },
            _ => self.parse_simple_stmt()
        }
    }
}

// Unittests for statement grammar rules ///////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    #[test]
    fn dummy_test() {
        assert_eq!(1 == 1, true);
    }
}