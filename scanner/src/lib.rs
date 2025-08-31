use std::fmt::Display;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::str::FromStr;

/// Enum > Trait

/// "Traits" - Interfaz

/// All the tokens we know
#[derive(Debug)]
pub(crate) enum TokenType {
    /// (
    OpenParens,
    /// )
    CloseParens,
    // +
    Plus,
    // -
    Minus,
    // *
    Star,
    // *
    Equal,
    // . // second best comment
    Dot,
    // <
    Less,
    // <=
    LessEqual,
    // >
    Great,
    // >=
    GreatEqual,
    // / // best comment
    ForwardSlash,
    // \
    BackSlash,
    // [
    OpenBracket,
    // ]
    CloseBracket,
    // '
    Quote,
    // `
    BackQuote,
    // ,
    Comma,
    // #
    Hashtag,
    /// Identifer
    Identifier,
    //
    String {
        literal: String,
    },
    //
    Number {
        literal: String,
    },

    // Palabras reservadas
    ReservedKeywords(ReservedKeywords),
}

#[derive(Debug)]
enum TokenTypeError {
    NotKnownToken,
}

impl FromStr for TokenType {
    type Err = TokenTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "(" => Ok(TokenType::OpenParens),
            ")" => Ok(TokenType::CloseParens),
            "+" => Ok(TokenType::Plus),
            "-" => Ok(TokenType::Minus),
            "*" => Ok(TokenType::Star),
            "." => Ok(TokenType::Dot),
            "<" => Ok(TokenType::Less),
            "<=" => Ok(TokenType::LessEqual),
            ">" => Ok(TokenType::Great),
            ">=" => Ok(TokenType::GreatEqual),
            "/" => Ok(TokenType::ForwardSlash),
            "\\" => Ok(TokenType::BackSlash),
            "[" => Ok(TokenType::OpenBracket),
            "]" => Ok(TokenType::CloseBracket),
            "`" => Ok(TokenType::BackQuote),
            "," => Ok(TokenType::Comma),
            "#" => Ok(TokenType::Hashtag),
            reserved if ReservedKeywords::from_str(s).is_ok() => {
                let reserved = ReservedKeywords::from_str(reserved).unwrap();
                Ok(TokenType::ReservedKeywords(reserved))
            }
            _ => Err(TokenTypeError::NotKnownToken),
        }
    }
}

#[derive(Debug)]
enum ReservedKeywords {
    Define,

    Let,
}

#[derive(Debug)]
enum ReservedKeywordError {
    NotReservedWord,
}

impl FromStr for ReservedKeywords {
    type Err = ReservedKeywordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "define" => Ok(ReservedKeywords::Define),
            "let" => Ok(ReservedKeywords::Let),
            _ => Err(ReservedKeywordError::NotReservedWord),
        }
    }
}

#[derive(Debug)]
struct Coordinate {
    line: u32,
    column: u32,
}
impl Coordinate {
    fn new(line: u32, column: u32) -> Coordinate {
        Coordinate { line, column }
    }
}

/// Macro para usar en un match y guardarte el character en una variable llamada "chary"
#[macro_export]
macro_rules! lol {
    (  $x:expr ) => {
            chary @ $x
    };
}

/// Vec of coordinates because a token can take up multiple coordinates
#[derive(Debug)]
struct Location {
    input: Input,
    coordinates: Vec<Coordinate>,
}

impl Location {
    fn new(input: Input, coordinates: Vec<Coordinate>) -> Location {
        Location { input, coordinates }
    }
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,

    /// Lexeme that originated the Token
    lexeme: String,

    location: Location,
}

impl Token {
    pub(crate) fn new(token_type: TokenType, lexeme: String, location: Location) -> Token {
        Token {
            token_type,
            lexeme,
            location,
        }
    }
}

// Lee todo el input y lo devuelve como un string. Lo aloca todo de una, asique que nadie lea Don Quijote de la Mancha con esta funcion!
fn read<R: Read>(mut input: BufReader<R>) -> String {
    let mut buffer = String::new();
    let len = input
        .read_to_string(&mut buffer)
        .expect("Failed to read line");

    buffer
}

#[derive(Clone, Debug)]
pub enum Input {
    Stdin,
    File { path: PathBuf },
}

/// Function that handles scanning
/// This function will scan all the "lexemes" present in the passed in the input (either a file or stdin).
pub fn scan(input: Input, text: String) -> Vec<Token> {
    // Le paso enumerate para tener la posicion del character

    // Esto nos devuelve una tripla del tipo:
    //
    #[allow(clippy::map_identity)]
    let columns = text
        .split_inclusive('\n')
        .flat_map(|line| line.chars().enumerate().map(|(column, _)| column))
        // En Emacs, me figura que la primera columna es la 0, dejo asi?
        .map(|column_number| column_number);

    // We add the mut to use the next method
    let mut characters = text
        .split_inclusive('\n')
        .enumerate()
        // Le sumo 1 al line number porque enumerate arranca en 0 pero la
        // primera linea de un archivo es la linea 1. Fuente: este archivo
        // Preserve newlines
        .map(|(line_number, line)| (line_number + 1, line))
        .flat_map(|(line_number, line)| line.chars().map(move |cha| (cha, line_number)))
        .zip(columns)
        // Le anado las columnas
        .map(|((character, line), column)| (character, line, column))
        .map(|(character, line, column)| (Coordinate::new(line as u32, column as u32), character))
        .peekable();

    let mut tokens: Vec<Token> = Vec::new();

    // We read until we run out of characters
    while let Some((coordinate, character)) = dbg!(characters.next()) {
        // dbg!(&character);
        match character {
            // We found a comment
            ';' => {
                // Cuando encontramos un comentario, salteamos hasta el primer newline
                while let Some((_, letter)) = characters.next()
                    && letter != '\n'
                {
                    dbg!(letter);
                }
            }
            '(' => {
                let token = Token::new(
                    TokenType::OpenParens,
                    String::from("("),
                    Location::new(input.clone(), vec![coordinate]),
                );
                tokens.push(token);
            }
            ')' => {
                let token = Token::new(
                    TokenType::CloseParens,
                    String::from(")"),
                    Location::new(input.clone(), vec![coordinate]),
                );
                tokens.push(token);
            }
            '[' => {
                let token = Token::new(
                    TokenType::OpenBracket,
                    String::from("["),
                    Location::new(input.clone(), vec![coordinate]),
                );
                tokens.push(token);
            }
            ']' => {
                let token = Token::new(
                    TokenType::CloseBracket,
                    String::from("["),
                    Location::new(input.clone(), vec![coordinate]),
                );
                tokens.push(token);
            }
            '+' => {
                let token = Token::new(
                    TokenType::Plus,
                    String::from("+"),
                    Location::new(input.clone(), vec![coordinate]),
                );
                tokens.push(token);
            }
            '-' => {
                let token = Token::new(
                    TokenType::Minus,
                    String::from("-"),
                    Location::new(input.clone(), vec![coordinate]),
                );
                tokens.push(token);
            }
            '*' => {
                let token = Token::new(
                    TokenType::Star,
                    String::from("*"),
                    Location::new(input.clone(), vec![coordinate]),
                );
                tokens.push(token);
            },
            // In the two cases below I have to check if I have an equal operator concatened
            '<' => {
                // The less is the last char or I have something distinct to equal.
                if !let Some((next_coord, next_char)) = characters.peek() || next_char != '=' {
                    let token = Token::new(
                    TokenType::Less,
                    String::from("<"),
                    Location::new(input.clone(), vec![coordinate]),
                    );
                    tokens.push(token);
                } else {
                    let token = Token::new(
                    TokenType::LessEqual,
                    String::from("<="),
                    Location::new(input.clone(), vec![coordinate]),
                    );
                    tokens.push(token);
                }
            }
            '>' => {
                // Same idea as above
                if !let Some((next_coord, next_char)) = characters.peek() || next_char != '=' {
                    let token = Token::new(
                    TokenType::Great,
                    String::from(">"),
                    Location::new(input.clone(), vec![coordinate]),
                    );
                    tokens.push(token);
                } else {
                    let token = Token::new(
                    TokenType::GreatEqual,
                    String::from(">="),
                    Location::new(input.clone(), vec![coordinate]),
                    );
                    tokens.push(token);
                }
            }
            '/' => {
                let token = Token::new(
                    TokenType::ForwardSlash,
                    String::from("/"),
                    Location::new(input.clone(), vec![coordinate]),
                );
                tokens.push(token);
            }
            '\\' => {
                // must be double bc \ its a escape character in Rust
                let token = Token::new(
                    TokenType::ForwardSlash,
                    String::from("\\"),
                    Location::new(input.clone(), vec![coordinate]),
                );
                tokens.push(token);
            }
            '\'' => {
                let token = Token::new(
                    TokenType::Quote,
                    String::from("\'"),
                    Location::new(input.clone(), vec![coordinate]),
                );
                tokens.push(token);
            }
            '`' => {
                let token = Token::new(
                    TokenType::BackQuote,
                    String::from("`"),
                    Location::new(input.clone(), vec![coordinate]),
                );
                tokens.push(token);
            }
            ',' => {
                let token = Token::new(
                    TokenType::Comma,
                    String::from(","),
                    Location::new(input.clone(), vec![coordinate]),
                );
                tokens.push(token);
            }
            '#' => {
                let token = Token::new(
                    TokenType::Hashtag,
                    String::from("#"),
                    Location::new(input.clone(), vec![coordinate]),
                );
                tokens.push(token);
            }
            '=' => {
                let token = Token::new(
                    TokenType::Equal,
                    String::from("="),
                    Location::new(input.clone(), vec![coordinate]),
                );
                tokens.push(token);
            }
            // String
            '\"' => {
                let mut lexeme = String::from("\"");
                let mut coordinates = vec![coordinate];
                // Iterate until we find a space

                if let Some((_, next)) = characters.peek() {
                    // Leemos hasta que haya otro "
                    // Si es parte de los Token conocidos, corta. Puede estar pegado al character
                    // Ej: (define (factorial n)
                    // Esto trata de atrapar el ) en n)
                    if TokenType::from_str(next.to_string().as_str()).is_err() {
                        while let Some((coord, letter)) = characters.next()
                            && letter != '\"'
                        {
                            lexeme.push(letter);
                            coordinates.push(coord);
                        }
                    }
                }

                let literal = {
                    let mut chars = lexeme.chars();
                    chars.next();
                    chars.next_back();
                    chars.as_str().to_string()
                };

                let token = Token::new(
                    TokenType::String { literal },
                    lexeme,
                    Location::new(input.clone(), coordinates),
                );
                tokens.push(token);
            }
            number if character.is_numeric() => {
                let valid_chars_in_numbers = ['.'];
                let mut count_of_dots = 0;

                let mut lexeme = String::from(number);
                let mut coordinates = vec![coordinate];
                // Iterate until we find a space

                if let Some((_, next)) = characters.peek() {
                    // Si es parte de los Token conocidos, corta. Puede estar pegado al character
                    // Ej: (define (factorial n)
                    // Esto trata de atrapar el ) en n)
                    if TokenType::from_str(next.to_string().as_str()).is_err() {
                        while let Some((coord, letter)) = characters.next()
                            && letter != ' '
                            && (letter.is_numeric() || valid_chars_in_numbers.contains(&letter))
                        {   
                            if valid_chars_in_numbers.contains(&letter) && count_of_dots >= 1 {
                                // I don't know how to do it, i throw panic (go moment)
                                panic!("Invalid number detected!");
                            } else {
                                lexeme.push(letter);
                                coordinates.push(coord);
                                count_of_dots += 1;
                            }
                        }
                    }
                }

                let token = Token::new(
                    // In the case of a number, the literal _is_ the lexeme
                    TokenType::Number {
                        literal: lexeme.clone(),
                    },
                    lexeme,
                    Location::new(input.clone(), coordinates),
                );
                tokens.push(token);
            }
            // Caso palabra: O es un identificador o una palabra reservada
            letter if character.is_alphabetic() => {
                let mut lexeme = String::from(letter);
                let mut coordinates = vec![coordinate];

                if let Some((_, next)) = characters.peek() {
                    // Si es parte de los Token conocidos, corta. Puede estar pegado al character
                    // Ej: (define (factorial n)
                    // Esto trata de atrapar el ) en n)
                    if TokenType::from_str(next.to_string().as_str()).is_err() {
                        while let Some((coord, letter)) = characters.next()
                            && letter != ' '
                        {
                            lexeme.push(letter);
                            coordinates.push(coord);
                        }
                    }
                }

                // Iterate until we find a space

                // Si es una palabra reservada, la guardamos como tal. Sino, es un identificador.
                let token_type = if let Ok(reserved_keyword) = ReservedKeywords::from_str(&lexeme) {
                    TokenType::ReservedKeywords(reserved_keyword)
                } else {
                    TokenType::Identifier
                };

                let token = Token::new(
                    token_type,
                    lexeme,
                    Location::new(input.clone(), coordinates),
                );

                tokens.push(token);
            }
            _ => (),
        };
        // tokens.append(token);
    }

    std::dbg!(&tokens);
    tokens
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
