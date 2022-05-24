// use std::fmt;

// macro_rules! push_token {
//     ($tokens:expr, $token:expr, $chars:expr) => {
//         match $chars.peek() {
//             Some(c) => match $token {
//                 TokenKind::C => println!("{:?}", c),
//                 TokenKind::D => println!("{:?}", c),
//                 TokenKind::Z => println!("{:?}", c),
//                 TokenKind::S => println!("{:?}", c),
//                 TokenKind::K => println!("{:?}", c),
//                 _ => {
//                     $tokens.push(Token { kind: $token });
//                 }
//             },
//             None => {
//                 ();
//             }
//         }
//         // $tokens.push(Token { kind: $token })
//     };
// }

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
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

// impl fmt::Display for TokenKind {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         use TokenKind::*;
//         match self {
//             Error => write!(f, "error"),
//             Whitespace => write!(f, "whitespace"),
//             Number => write!(f, "number"),
//             Punto => write!(f, "punto"),
//             Ch => write!(f, "ch"),
//             Dj => write!(f, "dj"),
//             Zj => write!(f, "zj"),
//             Sh => write!(f, "sh"),
//             Ks => write!(f, "ks"),
//             A => write!(f, "a"),
//             E => write!(f, "e"),
//             I => write!(f, "i"),
//             O => write!(f, "o"),
//             U => write!(f, "u"),
//             EGrave => write!(f, "e grave"),
//             EAcute => write!(f, "e acute"),
//             IAcute => write!(f, "i acute"),
//             OGrave => write!(f, "o grave"),
//             OAcute => write!(f, "o acute"),
//             UGrave => write!(f, "u grave"),
//             UAcute => write!(f, "u acute"),
//             UUmlaut => write!(f, "u umlaut"),
//             Ntilde => write!(f, "ntilde"),
//             B => write!(f, "b"),
//             D => write!(f, "d"),
//             F => write!(f, "f"),
//             G => write!(f, "g"),
//             H => write!(f, "h"),
//             J => write!(f, "j"),
//             K => write!(f, "k"),
//             L => write!(f, "l"),
//             M => write!(f, "m"),
//             N => write!(f, "n"),
//             P => write!(f, "p"),
//             Q => write!(f, "q"),
//             R => write!(f, "r"),
//             S => write!(f, "s"),
//             T => write!(f, "t"),
//             V => write!(f, "v"),
//             W => write!(f, "w"),
//             // X => write!(f, "x"),
//             Y => write!(f, "y"),
//             Z => write!(f, "z"),
//             _ => write!(f, "unknown"),
//         }
//     }
// }

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub path: String,
}

// impl fmt::Display for Token {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.kind)
//     }
// }

pub struct Lexer {}

impl Lexer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn tokenize(&mut self, content: &String) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut chars = content.chars().peekable();

        while let (Some(x), Some(y) )= (chars.next(), chars.peek()) {
            match (x, y) {
                // kombina
                ('c', 'h') => {tokens.push(Token { kind: TokenKind::Ch, path: "".to_string() }); chars.next();},
                ('d', 'j') => {tokens.push(Token { kind: TokenKind::Dj, path: "".to_string() }); chars.next();},
                ('z', 'j') => {tokens.push(Token { kind: TokenKind::Zj, path: "".to_string() }); chars.next();},
                ('s', 'h') => {tokens.push(Token { kind: TokenKind::Sh, path: "".to_string() }); chars.next();},
                ('k', 's') => {tokens.push(Token { kind: TokenKind::Ks, path: "".to_string() }); chars.next();},

                // vokal
                ('a', _) => tokens.push(Token { kind: TokenKind::A, path: "".to_string() }),
                ('e', _) => tokens.push(Token { kind: TokenKind::E, path: "".to_string() }),
                ('i', _) => tokens.push(Token { kind: TokenKind::I, path: "".to_string() }),
                ('o', _) => tokens.push(Token { kind: TokenKind::O, path: "".to_string() }),
                ('u', _) => tokens.push(Token { kind: TokenKind::U, path: "".to_string() }),

                // aksènt
                ('à', _) => tokens.push(Token { kind: TokenKind::AGrave, path: "".to_string() }),
                ('á', _) => tokens.push(Token { kind: TokenKind::AAcute, path: "".to_string() }),
                ('è', _) => tokens.push(Token { kind: TokenKind::EGrave, path: "".to_string() }),
                ('é', _) => tokens.push(Token { kind: TokenKind::EAcute, path: "".to_string() }),
                ('í', _) => tokens.push(Token { kind: TokenKind::IAcute, path: "".to_string() }),
                ('ò', _) => tokens.push(Token { kind: TokenKind::OGrave, path: "".to_string() }),
                ('ó', _) => tokens.push(Token { kind: TokenKind::OAcute, path: "".to_string() }),
                ('ù', _) => tokens.push(Token { kind: TokenKind::UGrave, path: "".to_string() }),
                ('ú', _) => tokens.push(Token { kind: TokenKind::UAcute, path: "".to_string() }),
                ('ü', _) => tokens.push(Token { kind: TokenKind::UUmlaut, path: "".to_string() }),
                ('ñ', _) => tokens.push(Token { kind: TokenKind::Ntilde, path: "".to_string() }),

                // konsonante
                ('b', _) => tokens.push(Token { kind: TokenKind::B, path: "".to_string() }),
                ('d', _) => tokens.push(Token { kind: TokenKind::D, path: "".to_string() }),
                ('f', _) => tokens.push(Token { kind: TokenKind::F, path: "".to_string() }),
                ('g', _) => tokens.push(Token { kind: TokenKind::G, path: "".to_string() }),
                ('h', _) => tokens.push(Token { kind: TokenKind::H, path: "".to_string() }),
                ('j', _) => tokens.push(Token { kind: TokenKind::J, path: "".to_string() }),
                ('k', _) => tokens.push(Token { kind: TokenKind::K, path: "".to_string() }),
                ('l', _) => tokens.push(Token { kind: TokenKind::L, path: "".to_string() }),
                ('m', _) => tokens.push(Token { kind: TokenKind::M, path: "".to_string() }),
                ('n', _) => tokens.push(Token { kind: TokenKind::N, path: "".to_string() }),
                ('p', _) => tokens.push(Token { kind: TokenKind::P, path: "".to_string() }),
                ('q', _) => tokens.push(Token { kind: TokenKind::Q, path: "".to_string() }),
                ('r', _) => tokens.push(Token { kind: TokenKind::R, path: "".to_string() }),
                ('s', _) => tokens.push(Token { kind: TokenKind::S, path: "".to_string() }),
                ('t', _) => tokens.push(Token { kind: TokenKind::T, path: "".to_string() }),
                ('v', _) => tokens.push(Token { kind: TokenKind::V, path: "".to_string() }),
                ('w', _) => tokens.push(Token { kind: TokenKind::W, path: "".to_string() }),
                ('x', _) => tokens.push(Token { kind: TokenKind::Ks, path: "".to_string() }),
                ('y', _) => tokens.push(Token { kind: TokenKind::Y, path: "".to_string() }),
                ('z', _) => tokens.push(Token { kind: TokenKind::Z, path: "".to_string() }),

                // utils
                (' ', _) => tokens.push(Token { kind: TokenKind::Space, path: "".to_string() }),
                ('.', _) => tokens.push(Token { kind: TokenKind::Punto, path: "".to_string() }),

                _ => tokens.push(Token { kind: TokenKind::Unknown, path: "".to_string() })
            }
        };

        tokens
    }
}