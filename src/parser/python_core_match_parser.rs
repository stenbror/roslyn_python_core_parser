use crate::parser::python_core_block_parser::BlockGrammarRules;
use crate::parser::python_core_expression_parser::ExpressionRules;
use crate::parser::python_core_parser::PythonCoreParser;
use crate::parser::python_core_statement_parser::StatementRules;
use crate::parser::python_core_tokenizer::LexerMethods;
use crate::parser::syntax_error::SyntaxError;
use crate::parser::syntax_nodes::SyntaxNode;
use crate::parser::token_nodes::Token;
use crate::parser::token_nodes::Token::DefaultToken;

pub(crate) trait MatchPatternRules {
    fn parse_match_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_subject_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_case_block(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_guard_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_patterns(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_as_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_or_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_closed_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;

    fn parse_wildcard_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_sequence_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_mappings_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_key_value_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_attr_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_power_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;


    fn parse_class_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;

    fn parse_capture_target(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;

    fn parse_open_sequence_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_star_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
}

impl MatchPatternRules for PythonCoreParser {
    fn parse_match_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut nodes = Vec::<Box<SyntaxNode>>::new();

        let symbol1 = match &*self.lexer.symbol {
            Token::NameToken( s, e, text, t) => {
                match text.as_str() {
                    "match" => Box::new(Token::MatchToken(*s, *e, t.clone())),
                    _ => return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting 'match' keyword in match statement!"))))
                }
            },
            _ => return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting 'match' keyword in match statement!"))))
        };

        let right = self.parse_subject_expr()?;

        match &*self.lexer.symbol {
            Token::ColonToken( _ , _ , _ ) => {
                let symbol2 = self.lexer.symbol.clone();
                self.lexer.advance();

                let symbol3 = match &*self.lexer.symbol {
                    Token::NewlineToken( _ , _ , _ , _ , _ ) => {
                        let symbol10 = self.lexer.symbol.clone();
                        self.lexer.advance();
                        symbol10
                    },
                    _ => return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting NEWLINE in 'match' statement!"))))
                };

                match &*self.lexer.symbol {
                    Token::IndentToken( _ , _ , _ ) => {
                        let symbol4 = self.lexer.symbol.clone();
                        self.lexer.advance();

                        /* First case must be available */
                        match &*self.lexer.symbol {
                            Token::NameToken( _ , _ , text, _ ) => {
                                if text.as_str() == "case" {
                                    nodes.push(self.parse_case_block()?)
                                }
                                else {
                                    return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting at least one 'case' keyword in 'match' statement!"))))
                                }
                            },
                            _ => return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting at least one 'case' keyword in 'match' statement!"))))
                        }

                        /* Optional rest of cases */
                        loop {
                            match &*self.lexer.symbol {
                                Token::NameToken( _ , _ , text, _ ) => {
                                    if text.as_str() == "case" {
                                        nodes.push(self.parse_case_block()?)
                                    }
                                    else {
                                        break;
                                    }
                                },
                                _ => break
                            }
                        }

                        /* Make sure we have a dedent after all case blocks. */
                        let symbol5 = match &*self.lexer.symbol {
                            Token::DedentToken( _ , _ , _ ) => {
                                let symbol11 = self.lexer.symbol.clone();
                                self.lexer.advance();
                                symbol11
                            },
                            _ => return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting DEDENT in 'match' statement!"))))
                        };

                        nodes.reverse();

                        Ok(Box::new(SyntaxNode::MatchStmtNode(pos, self.lexer.position, symbol1, right, symbol2, symbol3, symbol4, nodes, symbol5)))
                    },
                    _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting INDENT in 'match' statement!"))))
                }
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting ':' in 'match' statement!"))))
        }
    }

    fn parse_subject_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        match &*self.lexer.symbol {
            Token::MultiplyToken( _ , _ , _ ) => self.parse_star_expr_named_exp_elements(),
            _ => self.parse_named_expr()
        }
    }

    fn parse_case_block(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;

        let symbol1 = match &*self.lexer.symbol {
            Token::NameToken( s, e, text, t) => {
                match text.as_str() {
                    "case" => Box::new(Token::CaseToken(*s, *e, t.clone())),
                    _ => return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting 'case' keyword in 'case' block!"))))
                }
            },
            _ => return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting 'case' keyword in 'case' block!"))))
        };

        let left = self.parse_patterns()?;

        let guard = match &*self.lexer.symbol {
            Token::IfToken( _ , _ , _ ) => Some(self.parse_guard_expr()?),
            _ => None
        };

        match &*self.lexer.symbol {
            Token::ColonToken( _ , _ , _ ) => {
                let symbol2 = self.lexer.symbol.clone();
                self.lexer.advance();

                let right = self.parse_suite_stmt()?;

                Ok(Box::new(SyntaxNode::CaseElementStmtNode(pos, self.lexer.position, symbol1, left, guard, symbol2, right)))
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting ':' in 'case' block!"))))
        }
    }

    fn parse_guard_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;

        match &*self.lexer.symbol {
            Token::IfToken( _ , _ , _ ) => {
                let symbol1 = self.lexer.symbol.clone();
                self.lexer.advance();

                let right = self.parse_named_expr()?;

                Ok(Box::new(SyntaxNode::GuardElementStmtNode(pos, self.lexer.position, symbol1, right)))
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting 'if' keyword in 'case' guard!"))))
        }
    }

    fn parse_patterns(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        match &*self.lexer.symbol {
            Token::MultiplyToken( _ , _ , _ ) => self.parse_open_sequence_pattern(),
            _ => self.parse_as_pattern()
        }
    }

    fn parse_as_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let left = self.parse_or_pattern()?;

        match &*self.lexer.symbol {
            Token::AsToken( _ , _ , _ ) => {
                let symbol1 = self.lexer.symbol.clone();
                self.lexer.advance();

                let right = self.parse_capture_target()?;
                Ok(Box::new(SyntaxNode::MatchAsPattern(pos, self.lexer.position, left, symbol1, right)))
            },
            _ => Ok(left)
        }
    }

    fn parse_or_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut nodes = Vec::<Box<SyntaxNode>>::new();
        let mut separators = Vec::<Box<Token>>::new();

        let left = self.parse_closed_pattern()?;

        loop {
            match &*self.lexer.symbol {
                Token::BitOrToken( _ , _ , _ ) => {
                    separators.push(self.lexer.symbol.clone());
                    self.lexer.advance();

                    nodes.push(self.parse_closed_pattern()?);
                },
                _ => break
            }
        }

        match separators.len() {
            0 => Ok(left), /* No or '|' patterns found */
            _ => {
                nodes.reverse();
                separators.reverse();

                Ok(Box::new(SyntaxNode::MatchOrPatterns(pos, self.lexer.position, left, separators, nodes)))
            }
        }
    }

    fn parse_closed_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        match &*self.lexer.symbol {
            Token::FalseToken( _ , _ , _ ) |
            Token::TrueToken( _ , _ , _ ) |
            Token::NoneToken( _ , _ , _ ) |
            Token::StringToken( _ , _ , _ , _ ) => self.parse_atom_expr(),
            Token::NameToken( _ , _ , text , _ ) => {
                match text.as_str() {
                    "_" => self.parse_wildcard_pattern(),
                    _ => self.parse_class_pattern() // Handle calue, capture, class
                }
            },
            Token::MinusToken( _ , _ , _ ) |
            Token::NumberToken( _ , _ , _ , _ ) => {
                let minus = match &*self.lexer.symbol {
                    Token::MinusToken( _ , _ , _ ) => {
                        let symbol1 = self.lexer.symbol.clone();
                        self.lexer.advance();
                        Some(symbol1)
                    },
                    _ => None
                };

                let left = match &*self.lexer.symbol {
                    Token::NumberToken( _ , _ , _ , _ ) => self.parse_atom_expr()?,
                    _ => return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting number in closed pattern!"))))
                };

                match &*self.lexer.symbol {
                    Token::PlusToken( _ , _ , _ ) |
                    Token::MinusToken( _ , _ , _ ) => {
                        let symbol = self.lexer.symbol.clone();
                        self.lexer.advance();

                        let right = match &*self.lexer.symbol {
                            Token::NumberToken( _ , _ , _ , _ ) => self.parse_atom_expr()?,
                            _ => return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting number in closed pattern!"))))
                        };

                        Ok(Box::new(SyntaxNode::SignedImaginaryNumberNode(pos, self.lexer.position, minus, left, symbol, right)))
                    },
                    _ => Ok(Box::new(SyntaxNode::SignedNumberNode(pos, self.lexer.position, minus, left)))
                }
            },
            Token::LeftCurlyBracketToken( _ , _ , _ ) => self.parse_mappings_pattern(),
            Token::LeftParenToken( _ , _ , _ ) |
            Token::LeftSquareBracketToken( _ , _ , _ ) => self.parse_sequence_pattern(),
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting pattern!"))))
        }
    }

    fn parse_wildcard_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        match &*self.lexer.symbol {
            Token::NameToken( s , e , text, t ) => {
                match text.as_str() {
                    "_" => {
                        let symbol1 = Box::new(DefaultToken(*s, *e, t.clone()));
                        self.lexer.advance();

                        Ok(Box::new(SyntaxNode::DefaultPatterNode(pos, self.lexer.position, symbol1)))
                    },
                    _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting '_' in wildcard pattern!"))))
                }
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting '_' in wildcard pattern!"))))
        }
    }

    fn parse_sequence_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_mappings_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut nodes = Vec::<Box<SyntaxNode>>::new();
        let mut separators = Vec::<Box<Token>>::new();

        match &*self.lexer.symbol {
            Token::LeftCurlyBracketToken( _ , _ , _ ) => {
                let symbol1 = self.lexer.symbol.clone();
                self.lexer.advance();

                match &*self.lexer.symbol {
                    Token::RightCurlyBracketToken( _ , _ , _ ) => { /* Empty mapping */
                        let symbol2 = self.lexer.symbol.clone();
                        self.lexer.advance();

                        Ok(Box::new(SyntaxNode::MappingPatternNode(pos, self.lexer.position, symbol1, Vec::new(), Vec::new(), symbol2)))
                    },
                    Token::PowerToken( _ , _ , _ ) => { /* Single '**' element with optional ',' */
                        nodes.push(self.parse_power_argument_element()?);

                        match &*self.lexer.symbol {
                            Token::CommaToken( _ , _ , _ ) => {
                                separators.push(self.lexer.symbol.clone());
                                self.lexer.advance()
                            },
                            _ => ()
                        }

                        let symbol2 = match &*self.lexer.symbol {
                            Token::RightCurlyBracketToken( _ , _ , _ ) => {
                                let symbol10 = self.lexer.symbol.clone();
                                self.lexer.advance();
                                symbol10
                            },
                            _ => return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting '}' in mappings pattern!"))))
                        };

                        Ok(Box::new(SyntaxNode::MappingPatternNode(pos, self.lexer.position, symbol1, nodes, separators, symbol2)))
                    },
                    _ => { /* elements */

                        nodes.push(self.parse_key_value_pattern()?);

                        loop {
                            match &*self.lexer.symbol {
                                Token::CommaToken( _ , _ , _ ) => {
                                    separators.push(self.lexer.symbol.clone());
                                    self.lexer.advance();

                                    match &*self.lexer.symbol {
                                        Token::RightParenToken( _ , _ , _ ) => break,
                                        Token::PowerToken( _ , _ , _ ) => {
                                            nodes.push(self.parse_power_argument_element()?);

                                            match &*self.lexer.symbol {
                                                Token::CommaToken( _ , _ , _ ) => {
                                                    separators.push(self.lexer.symbol.clone());
                                                    self.lexer.advance()
                                                },
                                                _ => ()
                                            }

                                            break
                                        },
                                        _ => nodes.push(self.parse_key_value_pattern()?)
                                    }
                                },
                                _ => break
                            }
                        }

                        nodes.reverse();
                        separators.reverse();

                        let symbol2 = match &*self.lexer.symbol {
                            Token::RightCurlyBracketToken( _ , _ , _ ) => {
                                let symbol10 = self.lexer.symbol.clone();
                                self.lexer.advance();
                                symbol10
                            },
                            _ => return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting '}' in mappings pattern!"))))
                        };

                        Ok(Box::new(SyntaxNode::MappingPatternNode(pos, self.lexer.position, symbol1, nodes, separators, symbol2)))
                    }
                }
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting '{' in mappings pattern!"))))
        }
    }

    fn parse_key_value_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;

        let left = match &*self.lexer.symbol {
            Token::NameToken( _ , _ , _ , _ ) => self.parse_attr_pattern()?,
            Token::NoneToken( _ , _ , _ ) |
            Token::FalseToken( _ , _ , _ ) |
            Token::TrueToken( _ , _ , _ ) |
            Token::StringToken( _ , _ , _ , _ ) => self.parse_atom_expr()?,
            Token::MinusToken( _ , _ , _ ) |
            Token::NumberToken( _ , _ , _ , _ ) => {

                let minus = match &*self.lexer.symbol {
                    Token::MinusToken( _ , _ , _ ) => {
                        let symbol1 = self.lexer.symbol.clone();
                        self.lexer.advance();
                        Some(symbol1)
                    },
                    _ => None
                };

                let left = match &*self.lexer.symbol {
                    Token::NumberToken( _ , _ , _ , _ ) => self.parse_atom_expr()?,
                    _ => return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting number in closed pattern!"))))
                };

                match &*self.lexer.symbol {
                    Token::PlusToken( _ , _ , _ ) |
                    Token::MinusToken( _ , _ , _ ) => {
                        let symbol = self.lexer.symbol.clone();
                        self.lexer.advance();

                        let right = match &*self.lexer.symbol {
                            Token::NumberToken( _ , _ , _ , _ ) => self.parse_atom_expr()?,
                            _ => return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting number in closed pattern!"))))
                        };

                        Box::new(SyntaxNode::SignedImaginaryNumberNode(pos, self.lexer.position, minus, left, symbol, right))
                    },
                    _ => Box::new(SyntaxNode::SignedNumberNode(pos, self.lexer.position, minus, left))
                }
            },
            _ => return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting 'key' pattern in mappings pattern!"))))
        };

        let symbol = match &*self.lexer.symbol {
            Token::ColonToken( _ , _ , _ ) => {
                let symbol10 = self.lexer.symbol.clone();
                self.lexer.advance();
                symbol10
            },
            _ => return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting ':' in mappings pattern!"))))
        };

        let right = self.parse_as_pattern()?;

        Ok(Box::new(SyntaxNode::KeyValuePatternNode(pos, self.lexer.position, left, symbol, right)))
    }

    fn parse_attr_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut nodes = Vec::<Box<SyntaxNode>>::new();
        let mut separators = Vec::<Box<Token>>::new();

        match &*self.lexer.symbol {
            Token::NameToken( _ , _ , _ , _ ) => {
                nodes.push(self.parse_atom_expr()?);

                loop {
                    match &*self.lexer.symbol {
                        Token::PeriodToken( _ , _ , _ ) => {
                            separators.push(self.lexer.symbol.clone());
                            self.lexer.advance();

                            match &*self.lexer.symbol {
                                Token::NameToken( _ , _ , _ , _ ) => nodes.push(self.parse_atom_expr()?),
                                _ => return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting 'name' in 'key' pattern in mappings pattern!"))))
                            }
                        },
                        _ => break
                    }
                }

                nodes.reverse();
                separators.reverse();

                Ok(Box::new(SyntaxNode::NameAttributeNode(pos, self.lexer.position, nodes, separators)))
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting NAME in 'key' pattern in mappings pattern!"))))
        }
    }

    fn parse_power_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        match &*self.lexer.symbol {
            Token::PowerToken( _ , _ , _ ) => {
                let symbol1 = self.lexer.symbol.clone();
                self.lexer.advance();

                let right = self.parse_capture_target()?;

                Ok(Box::new(SyntaxNode::DoubleStarPatterNode(pos, self.lexer.position, symbol1, right)))
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting '**' in power pattern!"))))
        }
    }


    fn parse_class_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }


    fn parse_capture_target(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        match &*self.lexer.symbol {
            Token::NameToken( _ , _ , text, _ ) => {
                match text.as_str() {
                    "_" => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Cannot capture '_' in 'as' pattern!")))),
                    _ => {
                        let right = self.parse_atom_expr()?;

                        match &*self.lexer.symbol {
                            Token::PeriodToken( _ , _ , _ ) |
                            Token::LeftParenToken( _ , _ , _ ) |
                            Token::AssignToken( _ , _ , _ ) => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Cannot have '.', '(' or '=' after NAME in 'as' pattern!")))),
                            _ => Ok(right)
                        }
                    }
                }
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting variable name in 'as' pattern!"))))
        }
    }

    fn parse_open_sequence_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut nodes = Vec::<Box<SyntaxNode>>::new();
        let mut separators = Vec::<Box<Token>>::new();

        nodes.push(match &*self.lexer.symbol { Token::MultiplyToken( _ , _ , _ ) => self.parse_star_pattern()?, _ => self.parse_as_pattern()?});

        loop {
            match &*self.lexer.symbol {
                Token::CommaToken( _ , _ , _ ) => {
                    separators.push(self.lexer.symbol.clone());
                    self.lexer.advance();

                    match &*self.lexer.symbol {
                        Token::IfToken( _ , _ , _ ) |
                        Token::ColonToken( _ , _ , _ ) => break,
                        _ => nodes.push(match &*self.lexer.symbol { Token::MultiplyToken( _ , _ , _ ) => self.parse_star_pattern()?, _ => self.parse_as_pattern()?})
                    }
                },
                _ => break
            }
        }

        nodes.reverse();
        separators.reverse();

        Ok(Box::new(SyntaxNode::OpenSequencePatternNode(pos, self.lexer.position, nodes, separators)))
    }

    fn parse_star_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;

        match &*self.lexer.symbol {
            Token::MultiplyToken( _ , _ , _ ) => {
                let symbol1 = self.lexer.symbol.clone();
                self.lexer.advance();

                match &*self.lexer.symbol {
                    Token::NameToken( s , e , text, t ) => {
                        match text.as_str() {
                            "_" => {
                                let symbol2 = Box::new(DefaultToken(*s, *e, t.clone()));
                                self.lexer.advance();

                                let right = Box::new(SyntaxNode::DefaultPatterNode(pos, self.lexer.position, symbol2));

                                Ok(Box::new(SyntaxNode::StarPatternNode(pos, self.lexer.position, symbol1, right)))
                            },
                            _ => {
                                let right = self.parse_atom_expr()?;

                                Ok(Box::new(SyntaxNode::StarPatternNode(pos, self.lexer.position, symbol1, right)))
                            }
                        }
                    },
                    _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting variable name in '*' pattern!"))))
                }

            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting '*' in '*' pattern!"))))
        }
    }
}

// Unittests for match grammar rules ///////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    #[test]
    fn dummy_test() {
        assert_eq!(1 == 1, true);
    }
}