pub mod lexer {
    use crate::{Span, Token, TokenType};
    use plex::lexer;

    lexer! {
        fn next_token(text: 'a) -> (TokenType, &'a str);

        r"[ \t\r\n]" => (TokenType::Whitespace {
            token: Token::new(text.to_string(), 0, 0, 0, 0)
        }, text),

        r"[^ \t\r\n]+" => (TokenType::Identifier {
            token: Token::new(text.to_string(), 0, 0, 0, 0)
        }, text),
    }

    pub struct Lexer<'a> {
        original: &'a str,
        remaining: &'a str,
        line: usize,
        character: usize,
    }

    impl<'a> Lexer<'a> {
        pub fn new(s: &'a str) -> Self {
            Self {
                original: s,
                remaining: s,
                line: 0,
                character: 0,
            }
        }
    }

    impl<'a> Iterator for Lexer<'a> {
        type Item = (TokenType, Span);

        fn next(&mut self) -> Option<(TokenType, Span)> {
            let tok: (TokenType, &str) =
                if let Some(((token_type, text), new_remaining)) = next_token(self.remaining) {
                    self.remaining = new_remaining;
                    if token_type.is_new_line() {
                        self.line += 1;
                        self.character = text.len();
                    } else {
                        self.character += text.len();
                    }
                    (token_type, text)
                } else {
                    return None;
                };
            match tok {
                (tok, text) => {
                    let span = self.span_in(text);
                    let token =
                        tok.move_to(self.line, self.character - text.len(), span.lo, span.hi);
                    Some((token, span))
                }
            }
        }
    }

    impl<'a> Lexer<'a> {
        fn span_in(&self, s: &str) -> Span {
            let lo = s.as_ptr() as usize - self.original.as_ptr() as usize;
            Span {
                lo,
                hi: lo + s.len(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Token, TokenType};

    #[test]
    fn must_parse_simple_text() {
        let code = "foo";
        let lexer = lexer::Lexer::new(code);
        let result: Vec<TokenType> = lexer.map(|pair| pair.0).collect();
        let expected: Vec<TokenType> = vec![TokenType::Identifier {
            token: Token::new("foo".to_string(), 0, 0, 0, 3),
        }];
        assert_eq!(result, expected);
    }

    #[test]
    fn must_parse_long_text() {
        let code = "foobarhelloworldexamplecomtesttest";
        let lexer = lexer::Lexer::new(code);
        let result: Vec<TokenType> = lexer.map(|pair| pair.0).collect();
        let expected: Vec<TokenType> = vec![TokenType::Identifier {
            token: Token::new(
                "foobarhelloworldexamplecomtesttest".to_string(),
                0,
                0,
                0,
                34,
            ),
        }];
        assert_eq!(result, expected);
    }

    #[test]
    fn must_parse_text_with_space() {
        let code = "foo bar";
        let lexer = lexer::Lexer::new(code);
        let result: Vec<TokenType> = lexer.map(|pair| pair.0).collect();
        let expected: Vec<TokenType> = vec![
            TokenType::Identifier {
                token: Token::new("foo".to_string(), 0, 0, 0, 3),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 0, 3, 3, 4),
            },
            TokenType::Identifier {
                token: Token::new("bar".to_string(), 0, 4, 4, 7),
            },
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn must_parse_text_with_one_multi_character_space() {
        let code = "foo      bar";
        let lexer = lexer::Lexer::new(code);
        let result: Vec<TokenType> = lexer.map(|pair| pair.0).collect();
        let expected: Vec<TokenType> = vec![
            TokenType::Identifier {
                token: Token::new("foo".to_string(), 0, 0, 0, 3),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 0, 3, 3, 4),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 0, 4, 4, 5),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 0, 5, 5, 6),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 0, 6, 6, 7),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 0, 7, 7, 8),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 0, 8, 8, 9),
            },
            TokenType::Identifier {
                token: Token::new("bar".to_string(), 0, 9, 9, 12),
            },
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn must_parse_text_with_multiple_spaces() {
        let code = "foo bar hello world";
        let lexer = lexer::Lexer::new(code);
        let result: Vec<TokenType> = lexer.map(|pair| pair.0).collect();
        let expected: Vec<TokenType> = vec![
            TokenType::Identifier {
                token: Token::new("foo".to_string(), 0, 0, 0, 3),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 0, 3, 3, 4),
            },
            TokenType::Identifier {
                token: Token::new("bar".to_string(), 0, 4, 4, 7),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 0, 7, 7, 8),
            },
            TokenType::Identifier {
                token: Token::new("hello".to_string(), 0, 8, 8, 13),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 0, 13, 13, 14),
            },
            TokenType::Identifier {
                token: Token::new("world".to_string(), 0, 14, 14, 19),
            },
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn must_parse_text_with_new_line() {
        let code = "foo\nbar";
        let lexer = lexer::Lexer::new(code);
        let result: Vec<TokenType> = lexer.map(|pair| pair.0).collect();
        let expected: Vec<TokenType> = vec![
            TokenType::Identifier {
                token: Token::new("foo".to_string(), 0, 0, 0, 3),
            },
            TokenType::Whitespace {
                token: Token::new("\n".to_string(), 1, 0, 3, 4),
            },
            TokenType::Identifier {
                token: Token::new("bar".to_string(), 1, 1, 4, 7),
            },
        ];
        assert_eq!(result, expected);
    }
}
