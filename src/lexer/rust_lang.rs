use crate::lexer::{Token, TokenType};

pub mod lexer {
    use crate::lexer::{Span, Token, TokenType};
    use plex::lexer;

    lexer! {
        fn next_token(text: 'a) -> (TokenType, &'a str);

        r"( +|\t+|\n+)" => (TokenType::Whitespace {
            token: Token::new(text.to_string(), 0, 0, 0, 0)
        }, text),

        r"(\d+|\d+\.\d+|'[\S]')" => (TokenType::Literal {
            token: Token::new(text.to_string(), 0, 0, 0, 0)
        }, text),

        r"[+-/*%=<>]" => (TokenType::Operator {
            token: Token::new(text.to_string(), 0, 0, 0, 0)
        }, text),

        r"(:|::|\{|\}|\[|\]|,)" => (TokenType::Separator {
            token: Token::new(text.to_string(), 0, 0, 0, 0)
        }, text),

        r"(let|fn|type|struct|pub|impl|for|self|Self|mod|use|enum)" => (TokenType::Keyword {
            token: Token::new(text.to_string(), 0, 0, 0, 0)
        }, text),

        r"[^ \t\r\n:+-/*,]+" => (TokenType::Identifier {
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
            loop {
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
                        let token = tok.move_to(
                            self.line.clone(),
                            self.character - text.len(),
                            span.lo.clone(),
                            span.hi.clone(),
                        );
                        return Some((token, span));
                    }
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
