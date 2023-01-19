extern crate thiserror;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("IO Error")]
    FileIO(#[from] io::Error),

    #[error("was expecting {expected:?}, found {found:?}")]
    MissingExpectedSymbol { expected: TokenType, found: Token },
}

pub type Token = TokenType;

pub struct Punctuation {
    raw: char,
    kind: PunctuationKind,
}

#[derive(Debug)]
pub enum TokenType {
    /** End of Token Stream */
    EOF,
    /** Punctuation like , { [] }*/
    Punctuation {
        raw: char,
        kind: PunctuationKind,
    },

    /** Operators are 'actions' that you take on an entity i.e  '-' '=>' */
    Operator(String),

    /** A sequence of characters  */
    Identifier(String),

    /** A single character 'a'  */
    Char(char),

    /**   */
    Numeric(String),

    Unknown(char),
}

#[derive(Debug)]
pub enum PunctuationKind {
    /** '(' and '[' */
    Open(usize),

    /** ')' and ']' */
    Close(usize),

    /** ',' and ';' */
    Separator,
}

type BalancingDepthType = i32;

pub struct Lexer<'a> {
    // Human Readable positions in file
    pub cur_line: usize,
    pub cur_col: usize,

    // raw format / offset within the file (codepoints)
    pub codepoint_offset: usize,

    chars: std::iter::Peekable<std::str::Chars<'a>>,

    balancing_state: std::collections::HashMap<char, i32>,
}

impl<'a> Lexer<'a> {
    pub fn new(chars: &'a str) -> Lexer<'a> {
        cur_col: 1,
        cur_line: 1,

        codepoint_offset: 0,

        chars: chars.Chars().peekable(),

        balancing_state: std::collections::HashMap::new(),
    }

    fn push_open(&mut self,c: char) -> i32 {
        self.balancing_state.contains_key(&c) {
        } else {
            self.balancing_state.insert(*c, 0);
        }
    }

    fn transform_to_type(c: char) -> Option<PunctuationKind> {
        match c {
            '(' => Some(TokenType::Punctuation {raw: c, kind: PunctuationKind::Open(0)}),
            ')' => Some(TokenType::Punctuation {raw: c, kind: PunctuationKind::Close(0)}),
        }
    }
}
