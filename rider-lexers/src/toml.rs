pub mod lexer {
    use crate::{Token, TokenType};
    use std::ops::{Deref, DerefMut};

    use crate::*;

    #[derive(Debug)]
    pub struct Buffer(String);

    impl Buffer {
        pub fn new() -> Self {
            Self(String::new())
        }

        pub fn is_string(&self) -> bool {
            self.0.starts_with('\'') || self.0.starts_with('"')
        }

        pub fn is_escaped(&self) -> bool {
            self.0.ends_with('\\')
        }

        pub fn is_string_beginning(&self, c: char) -> bool {
            self.is_string() && self.0.starts_with(c)
        }

        pub fn is_white(&self) -> bool {
            self.0.as_str() == " "
                || self.0.as_str() == "\t"
                || self.0.as_str() == "\n"
                || self.0.as_str() == "\r"
        }
    }

    impl Deref for Buffer {
        type Target = String;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl DerefMut for Buffer {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    pub struct Lexer {
        content: String,
        buffer: Buffer,
        line: usize,
        character: usize,
        start: usize,
    }

    impl Lexer {
        pub fn new(content: String) -> Self {
            Self {
                content,
                line: 0,
                character: 0,
                start: 0,
                buffer: Buffer::new(),
            }
        }

        pub fn tokenize(&mut self) -> Vec<TokenType> {
            let mut tokens = vec![];
            let content = self.content.clone();
            for c in content.chars() {
                match c {
                    ' ' | '\n' if self.non_special() => {
                        self.push_non_empty(&mut tokens);
                        self.append_and_push(c, &mut tokens, |b| lexer_whitespace!(b))
                    }
                    '[' | ']' | '{' | '}' if self.non_special() => {
                        self.push_non_empty(&mut tokens);
                        self.append_and_push(c, &mut tokens, |b| lexer_separator!(b))
                    }
                    '=' if self.non_special() => {
                        self.push_non_empty(&mut tokens);
                        self.append_and_push(c, &mut tokens, |b| lexer_operator!(b))
                    }
                    '"' | '\'' if self.is_string_beginning(c) => {
                        self.append_and_push(c, &mut tokens, |b| lexer_string!(b))
                    }
                    _ => {
                        self.buffer.push(c);
                    }
                }
            }
            if !self.is_empty() {
                tokens.push(lexer_identifier!(self));
                self.clear();
            }
            tokens
        }

        fn append_and_push<F>(&mut self, c: char, tokens: &mut Vec<TokenType>, builder: F)
        where
            F: Fn(&Lexer) -> TokenType,
        {
            self.buffer.push(c);
            tokens.push(builder(&self));
            self.clear();
        }

        fn push_non_empty(&mut self, tokens: &mut Vec<TokenType>) {
            if self.is_empty() {
                return;
            }
            tokens.push(self.match_token());
        }

        fn non_special(&self) -> bool {
            !self.buffer.is_string()
        }

        fn match_token(&mut self) -> TokenType {
            let token = if self.buffer.is_string() {
                lexer_string!(self)
            } else if self.buffer.is_white() {
                lexer_whitespace!(self)
            } else {
                lexer_identifier!(self)
            };
            self.clear();
            token
        }

        fn clear(&mut self) {
            if self.buffer.contains('\n') {
                self.line += self.buffer.lines().count();
                self.character +=
                    self.buffer.len() - self.buffer.rfind('\n').unwrap_or(self.buffer.len());
                self.start += self.buffer.len();
            } else {
                self.character += self.buffer.len();
                self.start += self.buffer.len();
            }
            self.buffer.clear();
        }
    }

    impl Deref for Lexer {
        type Target = Buffer;

        fn deref(&self) -> &Self::Target {
            &self.buffer
        }
    }

    impl TokenBuilder for Lexer {
        fn text(&self) -> String {
            self.buffer.to_string()
        }

        fn line(&self) -> usize {
            self.line
        }

        fn character(&self) -> usize {
            self.character
        }

        fn start(&self) -> usize {
            self.start
        }

        fn end(&self, text: &String) -> usize {
            self.start + text.len()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        struct BuilderMock {
            line: usize,
            character: usize,
            start: usize,
            text: String,
        }

        impl TokenBuilder for BuilderMock {
            fn text(&self) -> String {
                self.text.clone()
            }

            fn line(&self) -> usize {
                self.line
            }

            fn character(&self) -> usize {
                self.character
            }

            fn start(&self) -> usize {
                self.start
            }

            fn end(&self, current_text: &String) -> usize {
                self.start + current_text.len()
            }
        }

        macro_rules! builder {
            ($text: expr, $line: expr, $character: expr, $start: expr) => {
                BuilderMock {
                    line: $line,
                    character: $character,
                    start: $start,
                    text: $text.to_owned(),
                }
            };
        }

        #[test]
        fn parse_empty() {
            let code = "".to_owned();
            let mut lexer = Lexer::new(code);
            let result = lexer.tokenize();
            let expected = vec![];
            assert_eq!(result, expected)
        }

        #[test]
        fn parse_section() {
            let code = "[package]".to_owned();
            let mut lexer = Lexer::new(code);
            let result = lexer.tokenize();
            let expected = vec![
                lexer_separator!(builder!("[", 0, 0, 0)),
                lexer_identifier!(builder!("package", 0, 1, 1)),
                lexer_separator!(builder!("]", 0, 8, 8)),
            ];
            assert_eq!(result, expected)
        }

        #[test]
        fn parse_package() {
            let code = "redis = \"*\"".to_owned();
            let mut lexer = Lexer::new(code);
            let result = lexer.tokenize();
            let expected = vec![
                lexer_identifier!(builder!("redis", 0, 0, 0)),
                lexer_whitespace!(builder!(" ", 0, 5, 5)),
                lexer_operator!(builder!("=", 0, 6, 6)),
                lexer_whitespace!(builder!(" ", 0, 7, 7)),
                lexer_string!(builder!("\"*\"", 0, 8, 8)),
            ];
            assert_eq!(result, expected)
        }

        #[test]
        fn parse_complex_package() {
            let code = "redis = { version = \"*\" }".to_owned();
            let mut lexer = Lexer::new(code);
            let result = lexer.tokenize();
            let expected = vec![
                lexer_identifier!(builder!("redis", 0, 0, 0)),
                lexer_whitespace!(builder!(" ", 0, 5, 5)),
                lexer_operator!(builder!("=", 0, 6, 6)),
                lexer_whitespace!(builder!(" ", 0, 7, 7)),
                lexer_separator!(builder!("{", 0, 8, 8)),
                lexer_whitespace!(builder!(" ", 0, 9, 9)),
                lexer_identifier!(builder!("version", 0, 10, 10)),
                lexer_whitespace!(builder!(" ", 0, 17, 17)),
                lexer_operator!(builder!("=", 0, 18, 18)),
                lexer_whitespace!(builder!(" ", 0, 19, 19)),
                lexer_string!(builder!("\"*\"", 0, 20, 20)),
                lexer_whitespace!(builder!(" ", 0, 23, 23)),
                lexer_separator!(builder!("}", 0, 24, 24)),
            ];
            assert_eq!(result, expected)
        }
    }
}

mod compiler {
    //    pub struct Compiler {}
}
