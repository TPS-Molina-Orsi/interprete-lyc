use std::fmt::Display;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;

/// Enum > Trait

/// "Traits" - Interfaz

/// All the tokens we know
pub(crate) enum TokenType {
    // (
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

enum Input {
    Stdin,

    File { path: PathBuf },
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

struct Scanner {
    /// Start of the current lexeme
    start: u32,
    /// Current pointer of the lexeme
    /// SIDENOTE: This doc comments suck.
    current: u32,
}

// No exponemos el struct Scanner por fuera de la crate porque es un detalle de
// implementacion. A los consumidores solo les interesa la funcion Scan y que
// devulva tokens
impl Scanner {
    pub(crate) fn new() -> Scanner {
        Scanner {
            start: 0,
            current: 0,
        }
    }

    // Implementacion del scaneo
    pub(crate) fn scan(&mut self, text: String) -> Vec<Token> {
        // Le paso enumerate para tener la posicion del character

        // Esto nos devuelve una tripla del tipo:
        //
        let columns = text
            .lines()
            .flat_map(|line| line.chars().enumerate().map(|(column, _)| column))
            // Le sumo 1 al line number porque enumerate arranca en 0 pero la
            // primera linea de un archivo es la linea 1. Fuente: este archivo
            .map(|column_number| (column_number + 1));

        let lines = text
            .lines()
            .enumerate()
            // Le sumo 1 al line number porque enumerate arranca en 0 pero la
            // primera linea de un archivo es la linea 1. Fuente: este archivo
            .map(|(line_number, line)| (line_number + 1, line))
            .flat_map(|(line_number, line)| line.chars().map(move |cha| (cha, line_number)))
            // Le anado las columnas
            .zip(columns)
            .map(|((character, line), column)| (character, line, column))
            .map(|(character, line, column)| {
                (Coordinate::new(line as u32, column as u32), character)
            });

        for (coordiate, character) in lines {
            dbg!(coordiate, character);
        }

        todo!()
    }
}

/// Function that handles scanning
/// This function will scan all the "lexemes" present in the passed in the input (either a file or stdin).
pub fn scan<R: Read>(mut input: BufReader<R>) -> Vec<Token> {
    let text = self::read(input);

    let mut scanner = Scanner::new();

    let tokens = scanner.scan(text);
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
