use std::{io::{BufReader, BufRead, Read}, fs::File, path::Path, result, collections::HashMap};

use flagset::{flags, FlagSet};

#[derive(Debug)]
pub enum ParseError {
    ConversionError(usize),
    HeaderError(usize),
    UnexpectedTokenError(usize),
    FrameError(usize),
}

flags! {
    enum Movement: u8 {
        Up = 0b001,
        Down = 0b010,
        Left = 0b0100,
        Right = 0b1000,
        None = 0b000,
    }
}

/// Parses a reader and returns a string with the transpiled contents.
/// 
/// # Errors
/// 
/// Will return a `ParseError` if parsing fails.
pub fn parse_reader<R: Read>(reader: BufReader<R>) -> Result<String, ParseError> {
    let mut at_body = false;
    let mut username: Option<String> = None;
    let mut level: Option<u8> = None;

    let mut max_frame = 0;

    let mut movement_map: Vec<(u32, FlagSet<Movement>)> = Vec::new();

    for (line_num, line) in reader.lines().map(result::Result::unwrap).enumerate() {
        let tokens: Vec<_> = line.split_whitespace().collect();

        if tokens.is_empty() {
            continue;
        }

        if at_body { 
            if tokens.len() < 2 {
                return Err(ParseError::UnexpectedTokenError(line_num));
            }

            let frame = tokens[0].parse::<u32>().map_err(|_| ParseError::ConversionError(line_num))?;
            if frame > max_frame {
                max_frame = frame;
            } else {
                return Err(ParseError::FrameError(line_num));
            }

            let mut movement_flag = FlagSet::<Movement>::new(0b0000).unwrap();

            for token_ref in tokens.split_first().unwrap().1 { // Loop over movement tokens, or-ing them each time
                let rhs = match *token_ref { // BitOr is Into<FLagset<_>>
                    "up" => Movement::Up,
                    "down" => Movement::Down,
                    "left" => Movement::Left,
                    "right" => Movement::Right,
                    "none" => Movement::None,
                    _ => return Err(ParseError::UnexpectedTokenError(line_num))
                };
                movement_flag |= rhs;
            }

            movement_map.push((frame, movement_flag));

        } else { // Fields aren't present!
            match tokens[0] {
                "level" => {
                    match tokens.get(1) {
                        Some(l) => level = Some(l.parse::<u8>().map_err(|_| ParseError::ConversionError(line_num))?),
                        None => return Err(ParseError::HeaderError(line_num)),
                    }
                },
                "username" => {
                    match tokens.get(1) {
                        Some(l) => username = Some((*l).to_string()),
                        None => return Err(ParseError::HeaderError(line_num)),
                    }
                },
                &_ => return Err(ParseError::UnexpectedTokenError(line_num))
            }

            if username.is_some() && level.is_some()  {
                at_body = true;
            }
        }
    }

    let mut body = format!("{}ǇǇ{}Ǉ0Ǉ", username.unwrap(), level.unwrap() + 1);

    for (f, m) in movement_map {
        body = format!("{}{}Ǉ{}Ǉ", body, f, m.bits());
    }

    body.pop(); // remove last sep

    body.insert_str(0, &(body.chars().count() + 12_345_678).to_string());

    Ok(body)
}

pub fn get_file_reader<P: AsRef<Path>>(file_path: P) -> std::io::Result<BufReader<File>> {
    let file = File::open(file_path)?;
    Ok(BufReader::new(file))
}
