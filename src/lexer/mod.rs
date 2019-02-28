pub mod token;

use token::Token;
use std::str::Chars;
use std::iter::Peekable;

pub struct Lexer<'a> {
    pub input: Peekable<Chars<'a>>,
    on_comment: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &String) -> Lexer {
        Lexer { input: input.chars().peekable(), on_comment: false }
    }

    fn skip(&mut self) -> Option<char> {
        let ch = self.input.next();
        match ch {
            Some(chr) => if chr.is_whitespace() && !self.is_line_ending(chr) { 
                self.skip() 
            }
            else if self.on_comment {
                if self.is_line_ending(chr) {
                    self.on_comment = false;
                    ch
                }
                else {
                    self.skip()
                }
            }
            else {
                ch
            },
            None => ch,
        }
    }

    fn skip_and_return(&mut self, token: Token) -> Token {
        self.input.next();
        token
    }

    fn handle_slash(&mut self) -> Token {
        if let Some('/') = self.input.peek() {
            self.on_comment = true;
            self.skip();

            return Token::Comment;
        }

        return Token::Slash;
    }

    fn handle_equal(&mut self) -> Token {
        match self.input.peek() {
            Some('=') => self.skip_and_return(Token::Equal),
            _ => Token::Assign,
        }
    }

    fn handle_dot(&mut self) -> Token {
        match self.input.peek() {
            Some('=') => self.skip_and_return(Token::ConstAssign),
            _ => Token::Dot,
        }
    }

    fn handle_cr(&mut self) -> Token {
        match self.input.peek() {
            Some('\n') => self.skip_and_return(Token::Endline),
            _ => Token::Endline,
        }
    }

    fn is_line_ending(&mut self, chr: char) -> bool {
        if chr == '\n' || chr == '\r' {
            if let Some('\n') = self.input.peek() {
                self.input.next();
            }

            return true;
        }
        return false;
    }

    pub fn build_name(&mut self, ch: Vec<char>) -> String {
        let next_char_or_none = self.input.peek();

        if next_char_or_none != None {
            let next_char = next_char_or_none.unwrap();
            if next_char.is_alphanumeric() || *next_char == '_' {
                let name = [&ch[..], &vec![*next_char][..]].concat();
                self.skip();
                return self.build_name(name);
            }
        }

        ch.into_iter().collect()
    }

    pub fn build_numerical_string(&mut self, ch: Vec<char>) -> String {
        let next_char_or_none = self.input.peek();

        if next_char_or_none != None {
            let next_char = next_char_or_none.unwrap();
            if next_char.is_numeric() {
                let number = [&ch[..], &vec![*next_char][..]].concat();
                self.skip();
                return self.build_numerical_string(number);
            }
        }

        ch.into_iter().collect()
    }

    pub fn build_number(&mut self, ch: Vec<char>) -> i64 {
        self.build_numerical_string(ch).parse::<i64>().unwrap()
    }

    pub fn build_string(&mut self, ch: Vec<char>) -> String {
        let next_char_or_none = self.input.peek();

        if next_char_or_none != None {
            let next_char = next_char_or_none.unwrap();
            if *next_char != '"' {
                let string = [&ch[..], &vec![*next_char][..]].concat();
                self.input.next();
                return self.build_string(string);
            }
            else {
                self.input.next();
            }
        }

        ch.into_iter().collect()
    }

    pub fn read_string(&mut self) -> Token {
        let first_char = self.input.next();
        if first_char == None {
            return Token::Str(String::from(""));
        }

        return Token::Str(self.build_string(vec![first_char.unwrap()]));
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let ch = self.skip();
        match ch {
            Some('?') => Some(Token::QuestionMark),
            Some('=') => Some(self.handle_equal()),
            Some(',') => Some(Token::Comma),
            Some('.') => Some(self.handle_dot()),
            Some(';') => Some(Token::Semicolon),
            Some('"') => Some(self.read_string()),
            Some('\'') => Some(Token::Quote),
            Some('\r') => Some(self.handle_cr()),
            Some('\n') => Some(Token::Endline),
            Some('/') => Some(self.handle_slash()),
            Some('(') => Some(Token::OpenParen),
            Some(')') => Some(Token::CloseParen),
            Some(chr) => {
                if chr.is_alphabetic() || chr == '_' {
                    Some(Token::Name(self.build_name(vec![chr])))
                }
                else if chr.is_numeric() {
                    Some(Token::Number(self.build_number(vec![chr])))
                }
                else {
                    Some(Token::StrangeChar(chr))
                }
            },
            None => None,
        }
    }
}