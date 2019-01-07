use std::ops::Deref;

pub mod plain;
pub mod rust_lang;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub enum Language {
    PlainText,
    Rust,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Whitespace { token: Token },
    Keyword { token: Token },
    String { token: Token },
    Identifier { token: Token },
    Literal { token: Token },
    Comment { token: Token },
    Operator { token: Token },
    Separator { token: Token },
}

impl TokenType {
    pub fn move_to(&self, line: usize, character: usize, start: usize, end: usize) -> Self {
        match self {
            TokenType::Whitespace { token } => TokenType::Whitespace {
                token: token.move_to(line, character, start, end),
            },
            TokenType::Keyword { token } => TokenType::Keyword {
                token: token.move_to(line, character, start, end),
            },
            TokenType::String { token } => TokenType::String {
                token: token.move_to(line, character, start, end),
            },
            TokenType::Identifier { token } => TokenType::Identifier {
                token: token.move_to(line, character, start, end),
            },
            TokenType::Literal { token } => TokenType::Literal {
                token: token.move_to(line, character, start, end),
            },
            TokenType::Comment { token } => TokenType::Comment {
                token: token.move_to(line, character, start, end),
            },
            TokenType::Operator { token } => TokenType::Operator {
                token: token.move_to(line, character, start, end),
            },
            TokenType::Separator { token } => TokenType::Separator {
                token: token.move_to(line, character, start, end),
            },
        }
    }

    pub fn is_new_line(&self) -> bool {
        match self {
            TokenType::Whitespace { token } => token.text().as_str() == "\n",
            _ => false,
        }
    }

    pub fn is_space(&self) -> bool {
        match self {
            TokenType::Whitespace { token } => token.text().as_str() == " ",
            _ => false,
        }
    }
}

impl Deref for TokenType {
    type Target = Token;

    fn deref(&self) -> &<Self as Deref>::Target {
        match self {
            TokenType::Whitespace { token } => token,
            TokenType::Keyword { token } => token,
            TokenType::String { token } => token,
            TokenType::Identifier { token } => token,
            TokenType::Literal { token } => token,
            TokenType::Comment { token } => token,
            TokenType::Operator { token } => token,
            TokenType::Separator { token } => token,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    line: usize,
    character: usize,
    start: usize,
    end: usize,
    text: String,
}

impl Token {
    pub fn new(text: String, line: usize, character: usize, start: usize, end: usize) -> Self {
        Self {
            text,
            line,
            character,
            start,
            end,
        }
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn line(&self) -> usize {
        self.line.clone()
    }

    pub fn character(&self) -> usize {
        self.character.clone()
    }

    pub fn start(&self) -> usize {
        self.start.clone()
    }

    pub fn end(&self) -> usize {
        self.end.clone()
    }

    pub fn move_to(&self, line: usize, character: usize, start: usize, end: usize) -> Self {
        Self {
            text: self.text.clone(),
            line,
            character,
            start,
            end,
        }
    }
}

pub fn parse(text: String, language: &Language) -> Vec<TokenType> {
    match language {
        &Language::PlainText => plain::lexer::Lexer::new(text.as_str())
            .inspect(|tok| warn!("tok: {:?}", tok))
            .map(|t| t.0)
            .collect(),
        &Language::Rust => rust_lang::lexer::Lexer::new(text.as_str())
            .inspect(|tok| warn!("tok: {:?}", tok))
            .map(|t| t.0)
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::*;

    #[test]
    fn must_parse_plain() {
        let buffer = "foo bar";
        let language = &Language::PlainText;
        let result = parse(buffer.to_string(), language);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn must_parse_rust() {
        let buffer = "foo bar";
        let language = &Language::Rust;
        let result = parse(buffer.to_string(), language);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn must_return_valid_value_for_text() {
        let token = Token::new("a".to_string(), 1, 2, 3, 4);
        let result = token.text();
        let text: String = "a".to_string();
        let expected = &text;
        assert_eq!(result, expected);
    }

    #[test]
    fn must_return_valid_value_for_line() {
        let token = Token::new("a".to_string(), 1, 2, 3, 4);
        let result = token.line();
        let expected = 1;
        assert_eq!(result, expected);
    }

    #[test]
    fn must_return_valid_value_for_character() {
        let token = Token::new("a".to_string(), 1, 2, 3, 4);
        let result = token.character();
        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    fn must_return_valid_value_for_start() {
        let token = Token::new("a".to_string(), 1, 2, 3, 4);
        let result = token.start();
        let expected = 3;
        assert_eq!(result, expected);
    }

    #[test]
    fn must_return_valid_value_for_end() {
        let token = Token::new("a".to_string(), 1, 2, 3, 4);
        let result = token.end();
        let expected = 4;
        assert_eq!(result, expected);
    }

    #[test]
    fn must_return_valid_value_for_move_to() {
        let token = Token::new("a".to_string(), 1, 2, 3, 4);
        let result = token.move_to(5, 6, 7, 8);
        let expected = Token::new("a".to_string(), 5, 6, 7, 8);
        assert_eq!(result, expected);
    }
}
