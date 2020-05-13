pub mod lexer {
    use crate::{Span, Token, TokenType};
    use plex::lexer;

    lexer! {
        fn next_token(text: 'a) -> (TokenType, &'a str);

        r"( +|\t+|\n)" => (TokenType::Whitespace {
            token: Token::new(text.to_string(), 0, 0, 0, 0)
        }, text),

        "(r\"|\")" => (TokenType::String {
            token: Token::new(text.to_string(), 0, 0, 0, 0)
        }, text),

        r"([0-9]+|[0-9]+\.[0-9]+|'[^']')" => (TokenType::Literal {
            token: Token::new(text.to_string(), 0, 0, 0, 0)
        }, text),

        r"(->|[+-/*%=<>#])" => (TokenType::Operator {
            token: Token::new(text.to_string(), 0, 0, 0, 0)
        }, text),

        r"(:|::|\{|\}|\[|\]|;|,|\)|\()" => (TokenType::Separator {
            token: Token::new(text.to_string(), 0, 0, 0, 0)
        }, text),

        r"(let|fn|type|struct|trait|pub|impl|for|self|Self|mod|use|enum|(iu)(8|16|32)|usize|bool)" => (TokenType::Keyword {
            token: Token::new(text.to_string(), 0, 0, 0, 0)
        }, text),

        r"[^0-9 \t\r\n:+-/*,';<>=%()\[\]{}][^ \t\r\n:+-/*,';<>=%()\[\]{}]*" => (TokenType::Identifier {
            token: Token::new(text.to_string(), 0, 0, 0, 0)
        }, text),

        r"'[^0-9 \t\r\n:+-/*,';<>=%()\[\]{}][^ \t\r\n:+-/*,';<>=%()\[\]{}]*" => (TokenType::Identifier {
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
                    (token_type, text)
                } else {
                    return None;
                };
            match tok {
                (tok, text) => {
                    let line = self.line;
                    if tok.is_new_line() {
                        self.line += 1;
                        self.character = text.len();
                    } else {
                        self.character += text.len();
                    }
                    let span = self.span_in(text);
                    let token = tok.move_to(line, self.character - text.len(), span.lo, span.hi);
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
    use crate::{Token, TokenType};

    use super::*;

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
                token: Token::new("      ".to_string(), 0, 3, 3, 9),
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
                token: Token::new("\n".to_string(), 0, 0, 3, 4),
            },
            TokenType::Identifier {
                token: Token::new("bar".to_string(), 1, 1, 4, 7),
            },
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn must_parse_keyword_let_and_char_literal() {
        let code = "let a = 'b';";
        let lexer = lexer::Lexer::new(code);
        let result: Vec<TokenType> = lexer.map(|pair| pair.0).collect();
        let expected: Vec<TokenType> = vec![
            TokenType::Keyword {
                token: Token::new("let".to_string(), 0, 0, 0, 3),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 0, 3, 3, 4),
            },
            TokenType::Identifier {
                token: Token::new("a".to_string(), 0, 4, 4, 5),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 0, 5, 5, 6),
            },
            TokenType::Operator {
                token: Token::new("=".to_string(), 0, 6, 6, 7),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 0, 7, 7, 8),
            },
            TokenType::Literal {
                token: Token::new("'b'".to_string(), 0, 8, 8, 11),
            },
            TokenType::Separator {
                token: Token::new(";".to_string(), 0, 11, 11, 12),
            },
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn must_parse_keyword_let_and_number_literal() {
        let code = "let a = 684;";
        let lexer = lexer::Lexer::new(code);
        let result: Vec<TokenType> = lexer.map(|pair| pair.0).collect();
        let expected: Vec<TokenType> = vec![
            TokenType::Keyword {
                token: Token::new("let".to_string(), 0, 0, 0, 3),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 0, 3, 3, 4),
            },
            TokenType::Identifier {
                token: Token::new("a".to_string(), 0, 4, 4, 5),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 0, 5, 5, 6),
            },
            TokenType::Operator {
                token: Token::new("=".to_string(), 0, 6, 6, 7),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 0, 7, 7, 8),
            },
            TokenType::Literal {
                token: Token::new("684".to_string(), 0, 8, 8, 11),
            },
            TokenType::Separator {
                token: Token::new(";".to_string(), 0, 11, 11, 12),
            },
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn must_parse_calculation() {
        let code = "let a = b + 684;";
        let lexer = lexer::Lexer::new(code);
        let result: Vec<TokenType> = lexer.map(|pair| pair.0).collect();
        let expected: Vec<TokenType> = vec![
            TokenType::Keyword {
                token: Token::new("let".to_string(), 0, 0, 0, 3),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 0, 3, 3, 4),
            },
            TokenType::Identifier {
                token: Token::new("a".to_string(), 0, 4, 4, 5),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 0, 5, 5, 6),
            },
            TokenType::Operator {
                token: Token::new("=".to_string(), 0, 6, 6, 7),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 0, 7, 7, 8),
            },
            TokenType::Identifier {
                token: Token::new("b".to_string(), 0, 8, 8, 9),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 0, 9, 9, 10),
            },
            TokenType::Operator {
                token: Token::new("+".to_string(), 0, 10, 10, 11),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 0, 11, 11, 12),
            },
            TokenType::Literal {
                token: Token::new("684".to_string(), 0, 12, 12, 15),
            },
            TokenType::Separator {
                token: Token::new(";".to_string(), 0, 15, 15, 16),
            },
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn must_parse_function() {
        let code = r"fn foo(a: i32, b: i32) -> int {
            a % b
        }";
        let lexer = lexer::Lexer::new(code);
        let result: Vec<TokenType> = lexer.map(|pair| pair.0).collect();
        let expected: Vec<TokenType> = vec![
            TokenType::Keyword {
                token: Token::new("fn".to_string(), 0, 0, 0, 2),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 0, 2, 2, 3),
            },
            TokenType::Identifier {
                token: Token::new("foo".to_string(), 0, 3, 3, 6),
            },
            TokenType::Separator {
                token: Token::new("(".to_string(), 0, 6, 6, 7),
            },
            TokenType::Identifier {
                token: Token::new("a".to_string(), 0, 7, 7, 8),
            },
            TokenType::Separator {
                token: Token::new(":".to_string(), 0, 8, 8, 9),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 0, 9, 9, 10),
            },
            TokenType::Identifier {
                token: Token::new("i32".to_string(), 0, 10, 10, 13),
            },
            TokenType::Operator {
                token: Token::new(",".to_string(), 0, 13, 13, 14),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 0, 14, 14, 15),
            },
            TokenType::Identifier {
                token: Token::new("b".to_string(), 0, 15, 15, 16),
            },
            TokenType::Separator {
                token: Token::new(":".to_string(), 0, 16, 16, 17),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 0, 17, 17, 18),
            },
            TokenType::Identifier {
                token: Token::new("i32".to_string(), 0, 18, 18, 21),
            },
            TokenType::Separator {
                token: Token::new(")".to_string(), 0, 21, 21, 22),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 0, 22, 22, 23),
            },
            TokenType::Operator {
                token: Token::new("->".to_string(), 0, 23, 23, 25),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 0, 25, 25, 26),
            },
            TokenType::Identifier {
                token: Token::new("int".to_string(), 0, 26, 26, 29),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 0, 29, 29, 30),
            },
            TokenType::Separator {
                token: Token::new("{".to_string(), 0, 30, 30, 31),
            },
            TokenType::Whitespace {
                token: Token::new("\n".to_string(), 0, 0, 31, 32),
            },
            TokenType::Whitespace {
                token: Token::new("            ".to_string(), 1, 1, 32, 44),
            },
            TokenType::Identifier {
                token: Token::new("a".to_string(), 1, 13, 44, 45),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 1, 14, 45, 46),
            },
            TokenType::Operator {
                token: Token::new("%".to_string(), 1, 15, 46, 47),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 1, 16, 47, 48),
            },
            TokenType::Identifier {
                token: Token::new("b".to_string(), 1, 17, 48, 49),
            },
            TokenType::Whitespace {
                token: Token::new("\n".to_string(), 1, 0, 49, 50),
            },
            TokenType::Whitespace {
                token: Token::new("        ".to_string(), 2, 1, 50, 58),
            },
            TokenType::Separator {
                token: Token::new("}".to_string(), 2, 9, 58, 59),
            },
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn must_parse_derive() {
        let buffer = "#[derive(Debug, Clone)]";
        let result: Vec<TokenType> = lexer::Lexer::new(buffer).map(|p| p.0).collect();
        let expected: Vec<TokenType> = vec![
            TokenType::Operator {
                token: Token::new("#".to_string(), 0, 0, 0, 1),
            },
            TokenType::Separator {
                token: Token::new("[".to_string(), 0, 1, 1, 2),
            },
            TokenType::Identifier {
                token: Token::new("derive".to_string(), 0, 2, 2, 8),
            },
            TokenType::Separator {
                token: Token::new("(".to_string(), 0, 8, 8, 9),
            },
            TokenType::Identifier {
                token: Token::new("Debug".to_string(), 0, 9, 9, 14),
            },
            TokenType::Operator {
                token: Token::new(",".to_string(), 0, 14, 14, 15),
            },
            TokenType::Whitespace {
                token: Token::new(" ".to_string(), 0, 15, 15, 16),
            },
            TokenType::Identifier {
                token: Token::new("Clone".to_string(), 0, 16, 16, 21),
            },
            TokenType::Separator {
                token: Token::new(")".to_string(), 0, 21, 21, 22),
            },
            TokenType::Separator {
                token: Token::new("]".to_string(), 0, 22, 22, 23),
            },
        ];
        assert_eq!(result, expected);
    }
}
