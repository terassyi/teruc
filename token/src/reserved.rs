pub const WHITE_SPACE: char = ' ';

pub const PLUS: char = '+';
pub const MINUS: char = '-';
pub const ASTERISK: char = '*';
pub const SLASH: char = '/';

pub const EQUAL: char = '=';
pub const EXCLAMATION: char = '!';

pub const OPEN_PAREN: char = '(';
pub const CLOSE_PAREN: char = ')';

pub const LESS_THAN: char = '<';
pub const GREATER_THAN: char = '>';

pub const SEMICOLON: char = ';';

pub const RETURN: &str = "return";

pub const RESERVED_CHARS: [char; 12] = [
    WHITE_SPACE,
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    EQUAL,
    EXCLAMATION,
    OPEN_PAREN,
    CLOSE_PAREN,
    LESS_THAN,
    GREATER_THAN,
    SEMICOLON,
];

pub const RESERVED_STR: [&str; 1] = [RETURN];
