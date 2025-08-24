use std::fmt::Display;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};

/// Enum > Trait

/// "Traits" - Interfaz

/// All the tokens we know
pub(crate) enum TokenType {
    /// (
    LeftParens,
    /// )
    RightParens,
    // +
    Plus,
    // -
    Minus,
    // *
    Star,
    // . // second best comment
    Dot,
    // / // best comment
    ForwardSlash,
    // \
    BackSlash,
    // [
    LeftBracket,
    // ]
    RightBracket,
    // '
    Quote,
    // `
    BackQuote,
    // `
    Quasiquote,
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

struct Location {
    input: Input,
    coord: Coordinate,
}

pub struct Token {
    token_type: TokenType,

    /// Lexeme that originated the Token
    lexeme: String,

    location: Location,
}

// Lee todo el input y lo devuelve como un string. Lo aloca todo de una, asique que nadie lea Don Quijote de la Mancha con esta funcion!
fn read<R: Read>(mut input: BufReader<R>) -> String {
    let mut buffer = String::new();
    let len = input
        .read_to_string(&mut buffer)
        .expect("Failed to read line");

    buffer
}

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
    let columns = text
        .lines()
        .flat_map(|line| line.chars().enumerate().map(|(column, _)| column))
        // Le sumo 1 al line number porque enumerate arranca en 0 pero la
        // primera linea de un archivo es la linea 1. Fuente: este archivo
        .map(|column_number| (column_number + 1));

    // We add the mut to use the next method
    let mut characters = text
        .lines()
        .enumerate()
        // Le sumo 1 al line number porque enumerate arranca en 0 pero la
        // primera linea de un archivo es la linea 1. Fuente: este archivo
        .map(|(line_number, line)| (line_number + 1, line))
        .flat_map(|(line_number, line)| line.chars().map(move |cha| (cha, line_number)))
        // Le anado las columnas
        .zip(columns)
        .map(|((character, line), column)| (character, line, column))
        .map(|(character, line, column)| (Coordinate::new(line as u32, column as u32), character));

    let mut start = 0;
    let mut current = 0;

    let mut tokens: Vec<Token> = Vec::new();

    // We read until we run out of characters
    while let Some((coordiate, character)) = characters.next() {
        dbg!(coordiate, character);
        characters.next();
        let token = match character {
            '(' => todo!(),
        };

        // tokens.append(token);
    }

    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
