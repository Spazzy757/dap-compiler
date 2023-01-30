extern crate thiserror;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("IO Error")]
    FileIO(#[from] io::Error),

    #[error("was expecting {expected:?}, found {found:?}")]
    MisingExpectedSymbol { expected: TokenType, found: Token },

    #[error("cant find opening symbol {open:?} for {symbol:?}")]
    MisbalancedSymbol { symbol: char, open: char },

    #[error("")]
    UnknownSymbol { symbol: String },
}

pub type Token = TokenType;

pub struct Punctuation {
    pub raw: char,
    pub kind: PunctuationKind,
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
    Open(BalancingDepthType),

    /** ')' and ']' */
    Close(BalancingDepthType),

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
        Lexer {
            cur_col: 1,
            cur_line: 1,

            codepoint_offset: 0,

            chars: chars.chars().peekable(),
            balancing_state: std::collections::HashMap::new(),
        }
    }

    fn map_balance(c: &char) -> char {
        match c {
            '}' => '{',
            '{' => '}',
            ')' => '(',
            '(' => ')',
            '[' => ']',
            ']' => '[',
            _ => panic!("mapping a unknown balancing character"),
        }
    }

    fn push_balance(&mut self, c: &char) -> BalancingDepthType {
        if let Some(v) = self.balancing_state.get_mut(&c) {
            *v += 1;
            *v - 1
        } else {
            self.balancing_state.insert(*c, 1);
            0
        }
    }

    fn pop_balance(&mut self, c: &char) -> Result<BalancingDepthType, LexerError> {
        if let Some(v) = self.balancing_state.get_mut(&Lexer::map_balance(&c)) {
            if *v >= 1 {
                *v -= 1;
                Ok(*v)
            } else {
                Err(LexerError::MisbalancedSymbol {
                    symbol: *c,
                    open: Lexer::map_balance(&c),
                })
            }
        } else {
            Err(LexerError::MisbalancedSymbol {
                symbol: *c,
                open: Lexer::map_balance(&c),
            })
        }
    }

    fm parse_number(&mut self, start: char) -> Result<Token. LexerError> {
        let mut seen_dot = false; 
        let mut seen_exp = false;
    }

    fn transform_to_type(&mut self, c: char) -> Result<TokenType, LexerError> {
        match c {
            '(' | '[' | '{' => Ok(TokenType::Punctuation {
                raw: c,
                kind: PunctuationKind::Open(self.push_balance(&c)),
            }),
            ')' | ']' | '}' => Ok(TokenType::Punctuation {
                raw: c,
                kind: PunctuationKind::Close(self.pop_balance(&c)?),
            }),
            '0' ..= '9' => self.parse_number(c)
            _ => Err(LexerError::UnknownSymbol {
                symbol: c.to_string(),
            }),
        }
    }

    pub fn consume_char(&mut self) -> Option<char> {
        match self.chars.next() {
            Some(c) => {
                self.cur_col += 1;
                if c == '\n' {
                    self.cur_line += 1;
                    self.cur_col = 1;
                }
                self.codepoint_offset += 1;
                Some(c)
            }
            _ => None,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.chars.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.consume_char();
        }
    }

    pub fn next_token(&mut self) -> Result<TokenType, LexerError> {
        self.skip_whitespace();

        if let Some(c) = self.consume_char() {
            self.transform_to_type(c)
        } else {
            Ok(TokenType::EOF)
        }
    }
}
