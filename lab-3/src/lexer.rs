use crate::token::{ self, Keyword, Token };

pub struct Lexer<'a> {
    chars: std::str::Chars<'a>,
    curr: Option<char>,
    next: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let curr = chars.next();
        let next = chars.next();
        Lexer {
            chars,
            curr,
            next,
        }
    }

    pub fn to_tokens(mut self) -> Result<Vec<Token>, Vec<Token>> {
        let mut tokens = Vec::new();
        let mut invalid_tokens = Vec::new();

        while self.curr.is_some() {
            match self.get_token() {
                Ok(token) => tokens.push(token),
                Err(inv) => invalid_tokens.push(inv)
            }
        }


        if invalid_tokens.is_empty() {
            Ok(tokens)
        }
        else {
            Err(invalid_tokens)
        }
    }

    fn get_token(&mut self) -> Result<Token, Token> {
        while self.curr.is_some_and(|c| c.is_whitespace()) {
            self.advance();
        }

        match self.curr {
            Some('+') => {
                self.advance();
                Ok(Token::Plus)
            }
            Some('-') => { 
                self.advance();
                Ok(Token::Minus)
            }
            Some('*') => {
                self.advance();
                Ok(Token::Times)
            }
            Some('(') => { 
                self.advance();
                Ok(Token::LParam)
            }
            Some(')') => { 
                self.advance();
                Ok(Token::RParam)
            }
            Some('{') => {
                self.advance();
                Ok(Token::LBrace)
            }
            Some('}') => {
                self.advance();
                Ok(Token::RBrace)
            }
             Some('>') => {
                 if let Some('=') = self.next {
                     self.advance();
                     self.advance();
                     Ok(Token::Ge)
                 }
                 else { 
                     self.advance();
                     Ok(Token::Gt)
                 }
             }
             Some('<') => {
                 if let Some('=') = self.next {
                     self.advance();
                     self.advance();
                     Ok(Token::Le)
                 }
                 else { 
                     self.advance();
                     Ok(Token::Lt)
                 }
             }
            Some(';') => {
                self.advance();
                Ok(Token::Semicolon)
            }
            Some(',') => {
                self.advance();
                Ok(Token::Comma)
            }
            Some('!') => {
                if self.next.is_some_and(|c| c == '=') {
                    self.advance();
                    self.advance();
                    Ok(Token::Ne)
                }
                else { 
                    self.advance();
                    Err(Token::Invalid('!'))
                }
            }
            Some('=') => {
                if self.next.is_some_and(|c| c == '=') {
                    self.advance();
                    self.advance();
                    Ok(Token::Equal)
                }
                else { 
                    self.advance();
                    Ok(Token::Assign) 
                }
            }
            Some(ch) if ch.is_alphabetic() => {
                Ok(self.process_alphabetic())
            }
            Some(ch) if ch.is_digit(10) => {
                Ok(self.process_num())
            }
            None => Ok(Token::EOF),
            // Error handling
            Some(e) => {
                self.advance();
                Err(Token::Invalid(e))
            }
        }

    }

    fn process_num(&mut self) -> Token {
        let mut num_str = String::new();
        let mut ch = self.curr;
        while ch.is_some_and(|c| c.is_digit(10)) {
            num_str.push(ch.unwrap());
            ch = self.advance();
        }
        Token::Int(num_str)
    }

    fn process_alphabetic(&mut self) -> Token {
        let mut s = String::new();
        let mut ch = self.curr;
        while ch.is_some_and(|c| c.is_alphanumeric() || c == '_') {
            s.push(ch.unwrap());
            ch = self.advance();
        }
        
        if token::Keyword::is_keyword(&s) {
            match s.as_str() {
                "int" => Token::Keywords(Keyword::Int),
                "if" => Token::Keywords(Keyword::If),
                "else" => Token::Keywords(Keyword::Else),
                "while" => Token::Keywords(Keyword::While),
                "return" => Token::Keywords(Keyword::Return),
                _ => Token::Ident(s)
            }
        }
        else {
            Token::Ident(s)
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.curr = self.next;
        self.next = self.chars.next();
        self.curr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_token_empty_input() {
        let mut lexer = Lexer::new("");
        assert_eq!(lexer.get_token(), Ok(Token::EOF));
    }

    #[test]
    fn test_get_token_keywords() {
        let mut lexer = Lexer::new("int");
        assert_eq!(lexer.get_token(), Ok(Token::Keywords(token::Keyword::Int)));

        let mut lexer = Lexer::new("if");
        assert_eq!(lexer.get_token(), Ok(Token::Keywords(token::Keyword::If)));

        let mut lexer = Lexer::new("else");
        assert_eq!(lexer.get_token(), Ok(Token::Keywords(token::Keyword::Else)));

        let mut lexer = Lexer::new("while");
        assert_eq!(lexer.get_token(), Ok(Token::Keywords(token::Keyword::While)));

        let mut lexer = Lexer::new("return");
        assert_eq!(lexer.get_token(), Ok(Token::Keywords(token::Keyword::Return)));
    }

    #[test]
    fn test_get_token_ident() {
        let mut lexer = Lexer::new("xjtu_compiler_2025");
        assert_eq!(lexer.get_token(), Ok(Token::Ident(String::from("xjtu_compiler_2025"))));
    }

    #[test]
    fn test_get_token_integer() {
        let mut lexer = Lexer::new("114514");
        assert_eq!(lexer.get_token(), Ok(Token::Int(String::from("114514"))));

        let mut lexer = Lexer::new("0");
        assert_eq!(lexer.get_token(), Ok(Token::Int(String::from("0"))));
    }

    #[test]
    #[ignore]
    fn test_get_token_signed_ingerger() {
        let mut lexer = Lexer::new("-114514");
        assert_eq!(lexer.get_token(), Ok(Token::Int(String::from("-114514"))));
    }

    #[test]
    fn test_get_token_operator() {
        let mut lexer = Lexer::new("+");
        assert_eq!(lexer.get_token(), Ok(Token::Plus));

        let mut lexer = Lexer::new("-");
        assert_eq!(lexer.get_token(), Ok(Token::Minus));

        let mut lexer = Lexer::new("(");
        assert_eq!(lexer.get_token(), Ok(Token::LParam));

        let mut lexer = Lexer::new(")");
        assert_eq!(lexer.get_token(), Ok(Token::RParam));

        let mut lexer = Lexer::new("=");
        assert_eq!(lexer.get_token(), Ok(Token::Assign));

        let mut lexer = Lexer::new(">");
        assert_eq!(lexer.get_token(), Ok(Token::Gt));

        let mut lexer = Lexer::new("<");
        assert_eq!(lexer.get_token(), Ok(Token::Lt));

        let mut lexer = Lexer::new("==");
        assert_eq!(lexer.get_token(), Ok(Token::Equal));
    }

    #[test]
    fn test_get_token_boundary() {
        let mut lexer = Lexer::new(";");
        assert_eq!(lexer.get_token(), Ok(Token::Semicolon));

        let mut lexer = Lexer::new(",");
        assert_eq!(lexer.get_token(), Ok(Token::Comma));
    }

    #[test]
    fn test_get_token_invalid() {
        let mut lexer = Lexer::new("#");
        assert_eq!(lexer.get_token(), Err(Token::Invalid('#')));
    }
}

