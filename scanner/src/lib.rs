use std::io::BufReader;
use std::path::PathBuf;

// struct Scanner {
//     field: i32,
// }

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

struct Coordinate {
    line: u32,
    column: u32,
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

/// Function that handles scanning
/// This function will scan all the "lexemes" present in the passed in the input (either a file or stdin).
pub fn scan<R>(input: BufReader<R>) -> Vec<Token> {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
