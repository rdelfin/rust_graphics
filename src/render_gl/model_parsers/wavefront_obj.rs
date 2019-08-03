use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::iter::{FromIterator, Iterator};

use nalgebra_glm as glm;

use crate::render_gl::model_parsers::interface::{FormatInterpreter, ModelData};

#[derive(Clone)]
enum WavefrontObjToken {
    Vertex,
    VertexNormal,
    VertexTexture,
    Group,
    Face,
    Decimal(f64),
    Integer(i64),
    StringVal(String),
    NewLine,
}

impl Display for WavefrontObjToken {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        match *self {
            WavefrontObjToken::Vertex => f.write_str("Vertex"),
            WavefrontObjToken::VertexNormal => f.write_str("VertexNormal"),
            WavefrontObjToken::VertexTexture => f.write_str("VertexTexture"),
            WavefrontObjToken::Group => f.write_str("Group"),
            WavefrontObjToken::Face => f.write_str("Face"),
            WavefrontObjToken::Decimal(ref v) => {
                return f.write_str(&format!("Decimal({})", v)[..]);
            },
            WavefrontObjToken::Integer(ref v) => {
                return f.write_str(&format!("Integer({})", v)[..]);
            },
            WavefrontObjToken::StringVal(ref v) => {
                return f.write_str(&format!("StringVal({})", v)[..]);
            }
            WavefrontObjToken::NewLine => f.write_str("NewLine"),
        }
    }
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

    fn parse(&self, tokens: Vec<WavefrontObjToken>) -> HashMap<String, ModelData> {
        let mut points: Vec<glm::Vec3> = vec![];
        let mut normals: Vec<glm::Vec3> = vec![];
        let mut models = HashMap::new();
        let mut curr_model: Option<String> = None;

        let mut iter = tokens.iter();
        let mut opt_token: Option<&WavefrontObjToken> = iter.next();

        let is_decimal = |t: WavefrontObjToken| match t {
            WavefrontObjToken::Decimal(_) => true,
            _ => false,
        };
        let is_integer = |t: WavefrontObjToken| match t {
            WavefrontObjToken::Integer(_) => true,
            _ => false,
        };
        let is_string = |t: WavefrontObjToken| match t {
            WavefrontObjToken::StringVal(_) => true,
            _ => false,
        };

        while opt_token.is_some() {
            let token = opt_token
                .expect("Token said it was some but returned none");

            match token {
                WavefrontObjToken::Vertex => {
                    let vertex_tokens = ingest_until_eol(&mut iter);
                    if vertex_tokens.len() != 3 || !vertex_tokens.into_iter().all(is_decimal) {
                        return models;
                    }
                    let WavefrontObjToken::Decimal(x) = vertex_tokens[0];
                    let WavefrontObjToken::Decimal(y) = vertex_tokens[1];
                    let WavefrontObjToken::Decimal(z) = vertex_tokens[2];

                    points.push(glm::vec3(x as f32, y as f32, z as f32))
                },
                WavefrontObjToken::VertexNormal => {
                    let vertex_tokens = ingest_until_eol(&mut iter);
                    if vertex_tokens.len() != 3 || !vertex_tokens.into_iter().all(is_decimal) {
                        return models;
                    }
                    let WavefrontObjToken::Decimal(x) = vertex_tokens[0];
                    let WavefrontObjToken::Decimal(y) = vertex_tokens[1];
                    let WavefrontObjToken::Decimal(z) = vertex_tokens[2];

                    normals.push(glm::vec3(x as f32, y as f32, z as f32))
                },
                WavefrontObjToken::VertexTexture => {
                    // We're not doing anything with this data, so just validate
                    let vertex_tokens = ingest_until_eol(&mut iter);
                    if vertex_tokens.len() != 3 || !vertex_tokens.into_iter().all(is_decimal) {
                        return models;
                    }
                },
                WavefrontObjToken::Group => {
                    let vertex_tokens = ingest_until_eol(&mut iter);
                    if vertex_tokens.len() != 1 || !is_string(vertex_tokens[0]) {
                        return models;
                    }
                    let WavefrontObjToken::StringVal(name) = vertex_tokens[0];
                    models.insert(name.clone(), ModelData::new_empty());
                    models[&name].name = name.clone();
                    curr_model = Some(name.clone());
                },
                WavefrontObjToken::Face => {
                    let vertex_tokens = ingest_until_eol(&mut iter);
                    if !vertex_tokens.into_iter().all(is_integer) {
                        return models;
                    }

                    if vertex_tokens.len() == 4 {
                        let WavefrontObjToken::Integer(x) = vertex_tokens[0];
                        let WavefrontObjToken::Integer(y) = vertex_tokens[1];
                        let WavefrontObjToken::Integer(z) = vertex_tokens[3];
                        let WavefrontObjToken::Integer(w) = vertex_tokens[3];

                        let model_name = curr_model.unwrap();

                        models[&model_name].points.push(points[(x+1) as usize]);
                        models[&model_name].points.push(points[(y+1) as usize]);
                        models[&model_name].points.push(points[(z+1) as usize]);
                        models[&model_name].points.push(points[(x+1) as usize]);
                        models[&model_name].points.push(points[(z+1) as usize]);
                        models[&model_name].points.push(points[(w+1) as usize]);

                        models[&model_name].normals.push(normals[(x+1) as usize]);
                        models[&model_name].normals.push(normals[(y+1) as usize]);
                        models[&model_name].normals.push(normals[(z+1) as usize]);
                        models[&model_name].normals.push(normals[(x+1) as usize]);
                        models[&model_name].normals.push(normals[(z+1) as usize]);
                        models[&model_name].normals.push(normals[(w+1) as usize]);
                    } else if vertex_tokens.len() == 3 {
                        let WavefrontObjToken::Integer(x) = vertex_tokens[0];
                        let WavefrontObjToken::Integer(y) = vertex_tokens[1];
                        let WavefrontObjToken::Integer(z) = vertex_tokens[3];

                        let model_name = curr_model.unwrap();

                        models[&model_name].points.push(points[(x+1) as usize]);
                        models[&model_name].points.push(points[(y+1) as usize]);
                        models[&model_name].points.push(points[(z+1) as usize]);

                        models[&model_name].normals.push(normals[(x+1) as usize]);
                        models[&model_name].normals.push(normals[(y+1) as usize]);
                        models[&model_name].normals.push(normals[(z+1) as usize]);
                    } else {
                        return models;
                    }
                },
                _ => {
                    panic!(format!("Unrecognized start of line: {}", token))
                }
            }

            opt_token = iter.next();
        }

        models
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

    if loc == 0 {
        return (None, data);
    }

    let str_seg = &data[..loc];
    
    return match str_seg {
        "v" =>  (Some(WavefrontObjToken::Vertex), &data[loc..]),
        "vn" => (Some(WavefrontObjToken::VertexNormal), &data[loc..]),
        "vt" => (Some(WavefrontObjToken::VertexTexture), &data[loc..]),
        "g" =>  (Some(WavefrontObjToken::Group), &data[loc..]),
        "f" =>  (Some(WavefrontObjToken::Face), &data[loc..]),
        _ => (
            Some(WavefrontObjToken::StringVal(data[..loc].to_string())),
            &data[loc..],
        ),
    }
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
                WavefrontObjToken::Decimal(
                    sign * data[..loc].parse::<f64>().unwrap()
                )
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
                WavefrontObjToken::Integer(
                    sign * data[..loc].parse::<i64>().unwrap()
                )
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
                return (Some(WavefrontObjToken::NewLine), &data[1..]);
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

fn ingest_until_eol<'a, I>(
    iter: &mut I
) -> Vec<WavefrontObjToken> where I: Iterator<Item = &'a WavefrontObjToken> {
    let mut tokens: Vec<WavefrontObjToken> = vec![];
    let opt_token = iter.next();

    while opt_token.is_some() {
        let token = opt_token
            .expect("Token said it was some but returned none");

        match token {
            WavefrontObjToken::NewLine => {
                break;
            }
            _ => {
                tokens.push(*token);
            }
        }

        opt_token = iter.next();
    }

    tokens
}
