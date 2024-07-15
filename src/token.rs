use std::fmt::{Display, Formatter};

#[rustfmt::skip]
#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    Bang, Equal, Greater, Less,

    // one or two character tokens.
    BangEqual, EqualEqual, GreaterEqual, LessEqual,

    // literals.
    Identifier, String(String), Number(f64),

    // keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    Eof
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
}

impl Display for Token {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.token_type {
            TokenType::LeftParen => write!(f, "LEFT_PAREN {} null", self.lexeme), TokenType::RightParen => write!(f, "RIGHT_PAREN {} null", self.lexeme),
            TokenType::LeftBrace => write!(f, "LEFT_BRACE {} null", self.lexeme), TokenType::RightBrace => write!(f, "RIGHT_BRACE {} null", self.lexeme),
            TokenType::Comma => write!(f, "COMMA {} null", self.lexeme), TokenType::Dot => write!(f, "DOT {} null", self.lexeme),
            TokenType::Minus => write!(f, "MINUS {} null", self.lexeme), TokenType::Plus => write!(f, "PLUS {} null", self.lexeme),
            TokenType::Semicolon => write!(f, "SEMICOLON {} null", self.lexeme), TokenType::Star => write!(f, "STAR {} null", self.lexeme),
            TokenType::Equal => write!(f, "EQUAL {} null", self.lexeme), TokenType::EqualEqual => write!(f, "EQUAL_EQUAL {} null", self.lexeme),
            TokenType::Bang => write!(f, "BANG {} null", self.lexeme), TokenType::BangEqual => write!(f, "BANG_EQUAL {} null", self.lexeme),
            TokenType::Less => write!(f, "LESS {} null", self.lexeme), TokenType::LessEqual => write!(f, "LESS_EQUAL {} null", self.lexeme),
            TokenType::Greater => write!(f, "GREATER {} null", self.lexeme), TokenType::GreaterEqual => write!(f, "GREATER_EQUAL {} null", self.lexeme),
            TokenType::Slash => write!(f, "SLASH {} null", self.lexeme), TokenType::String(literal) => write!(f, "STRING \"{}\" {literal}", self.lexeme),
            TokenType::Number(literal) => {
                if literal.fract() == 0.0 {
                    write!(f, "NUMBER {} {:.1}", self.lexeme, literal)
                } else {
                    write!(f, "NUMBER {} {}",  self.lexeme , literal)
                }

            },
            TokenType::Identifier => write!(f, "IDENTIFIER {} null", self.lexeme), TokenType::And => write!(f, "AND {} null", self.lexeme),
            TokenType::Class => write!(f, "CLASS {} null", self.lexeme), TokenType::Else => write!(f, "ELSE {} null", self.lexeme),
            TokenType::False => write!(f, "FALSE {} null", self.lexeme), TokenType::For => write!(f, "FOR {} null", self.lexeme),
            TokenType::Fun => write!(f, "FUN {} null", self.lexeme), TokenType::If => write!(f, "IF {} null", self.lexeme),
            TokenType::Nil => write!(f, "NIL {} null", self.lexeme), TokenType::Or => write!(f, "OR {} null", self.lexeme),
            TokenType::Print => write!(f, "PRINT {} null", self.lexeme), TokenType::Return => write!(f, "RETURN {} null", self.lexeme),
            TokenType::Super => write!(f, "SUPER {} null", self.lexeme), TokenType::This => write!(f, "THIS {} null", self.lexeme),
            TokenType::True => write!(f, "TRUE {} null", self.lexeme), TokenType::Var => write!(f, "VAR {} null", self.lexeme),
            TokenType::While => write!(f, "WHILE {} null", self.lexeme),
            TokenType::Eof => write!(f, "EOF  null"),
        }
    }
}