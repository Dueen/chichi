use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // utils
    Unknown,
    Space,
    Punto,

    // kombina
    Ch,
    Dj,
    Zj,
    Sh,
    Ks,

    // vokal
    A,
    E,
    I,
    O,
    U,

    // aksènt
    AGrave,
    AAcute,
    EGrave,
    EAcute,
    IAcute,
    OGrave,
    OAcute,
    UGrave,
    UAcute,
    UUmlaut,
    Ntilde,

    // konsonante
    B,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    M,
    N,
    P,
    Q,
    R,
    S,
    T,
    V,
    W,
    Y,
    Z,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub path: PathBuf,
}

pub struct Lexer {}

impl Lexer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn tokenize(&mut self, content: &String) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = content.chars().peekable();

        let mut push = |kind: TokenKind, path: &str| {
            let path = std::path::PathBuf::from(format!(
                "{}/{}/{}{}",
                env!("CARGO_MANIFEST_DIR"),
                "assets",
                path.to_lowercase(),
                ".wav"
            ));
            tokens.push(Token { kind, path });
        };

        while let (Some(x), y) = (chars.next(), chars.peek()) {
            match (x, y) {
                // kombina
                ('c', Some('h')) => {
                    push(TokenKind::Ch, "A"); // FIXME: add CH sound
                    chars.next();
                }
                ('d', Some('j')) => {
                    push(TokenKind::Dj, "A"); // FIXME: add DJ sound
                    chars.next();
                }
                ('z', Some('j')) => {
                    push(TokenKind::Zj, "A"); // FIXME: add ZJ sound
                    chars.next();
                }
                ('s', Some('h')) => {
                    push(TokenKind::Sh, "A"); // FIXME: add SH sound
                    chars.next();
                }
                ('k', Some('s')) => {
                    push(TokenKind::Ks, "A"); // FIXME: add KS sound
                    chars.next();
                }

                // vokal
                ('a', _) => push(TokenKind::A, "A"),
                ('e', _) => push(TokenKind::E, "E"),
                ('i', _) => push(TokenKind::I, "I"),
                ('o', _) => push(TokenKind::O, "O"),
                ('u', _) => push(TokenKind::U, "U"),

                // aksènt
                ('à', _) => push(TokenKind::AGrave, "A"), // FIXME: AGrave
                ('á', _) => push(TokenKind::AAcute, "A"), // FIXME: AAcute
                ('è', _) => push(TokenKind::EGrave, "E"), // FIXME: EGrave
                ('é', _) => push(TokenKind::EAcute, "E"), // FIXME: EAcute
                ('í', _) => push(TokenKind::IAcute, "I"), // FIXME: IAcute
                ('ò', _) => push(TokenKind::OGrave, "O"), // FIXME: OGrave
                ('ó', _) => push(TokenKind::OAcute, "O"), // FIXME: OAcute
                ('ù', _) => push(TokenKind::UGrave, "U"), // FIXME: UGrave
                ('ú', _) => push(TokenKind::UAcute, "U"), // FIXME: UAcute
                ('ü', _) => push(TokenKind::UUmlaut, "U"), // FIXME: UUmlaut
                ('ñ', _) => push(TokenKind::Ntilde, "N"), // FIXME: Ntilde

                // konsonante
                ('b', _) => push(TokenKind::B, "B"),
                ('d', _) => push(TokenKind::D, "D"),
                ('f', _) => push(TokenKind::F, "F"),
                ('g', _) => push(TokenKind::G, "G"),
                ('h', _) => push(TokenKind::H, "H"),
                ('k', _) => push(TokenKind::K, "K"),
                ('l', _) => push(TokenKind::L, "L"),
                ('m', _) => push(TokenKind::M, "M"),
                ('n', _) => push(TokenKind::N, "N"),
                ('p', _) => push(TokenKind::P, "P"),
                ('q', _) => push(TokenKind::Q, "Q"),
                ('r', _) => push(TokenKind::R, "R"),
                ('s', _) => push(TokenKind::S, "S"),
                ('t', _) => push(TokenKind::T, "T"),
                ('v', _) => push(TokenKind::V, "V"),
                ('w', _) => push(TokenKind::W, "W"),
                ('y', _) => push(TokenKind::Y, "Y"),
                ('z', _) => push(TokenKind::Z, "Z"),

                // utils
                (' ', _) => push(TokenKind::Space, "A"), // FIXME: Add space sound
                ('.', _) => push(TokenKind::Punto, "A"), // FIXME: Add punto sound

                _ => push(TokenKind::Unknown, "A"), // FIXME: Add unknown sound or error
            }
        }

        tokens
    }
}

// TODO: Add tests

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_can_tokenize_letters() {
//         let mut lexer = Lexer::tokenize("abcdefghijklmnopqrstuvwxyz");

//         assert_eq!(lexer.next(), Some(Token::A));
//         assert_eq!(lexer.next(), Some(Token::B));
//         assert_eq!(lexer.next(), Some(Token::C));
//         assert_eq!(lexer.next(), Some(Token::D));
//         assert_eq!(lexer.next(), Some(Token::E));
//         assert_eq!(lexer.next(), Some(Token::F));
//         assert_eq!(lexer.next(), Some(Token::G));
//         assert_eq!(lexer.next(), Some(Token::H));
//         assert_eq!(lexer.next(), Some(Token::I));
//         assert_eq!(lexer.next(), Some(Token::J));
//         assert_eq!(lexer.next(), Some(Token::K));
//         assert_eq!(lexer.next(), Some(Token::L));
//         assert_eq!(lexer.next(), Some(Token::M));
//         assert_eq!(lexer.next(), Some(Token::N));
//         assert_eq!(lexer.next(), Some(Token::O));
//         assert_eq!(lexer.next(), Some(Token::P));
//         assert_eq!(lexer.next(), Some(Token::Q));
//         assert_eq!(lexer.next(), Some(Token::R));
//         assert_eq!(lexer.next(), Some(Token::S));
//         assert_eq!(lexer.next(), Some(Token::T));
//         assert_eq!(lexer.next(), Some(Token::U));
//         assert_eq!(lexer.next(), Some(Token::V));
//         assert_eq!(lexer.next(), Some(Token::W));
//         assert_eq!(lexer.next(), Some(Token::X));
//         assert_eq!(lexer.next(), Some(Token::Y));
//         assert_eq!(lexer.next(), Some(Token::Z));
//     }

//     // è é í ò ó

//     #[test]
//     fn it_can_tokenize_accented_letters() {
//         let mut lexer = Token::lexer("èéíòó");

//         assert_eq!(lexer.next(), Some(Token::EGrave));
//         assert_eq!(lexer.next(), Some(Token::EAcute));
//         assert_eq!(lexer.next(), Some(Token::IAcute));
//         assert_eq!(lexer.next(), Some(Token::OGrave));
//         assert_eq!(lexer.next(), Some(Token::OAcute));
//     }

//     //  CH DJ ZJ SH KS

//     #[test]
//     fn it_can_tokenize_ch() {
//         let mut lexer = Token::lexer("ch");

//         assert_eq!(lexer.next(), Some(Token::Ch));
//     }

//     #[test]
//     fn it_can_tokenize_dj() {
//         let mut lexer = Token::lexer("dj");

//         assert_eq!(lexer.next(), Some(Token::Dj));
//     }

//     #[test]
//     fn it_can_tokenize_zj() {
//         let mut lexer = Token::lexer("zj");

//         assert_eq!(lexer.next(), Some(Token::Zj));
//     }

//     #[test]
//     fn it_can_tokenize_sh() {
//         let mut lexer = Token::lexer("sh");

//         assert_eq!(lexer.next(), Some(Token::Sh));
//     }

//     #[test]
//     fn it_can_tokenize_ks() {
//         let mut lexer = Token::lexer("ks");

//         assert_eq!(lexer.next(), Some(Token::Ks));
//     }
// }
