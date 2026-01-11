#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keywords(Keyword),
    Ident(String),
    Int(String),
    Plus,
    Minus,
    Times,
    LParam,
    RParam,
    LBrace,
    RBrace,
    Equal,
    Gt,
    Lt,
    Ge,
    Le,
    Ne,
    Semicolon,
    Comma,
    Assign,
    Invalid(char),
    EOF
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Int,
    If,
    Else,
    While,
    Return
}

impl Token {
    pub fn id(&self) -> i32 {
        match self {
            Token::Keywords(keyword) => match keyword {
                Keyword::Int => 1,
                Keyword::If => 2,
                Keyword::Else => 3,
                Keyword::While => 4,
                Keyword::Return => 5,
            }
            Token::Ident(_) => 6,
            Token::Int(_) => 7,
            Token::Plus => 8,
            Token::Minus => 9,
            Token::Times => 10,
            Token::LParam => 11,
            Token::RParam => 12,
            Token::LBrace => 13,
            Token::RBrace => 14,
             Token::Equal => 15,
             Token::Gt => 16,
             Token::Lt => 17,
             Token::Ge => 21,
             Token::Le => 22,
             Token::Ne => 23,
             Token::Semicolon => 18,
             Token::Comma => 19,
             Token::Assign => 20,
            Token::EOF => 0,
            Token::Invalid(_) => -1
        }
    }
    pub fn content(&self) -> &str {
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
            "int",
            "if",
            "else",
            "while",
            "return"
        ];
    
        KEYWORDS.contains(&s)
    }
}

#[cfg(test)]
mod tests {
}
