use crate::parser::python_core_expression_parser::ExpressionRules;
use crate::parser::python_core_statement_parser::StatementRules;
use crate::parser::python_core_tokenizer::LexerMethods;
use crate::parser::syntax_error::SyntaxError;
use crate::parser::syntax_nodes::SyntaxNode;
use crate::parser::token_nodes::Token;
use super::python_core_parser::PythonCoreParser;

pub(crate) trait MatchPatternRules {
    fn parse_match_stmt(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_subject_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_case_block(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_guard(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_patterns(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_as_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_or_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_closed_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_literal_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_literal_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_complex_number(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_signed_number(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_signed_real_number(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_real_number(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_imaginary_number(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_capture_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_pattern_capture_target(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_wildcard_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_value_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_attribute_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_name_or_attr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_group_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_sequence_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_open_sequence_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_maybe_sequence_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_maybe_star_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_star_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_mapping_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_key_value_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_double_star_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_class_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_positional_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_keywords_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
    fn parse_keyword_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>>;
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

                        let symbol5 = self.lexer.symbol.clone();
                        self.lexer.advance();

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
        let pos = self.lexer.position;
        match &*self.lexer.symbol {
            Token::MultiplyToken( _ , _ , _ ) => {
                let left = self.parse_star_expr()?;

                match &*self.lexer.symbol {
                    Token::CommaToken( _ , _ , _ ) => {
                        let symbol = self.lexer.symbol.clone();
                        self.lexer.advance();

                        let right = self.parse_star_expr()?;

                        Ok(Box::new(SyntaxNode::SubjectExprNode(pos, self.lexer.position, left, Some(symbol), Some(right))))
                    },
                    _ => Ok(Box::new(SyntaxNode::SubjectExprNode(pos, self.lexer.position, left, None, None)))
                }
            },
            _ => {
                let left = self.parse_named_expr()?;
                Ok(Box::new(SyntaxNode::SubjectExprNode(pos, self.lexer.position, left, None, None)))
            }
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
            Token::IfToken( _ , _ , _ ) => Some(self.parse_guard()?),
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

    fn parse_guard(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
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
        todo!()
    }

    fn parse_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_as_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let left = self.parse_or_pattern()?;

        match &*self.lexer.symbol {
            Token::AsToken( _ , _ , _ ) => {
                let symbol = self.lexer.symbol.clone();
                self.lexer.advance();
                let right = self.parse_capture_pattern()?;

                Ok(Box::new(SyntaxNode::MatchAsPattern(pos, self.lexer.position, left, symbol, right)))
            },
            _ => Ok(left)
        }
    }

    fn parse_or_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        let mut nodes = Vec::<Box<SyntaxNode>>::new();
        let mut symbols = Vec::<Box<Token>>::new();

        let left = self.parse_closed_pattern()?;

        loop {
            match &*self.lexer.symbol {
                Token::BitOrToken( _ , _ , _ ) => {
                     symbols.push(self.lexer.symbol.clone());
                    self.lexer.advance();

                    nodes.push(self.parse_closed_pattern()?)
                },
                _ => break
            }
        }

        nodes.reverse();
        symbols.reverse();

        match &symbols.len() {
            1 => Ok(left),
            _ => Ok(Box::new(SyntaxNode::MatchOrPatterns(pos, self.lexer.position, left, symbols, nodes)))
        }
    }

    fn parse_closed_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_literal_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_literal_expr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_complex_number(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_signed_number(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_signed_real_number(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_real_number(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_imaginary_number(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_capture_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_pattern_capture_target(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_wildcard_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;

        match &*self.lexer.symbol {
            Token::NameToken( s , e , text, t ) => {
                match text.as_str() {
                    "_" => {
                        let symbol = Box::new(Token::DefaultToken(*s, *e, t.clone()));
                        self.lexer.advance();
                        Ok(Box::new(SyntaxNode::DefaultPatterNode(pos, self.lexer.position, symbol)))
                    },
                    _ => return Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting '_' in 'wildcard' pattern!"))))
                }
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting '_' in 'wildcard' pattern!"))))
        }
    }

    fn parse_value_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_attribute_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_name_or_attr(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_group_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_sequence_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_open_sequence_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_maybe_sequence_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_maybe_star_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_star_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_mapping_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;
        match &*self.lexer.symbol {
            Token::LeftCurlyBracketToken( _ , _ , _ ) => {
                let symbol1 = self.lexer.symbol.clone();
                self.lexer.advance();

                let mut nodes = Vec::<Box<SyntaxNode>>::new();
                let mut separators = Vec::<Box<Token>>::new();

                match &*self.lexer.symbol {
                    Token::RightCurlyBracketToken( _ , _ , _ ) => { /* Empty mapping structure */
                        let symbol2 = self.lexer.symbol.clone();
                        self.lexer.advance();

                        Ok(Box::new(SyntaxNode::MappingPatternNode(pos, self.lexer.position, symbol1, nodes, separators, symbol2)))
                    },
                    Token::PowerToken( _ , _ , _ ) => { /* Mapping structure with double star pattern only */
                        nodes.push(self.parse_double_star_pattern()?);

                        match &*self.lexer.symbol {
                            Token::CommaToken( _ , _ , _ ) => {
                                separators.push(self.lexer.symbol.clone());
                                self.lexer.advance()
                            },
                            _ => ()
                        }

                        match &*self.lexer.symbol {
                            Token::RightCurlyBracketToken( _ , _ , _ ) => {
                                let symbol2 = self.lexer.symbol.clone();
                                self.lexer.advance();

                                Ok(Box::new(SyntaxNode::MappingPatternNode(pos, self.lexer.position, symbol1, nodes, separators, symbol2)))
                            },
                            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting '}' in 'mapping' pattern!"))))
                        }
                    },
                    _ => { /* More mappings pattern given */
                        nodes.push(self.parse_keyword_pattern()?);

                        loop {
                            match &*self.lexer.symbol {
                                Token::CommaToken( _ , _ , _ ) => {
                                    separators.push(self.lexer.symbol.clone());
                                    self.lexer.advance();

                                    match &*self.lexer.symbol {
                                        Token::PowerToken( _ , _ , _ ) => {
                                            nodes.push(self.parse_double_star_pattern()?);

                                            match &*self.lexer.symbol {
                                                Token::CommaToken( _ , _ , _ ) => {
                                                    separators.push(self.lexer.symbol.clone());
                                                    self.lexer.advance()
                                                },
                                                _ => ()
                                            }

                                            break
                                        },
                                        Token::RightCurlyBracketToken( _ , _ , _ ) => break,
                                        _ => nodes.push(self.parse_keyword_pattern()?)
                                    }
                                },
                                _ => break
                            }
                        }

                        match &*self.lexer.symbol {
                            Token::RightCurlyBracketToken( _ , _ , _ ) => {
                                let symbol2 = self.lexer.symbol.clone();
                                self.lexer.advance();

                                Ok(Box::new(SyntaxNode::MappingPatternNode(pos, self.lexer.position, symbol1, nodes, separators, symbol2)))
                            },
                            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting '}' in 'mapping' pattern!"))))
                        }
                    }
                }
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting '{' in 'mapping' pattern!"))))
        }
    }

    fn parse_key_value_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;

        let left = match &*self.lexer.symbol {
            Token::NameToken( _ , _ , _ , _ ) => self.parse_name_or_attr()?,
            _ => self.parse_literal_expr()?
        };

        match &*self.lexer.symbol {
            Token::ColonToken( _ , _ , _ ) => {
                let symbol1 = self.lexer.symbol.clone();
                self.lexer.advance();

                let right = self.parse_pattern()?;

                Ok(Box::new(SyntaxNode::KeyValuePatternNode(pos, self.lexer.position, left, symbol1, right)))
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting ':' in 'key-value' pattern!"))))
        }
    }

    fn parse_double_star_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;

        match &*self.lexer.symbol {
            Token::PowerToken( _ , _ , _ ) => {
                let symbol1 = self.lexer.symbol.clone();
                self.lexer.advance();

                let right = self.parse_pattern_capture_target()?;

                Ok(Box::new(SyntaxNode::DoubleStarPatterNode(pos, self.lexer.position, symbol1, right)))
            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting '**' in 'double star' pattern!"))))
        }
    }

    fn parse_class_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_positional_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_keywords_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        todo!()
    }

    fn parse_keyword_pattern(&mut self) -> Result<Box<SyntaxNode>, Box<SyntaxError>> {
        let pos = self.lexer.position;

        match &*self.lexer.symbol {
            Token::NameToken( _ , _ , _ , _ ) => {
                let symbol1 = self.lexer.symbol.clone();
                self.lexer.advance();

                match &*self.lexer.symbol {
                    Token::AssignToken( _ , _ , _ ) => {
                        let symbol2 = self.lexer.symbol.clone();
                        self.lexer.advance();

                        let right = self.parse_pattern()?;

                        Ok(Box::new(SyntaxNode::KeywordPatternNode(pos, self.lexer.position, symbol1, symbol2, right)))
                    },
                    _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting '=' in 'keyword' pattern!"))))
                }

            },
            _ => Err(Box::new(SyntaxError::new(self.lexer.position, String::from("Expecting NAME in 'keyword' pattern!"))))
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
