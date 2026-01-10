#[derive(Debug, PartialEq)]
pub enum Token {
    Keywords(Keyword),
    Ident(String),
    Int(String),
    Plus,
    Minus,
    LParam,
    RParam,
    Equal,
    Gt,
    Lt,
    Semicolon,
    Comma,
    Colon,
    Assign,
    Invalid(char),
    EOF
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Program,
    Begin,
    End,
    Var,
    Integer,
    If,
    Then,
    Else,
    Do,
    While
}

impl Token {
    pub fn get_id(&self) -> i32 {
        match self {
            Token::Keywords(keyword) => match keyword {
                Keyword::Program => 1,
                Keyword::Begin => 2,
                Keyword::End => 3,
                Keyword::Var => 4,
                Keyword::Integer => 5,
                Keyword::If => 6,
                Keyword::Then => 7,
                Keyword::Else => 8,
                Keyword::Do => 9,
                Keyword::While => 10
            }
            Token::Ident(_) => 11,
            Token::Int(_) => 12,
            Token::Plus => 13,
            Token::Minus => 14,
            Token::LParam => 15,
            Token::RParam => 16,
            Token::Equal => 17,
            Token::Gt => 18,
            Token::Lt => 19,
            Token::Semicolon => 20,
            Token::Comma => 21,
            Token::Colon => 22,
            Token::Assign => 23,
            Token::EOF => 0,
            Token::Invalid(_) => -1
        }
    }
    pub fn get_content(&self) -> &str {
        match self {
            Token::Ident(s) => s.as_str(),
            Token::Int(s) => s.as_str(),
            _ => "-"
        }
    }
}

impl Keyword {
    pub fn is_keyword(s: &str) -> bool {
        const KEYWORDS: &[&str] = &[
            "program",
            "begin",
            "end",
            "var",
            "integer",
            "if",
            "then",
            "else",
            "do",
            "while"
        ];
    
        KEYWORDS.contains(&s)
    }
}

#[cfg(test)]
mod tests {
}
