use std::collections::HashSet;
use std::iter::FromIterator;

use crate::render_gl::model_parsers::interface::{FormatInterpreter, ModelData};

enum WavefrontObjToken {
    STRING { v: String },
    DECIMAL { v: f64 },
    INTEGER { v: i64 },
    NEW_LINE,
}

lazy_static! {
    static ref WHITESPACE_CHARS: HashSet<char> = {
        let chars = [' ', '\t'];
        HashSet::from_iter(chars.iter().cloned())
    };
}

lazy_static! {
    static ref STRING_CHARS: HashSet<char> = {
        let chars = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
            'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
            'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '-', '_',
        ];
        HashSet::from_iter(chars.iter().cloned())
    };
}

lazy_static! {
    static ref NUMERIC_CHARS: HashSet<char> = {
        let chars = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        HashSet::from_iter(chars.iter().cloned())
    };
}

pub struct WavefrontObjInterpreter {}

impl FormatInterpreter<WavefrontObjToken> for WavefrontObjInterpreter {
    fn lex(&self, data: &str) -> Vec<WavefrontObjToken> {
        let mut tokens = vec![];
        let mut data_window = data;
        // Order here is important since some cases are subsets of others
        // Integers can be read as decimals, which can all be read as strings
        let lexers: Vec<&dyn Fn(&str) -> (Option<WavefrontObjToken>, &str)> =
            vec![&lex_integer, &lex_decimal, &lex_string, &lex_new_line];

        while data_window.len() > 0 {
            for lexer in &lexers {
                let (opt_token, new_window) = (lexer)(data_window);
                match opt_token {
                    Some(token) => {
                        tokens.push(token);
                        data_window = new_window;
                        continue;
                    }
                    _ => {}
                }
            }

            // lex_whitespace special case
            data_window = lex_whitespace(data_window);
        }

        tokens
    }

    fn parse(&self, tokens: Vec<WavefrontObjToken>) -> ModelData {
        ModelData::new_empty()
    }
}

fn lex_string(data: &str) -> (Option<WavefrontObjToken>, &str) {
    let mut char_iter = data.chars();
    let mut loc = 0;
    let mut curr_char = char_iter.next();

    loop {
        match curr_char {
            Some(c) => {
                if WHITESPACE_CHARS.contains(&c) {
                    break;
                }

                if !STRING_CHARS.contains(&c) {
                    loc = 0;
                    break;
                }
            },
            None => {
                break;
            },
        };

        curr_char = char_iter.next();
        loc += 1;
    }

    (
        if loc == 0 {
            None
        } else {
            Some(WavefrontObjToken::STRING{v: data[..loc].to_string()})
        },
        &data[loc..]
    )
}

fn lex_decimal(data: &str) -> (Option<WavefrontObjToken>, &str) {
    let mut char_iter = data.chars();
    let mut loc = 0;
    let mut curr_char = char_iter.next();
    let mut sign: f64 = 1.0;
    let mut dot_seen = false;

    match curr_char {
        Some(c) => {
            if c == '-' {
                sign = -1.0;
                loc += 1;
                curr_char = char_iter.next();
            }
        },
        None => {},
    }

    loop {
        match curr_char {
            Some(c) => {
                if WHITESPACE_CHARS.contains(&c) {
                    break;
                }

                if c == '.' {
                    if !dot_seen {
                        dot_seen = true;
                    } else {
                        loc = 0;
                        break;
                    }
                } else if !NUMERIC_CHARS.contains(&c) {
                    loc = 0;
                    break;
                }
            },
            None => {
                break;
            },
        }

        curr_char = char_iter.next();
        loc += 1;
    }

    (
        if loc == 0 {
            None
        } else {
            Some(
                WavefrontObjToken::DECIMAL{
                    v: sign * data[..loc].parse::<f64>().unwrap()
                }
            )
        },
        &data[loc..]
    )
}

fn lex_integer(data: &str) -> (Option<WavefrontObjToken>, &str) {
    let mut char_iter = data.chars();
    let mut loc = 0;
    let mut curr_char = char_iter.next();
    let mut sign: i64 = 1;

    match curr_char {
        Some(c) => {
            if c == '-' {
                sign = -1;
                loc += 1;
                curr_char = char_iter.next();
            }
        },
        None => {},
    }

    loop {
        match curr_char {
            Some(c) => {
                if WHITESPACE_CHARS.contains(&c) {
                    break;
                }

                if !NUMERIC_CHARS.contains(&c) {
                    loc = 0;
                    break;
                }
            },
            None => {
                break;
            },
        }

        curr_char = char_iter.next();
        loc += 1;
    }

    (
        if loc == 0 {
            None
        } else {
            Some(
                WavefrontObjToken::INTEGER{
                    v: sign * data[..loc].parse::<i64>().unwrap()
                }
            )
        },
        &data[loc..]
    )
}

fn lex_new_line(data: &str) -> (Option<WavefrontObjToken>, &str) {
    let mut char_iter = data.chars();
    let curr_char = char_iter.next();

    match curr_char {
        Some(c) => {
            if c == '\n' {
                return (Some(WavefrontObjToken::NEW_LINE), &data[1..]);
            }
        },
        None => { },
    }

    (None, data)
}

fn lex_whitespace(data: &str) -> (&str) {
    let mut char_iter = data.chars();
    let mut loc = 0;
    let mut curr_char = char_iter.next();

    loop {
        match curr_char {
            Some(c) => {
                if !WHITESPACE_CHARS.contains(&c) {
                    break;
                }
            },
            None => {
                break;
            }
        }
        
        curr_char = char_iter.next();
        loc += 1;
    }

    &data[loc..]
}
