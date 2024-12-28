
use crate::parser::python_core_expression_parser::ExpressionRules;
use crate::parser::python_core_statement_parser::StatementRules;
use crate::parser::python_core_tokenizer::LexerMethods;
use crate::parser::syntax_error::SyntaxError;
use crate::parser::syntax_nodes::SyntaxNode;
use crate::parser::syntax_nodes::SyntaxNode::TypedPowerElementNode;
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
    fn parse_typed_element_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_typed_star_element_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_typed_power_element_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_tfp_def(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_var_args_list_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_func_body_suite_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_func_type_input(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_func_type(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_type_list(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
}

impl BlockGrammarRules for PythonCoreParser {
    fn parse_single_input(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        match &*self.lexer.symbol {
            Token::NewlineToken( _ , _ , _ , _ , _ ) => {
                let symbol1 = self.lexer.symbol.clone();
                self.lexer.advance();
                Ok(Box::new(SyntaxNode::SingleInputStmtNode(pos, self.lexer.position, None, Some(symbol1))))
            },
            Token::IfToken( _ , _ , _ ) |
            Token::WhileToken( _ , _ , _ ) |
            Token::ForToken( _ , _ , _ ) |
            Token::TryToken( _ , _ , _ ) |
            Token::WithToken( _ , _ , _ ) |
            Token::DefToken( _ , _ , _ ) |
            Token::ClassToken( _ , _ , _ ) |
            Token::MatricesToken( _ , _ , _ ) |
            Token::AsyncToken( _ , _ , _ ) => {
                let right = self.parse_stmt()?;

                match &*self.lexer.symbol {
                    Token::NewlineToken( _ , _ , _ , _ , _ ) => {
                        let symbol2 = self.lexer.symbol.clone();
                        self.lexer.advance();
                        Ok(Box::new(SyntaxNode::SingleInputStmtNode(pos, self.lexer.position, Some(right), Some(symbol2))))
                    },
                    _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting NEWLINE after statement in single input!"))))
                }
            },
            _ => {
                let right = self.parse_simple_stmt()?;
                Ok(Box::new(SyntaxNode::SingleInputStmtNode(pos, self.lexer.position, Some(right), None)))
            }
        }
    }

    fn parse_file_input(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut nodes = Vec::<Box<SyntaxNode>>::new();
        let mut separators = Vec::<Box<Token>>::new();

        loop {
            match &*self.lexer.symbol {
                Token::EofToken( _ , _ , _ ) => {
                    nodes.reverse();
                    separators.reverse();

                    return Ok(Box::new(SyntaxNode::FileInputStmtNode(pos, self.lexer.position, nodes, separators, self.lexer.symbol.clone())))
                },
                Token::NewlineToken( _ , _ , _ , _ , _ ) => {
                    let symbol = self.lexer.symbol.clone();
                    self.lexer.advance();
                    separators.push(symbol)
                },
                _ => nodes.push(self.parse_stmt()?),
            }
        }
    }

    fn parse_eval_input(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let right = self.parse_test_list_expr()?;
        let mut nodes = Vec::<Box<Token>>::new();

        loop {
            match &*self.lexer.symbol {
                Token::NewlineToken( _ , _ , _ , _ , _ ) => {
                    let symbol1 = self.lexer.symbol.clone();
                    self.lexer.advance();
                    nodes.push(symbol1)
                },
                Token::EofToken( _ , _ , _ ) => return Ok(Box::new(SyntaxNode::EvalInputStmtNode(pos, self.lexer.position, right, nodes, self.lexer.symbol.clone()))),
                _ => return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting End of file at end of eval input!"))))
            }
        }
    }

    fn parse_decorator_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol1 = self.lexer.symbol.clone();
        self.lexer.advance();
        let left = self.parse_dotted_name_stmt()?;

        match &*self.lexer.symbol {
            Token::LeftParenToken( _ , _ , _ ) => {
                let symbol2 = Some(self.lexer.symbol.clone());
                self.lexer.advance();
                let right = match &*self.lexer.symbol {
                    Token::RightParenToken( _ , _ , _ ) => None,
                    _ => Some(self.parse_arg_list_expr()?)
                };
                match &*self.lexer.symbol {
                    Token::RightParenToken( _ , _ , _ ) => {
                        let symbol3 = Some(self.lexer.symbol.clone());
                        self.lexer.advance();

                        match &*self.lexer.symbol {
                            Token::NewlineToken( _ , _ , _ , _ , _ ) => {
                                let symbol4 = self.lexer.symbol.clone();
                                self.lexer.advance();

                                Ok(Box::new(SyntaxNode::DecoratorStmtNode(pos, self.lexer.position, symbol1, left, symbol2, right, symbol3, symbol4)))
                            },
                            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting NEWLINE after decorator argument name!"))))
                        }
                    },
                    _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting ')' after decorator argument name!"))))
                }
            },
            Token::NewlineToken( _ , _ , _ , _ , _ ) => {
                let symbol4 = self.lexer.symbol.clone();
                self.lexer.advance();
                Ok(Box::new(SyntaxNode::DecoratorStmtNode(pos, self.lexer.position, symbol1, left, None, None, None, symbol4)))
            }
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting '(' or NEWLINE after decorator name!"))))
        }
    }

    fn parse_decorators_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut nodes = Vec::<Box<SyntaxNode>>::new();
        nodes.push(self.parse_decorator_stmt()?);

        loop {
            match &*self.lexer.symbol {
                Token::MatricesToken( _ , _ , _ ) => nodes.push(self.parse_decorator_stmt()?),
                _ => break
            }
        }

        nodes.reverse();
        Ok(Box::new(SyntaxNode::DecoratorsStmtNode(pos, self.lexer.position, nodes)))
    }

    fn parse_decorated_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let left = self.parse_decorators_stmt()?;
        let right = match &*self.lexer.symbol {
            Token::ClassToken( _ , _ , _ ) => self.parse_class_stmt()?,
            Token::AsyncToken( _ , _ , _ ) => self.parse_async_stmt()?,
            Token::DefToken( _ , _ , _ ) => self.parse_func_def_stmt()?,
            _ => return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting 'class', 'def' or 'async' after decorators!"))))
        };

        Ok(Box::new(SyntaxNode::DecoratedStmtNode(pos, self.lexer.position, left, right)))
    }

    fn parse_class_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol1 = self.lexer.symbol.clone();
        self.lexer.advance();

        match &*self.lexer.symbol {
            Token::NameToken( _ , _ , _ , _ ) => {
                let symbol2 = self.lexer.symbol.clone();
                self.lexer.advance();

                match &*self.lexer.symbol {
                    Token::LeftParenToken( _ , _ , _ ) => {
                        let symbol3 = Some(self.lexer.symbol.clone());
                        self.lexer.advance();

                        let right = match &*self.lexer.symbol {
                            Token::RightParenToken( _ , _ , _ ) => None,
                            _ => Some(self.parse_arg_list_expr()?)
                        };
                        match &*self.lexer.symbol {
                            Token::RightParenToken( _ , _ , _ ) => {
                                let symbol4 = Some(self.lexer.symbol.clone());
                                self.lexer.advance();

                                match &*self.lexer.symbol {
                                    Token::ColonToken( _ , _ , _ ) => {
                                        let symbol5 = self.lexer.symbol.clone();
                                        self.lexer.advance();

                                        let next = self.parse_suite_stmt()?;

                                        Ok(Box::new(SyntaxNode::ClassDefStmtNode(pos, self.lexer.position, symbol1, symbol2, symbol3, right, symbol4, symbol5, next)))
                                    },
                                    _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting ':' in 'class' statement!"))))
                                }
                            },
                            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting ')' in 'class' statement!"))))
                        }
                    },
                    Token::ColonToken( _ , _ , _ ) => {
                        let symbol3 = self.lexer.symbol.clone();
                        self.lexer.advance();

                        let right = self.parse_suite_stmt()?;

                        Ok(Box::new(SyntaxNode::ClassDefStmtNode(pos, self.lexer.position, symbol1, symbol2, None, None, None, symbol3, right)))
                    },
                    _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting ':' in 'class' statement!"))))
                }

            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting NAME after 'class'!"))))
        }
    }

    fn parse_func_def_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol1 = self.lexer.symbol.clone();
        self.lexer.advance();

        match &*self.lexer.symbol {
            Token::NameToken( _ , _ , _ , _ ) => {
                let name = self.lexer.symbol.clone();
                self.lexer.advance();

                let left = self.parse_parameters_stmt()?;

                let (symbol2, node1 ) = match &*self.lexer.symbol {
                    Token::ArrowToken( _ , _ , _ ) => {
                        let symbol = self.lexer.symbol.clone();
                        self.lexer.advance();

                        let right = self.parse_test_expr()?;

                        ( Some(symbol), Some(right) )
                    },
                    _ => ( None, None )
                };

                match &*self.lexer.symbol {
                    Token::ColonToken( _ , _ , _ ) => {
                        let symbol3 = self.lexer.symbol.clone();
                        self.lexer.advance();

                        let tc = match &*self.lexer.symbol {
                            Token::TypeCommentToken( _ , _ , _ , _ ) => {
                                let symbol3 = self.lexer.symbol.clone();
                                self.lexer.advance();

                                Some(symbol3)
                            },
                            _ => None
                        };

                        let next = self.parse_func_body_suite_stmt()?;

                        Ok(Box::new(SyntaxNode::FuncDefinitionNode(pos, self.lexer.position, symbol1, name, left, symbol2, node1, symbol3,  tc, next)))
                    },
                    _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting ':' in 'def' statement!"))))
                }
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting NAME after 'def'!"))))
        }
    }

    fn parse_parameters_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        match &*self.lexer.symbol {
            Token::LeftParenToken( _ , _ , _ ) => {
                let symbol1 = self.lexer.symbol.clone();
                self.lexer.advance();

                let right = match &*self.lexer.symbol {
                    Token::RightParenToken( _ , _ , _ ) => None,
                    _ => Some(self.parse_typed_args_list_stmt()?)
                };

                match &*self.lexer.symbol {
                    Token::RightParenToken( _ , _ , _ ) => {
                        let symbol2 = self.lexer.symbol.clone();
                        self.lexer.advance();

                        Ok(Box::new(SyntaxNode::ParametersNode(pos, self.lexer.position, symbol1, right, symbol2)))
                    },
                    _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting ')' in function declaration!"))))
                }

            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting '(' in function declaration!"))))
        }
    }

    fn parse_typed_args_list_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut nodes = Vec::<Box<SyntaxNode>>::new();
        let mut separators = Vec::<Box<Token>>::new();

        match &*self.lexer.symbol {
            Token::PowerToken( _ , _ , _ ) => {
                nodes.push(self.parse_typed_power_element_stmt()?);
                Ok(Box::new(SyntaxNode::TypedListNode(pos, self.lexer.position, nodes, separators, None)))
            },
            Token::MultiplyToken( _ , _ , _ ) => {

                todo!()
            },
            _ => {

                todo!()
            }
        }
    }

    fn parse_typed_element_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;

        let tc = match &*self.lexer.symbol {
            Token::TypeCommentToken( _ , _ , _ , _ ) => {
                let symbol1 = self.lexer.symbol.clone();
                self.lexer.advance();
                Some(symbol1)
            },
            _ => None
        };

        let left = self.parse_tfp_def()?;

        let ( symbol2, right ) = match &*self.lexer.symbol {
            Token::AssignToken( _ , _ , _ ) => {
                let symbol1 = self.lexer.symbol.clone();
                self.lexer.advance();

                let right = self.parse_test_expr()?;

                ( Some(symbol1), Some(right) )
            },
            _ => ( None, None )
        };

        Ok(Box::new(SyntaxNode::TypedElementNode(pos, self.lexer.position, tc, left, symbol2, right)))
    }

    fn parse_typed_star_element_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol1 = self.lexer.symbol.clone();
        self.lexer.advance();

        let right = match &*self.lexer.symbol {
            Token::CommaToken( _ , _ , _ ) |
            Token::RightParenToken( _ , _ , _ ) => None,
            _ => Some(self.parse_tfp_def()?)
        };

        Ok(Box::new(SyntaxNode::TypedStarElementNode(pos, self.lexer.position, symbol1, right)))
    }

    fn parse_typed_power_element_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let symbol1 = self.lexer.symbol.clone();
        self.lexer.advance();
        let right = self.parse_tfp_def()?;

        let extra = match &*self.lexer.symbol {
            Token::CommaToken( _ , _ , _ ) => {
                let symbol2 = self.lexer.symbol.clone();
                self.lexer.advance();
                Some(symbol2)
            },
            _ => None
        };

        let tc = match &*self.lexer.symbol {
            Token::TypeCommentToken( _ , _ , _ , _ ) => {
                let symbol3 = self.lexer.symbol.clone();
                self.lexer.advance();
                Some(symbol3)
            },
            _ => None
        };

        Ok(Box::new(TypedPowerElementNode(pos, self.lexer.position, symbol1, right, extra, tc)))
    }

    fn parse_tfp_def(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;

        match &*self.lexer.symbol {
            Token::NameToken( _ , _ , _ , _ ) => {
                let left = self.parse_test_expr()?;

                match &*self.lexer.symbol {
                    Token::ColonToken( _ , _ , _ ) => {
                        let symbol1 = self.lexer.symbol.clone();
                        self.lexer.advance();

                        let right = self.parse_test_expr()?;

                        Ok(Box::new(SyntaxNode::TypedFormalParameterNode(pos, self.lexer.position, left, symbol1, right)))
                    },
                    _ => Ok(left)
                }
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting Name literal in argument!"))))
        }
    }

    fn parse_var_args_list_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        match &*self.lexer.symbol {
            Token::PowerToken( _ , _ , _ ) => {

                todo!()
            },
            Token::MultiplyToken( _ , _ , _ ) => {

                todo!()
            },
            _ => {

                todo!()
            }
        }
    }

    fn parse_func_body_suite_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut nodes = Vec::<Box<SyntaxNode>>::new();
        let mut tc : Option<Box<Token>> = None;
        let mut tc_nl : Option<Box<Token>> = None;

        match &*self.lexer.symbol {
            Token::NewlineToken( _ , _ , _ , _ , _ ) => {
                let symbol1 = self.lexer.symbol.clone();
                self.lexer.advance();

                /* Type Comment handling */
                match &*self.lexer.symbol {
                    Token::TypeCommentToken( _ , _ , _ , _ ) => {
                        tc = Some(self.lexer.symbol.clone());
                        self.lexer.advance();

                        match &*self.lexer.symbol {
                            Token::NewlineToken( _ , _ , _ , _ , _ ) => {
                                tc_nl = Some(self.lexer.symbol.clone());
                                self.lexer.advance()
                            },
                            _ => return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting NEWLINE after type comment in func body!"))))
                        }
                    },
                    _ => ()
                }

                /* Body block */
                match &*self.lexer.symbol {
                    Token::IndentToken( _ , _ , _ ) => {
                        let symbol2 = self.lexer.symbol.clone();
                        self.lexer.advance();

                        nodes.push (self.parse_stmt()?);

                        loop {
                            match &*self.lexer.symbol {
                                Token::DedentToken( _ , _ , _ ) => {
                                    let symbol3 = self.lexer.symbol.clone();
                                    self.lexer.advance();

                                    nodes.reverse();

                                    return Ok(Box::new(SyntaxNode::FuncBodyStmtNode(pos, self.lexer.position, symbol1, tc, tc_nl, symbol2, nodes, symbol3)))
                                },
                                _ => nodes.push (self.parse_stmt()?)
                            }
                        }
                    },
                    _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting Indent in func body!"))))
                }
            },
            _ => self.parse_simple_stmt()
        }
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
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting EOF in func type input!"))))
        }
    }

    fn parse_func_type(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        match &*self.lexer.symbol {
            Token::LeftParenToken( _ , _ , _ ) => {
                let symbol1 = self.lexer.symbol.clone();
                self.lexer.advance();

                let left = match &*self.lexer.symbol {
                    Token::RightParenToken( _ , _ , _ ) => None,
                    _ => Some(self.parse_type_list()?)
                };

                match &*self.lexer.symbol {
                    Token::RightParenToken( _ , _ , _ ) => {
                        let symbol2 = self.lexer.symbol.clone();
                        self.lexer.advance();

                        match &*self.lexer.symbol {
                            Token::ArrowToken( _ , _ , _ ) => {
                                let symbol3 = self.lexer.symbol.clone();
                                self.lexer.advance();

                                let right = self.parse_test_expr()?;

                                Ok(Box::new(SyntaxNode::FuncTypeStmtNode(pos, self.lexer.position, symbol1, left, symbol2, symbol3, right)))
                            },
                            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting '->' in func type input!"))))
                        }
                    },
                    _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting ')' in func type input!"))))
                }
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting '(' in func type input!"))))
        }
    }

    fn parse_type_list(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut nodes = Vec::<Box<SyntaxNode>>::new();
        let mut separators = Vec::<Box<Token>>::new();
        let mut symbol1 : Option<Box<Token>> = None;
        let mut symbol2 : Option<Box<Token>> = None;
        let mut node1 : Option<Box<SyntaxNode>> = None;
        let mut node2 : Option<Box<SyntaxNode>> = None;

        match &*self.lexer.symbol {
            Token::PowerToken( _ , _ , _ ) => {
                symbol2 = Some(self.lexer.symbol.clone());
                self.lexer.advance();
                node2 = Some(self.parse_test_expr()?)
            },
            Token::MultiplyToken( _ , _ , _ ) => {
                symbol1 = Some(self.lexer.symbol.clone());
                self.lexer.advance();

                node1 = match &*self.lexer.symbol {
                    Token::RightParenToken( _ , _ , _ ) | Token::CommaToken( _ , _ , _ ) => None,
                    _ => Some(self.parse_test_expr()?)
                };

                loop {
                    match &*self.lexer.symbol {
                        Token::CommaToken( _ , _ , _ ) => {
                            separators.push(self.lexer.symbol.clone());
                            self.lexer.advance();

                            match &*self.lexer.symbol {
                                Token::PowerToken( _ , _ , _ ) => {
                                    symbol2 = Some(self.lexer.symbol.clone());
                                    self.lexer.advance();

                                    nodes.push(self.parse_test_expr()?);
                                    break
                                },
                                _ => nodes.push(self.parse_test_expr()?)
                            }
                        },
                        _ => break
                    }
                }
            },
            _ => {
                nodes.push(self.parse_test_expr()?);

                'outer: loop {
                    match &*self.lexer.symbol {
                        Token::CommaToken( _ , _ , _ ) => {
                            separators.push(self.lexer.symbol.clone());
                            self.lexer.advance();

                            match &*self.lexer.symbol {
                                Token::PowerToken( _ , _ , _ ) => {
                                    symbol2 = Some(self.lexer.symbol.clone());
                                    self.lexer.advance();
                                    node2 = Some(self.parse_test_expr()?);
                                    break
                                },
                                Token::MultiplyToken( _ , _ , _ ) => {
                                    symbol1 = Some(self.lexer.symbol.clone());
                                    self.lexer.advance();

                                    node1 = match &*self.lexer.symbol {
                                        Token::RightParenToken( _ , _ , _ ) | Token::CommaToken( _ , _ , _ ) => None,
                                        _ => Some(self.parse_test_expr()?)
                                    };

                                    loop {
                                        match &*self.lexer.symbol {
                                            Token::CommaToken( _ , _ , _ ) => {
                                                separators.push(self.lexer.symbol.clone());
                                                self.lexer.advance();

                                                match &*self.lexer.symbol {
                                                    Token::PowerToken( _ , _ , _ ) => {
                                                        symbol2 = Some(self.lexer.symbol.clone());
                                                        self.lexer.advance();
                                                        node2 = Some(self.parse_test_expr()?);
                                                    },
                                                    _ => nodes.push(self.parse_test_expr()?)
                                                }
                                            },
                                            _ => break 'outer
                                        }
                                    }
                                },
                                _ => nodes.push(self.parse_test_expr()?)
                            }
                        },
                        _ => break
                    }
                }
            }
        }

        nodes.reverse();
        separators.reverse();

        Ok(Box::new(SyntaxNode::TypeListStmtNode(pos, self.lexer.position, nodes, separators, symbol1, node1, symbol2, node2)))
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
