use logos::Logos;

#[derive(Logos, Debug, Eq, PartialEq, Hash)]
pub enum Token {
    // util
    #[regex(r"\s+")]
    Whitespace,

    #[regex(r"\d+")]
    Number,

    #[token(".")]
    Punto,

    // kombina
    #[token("ch")]
    Ch,
    #[token("dj")]
    Dj,
    #[token("zj")]
    Zj,
    #[token("sh")]
    Sh,
    #[token("ks")]
    Ks,

    // vokal
    #[token("a")]
    A,
    #[token("e")]
    E,
    #[token("i")]
    I,
    #[token("o")]
    O,
    #[token("u")]
    U,
    // aksènt
    #[token("è")]
    EGrave,
    #[token("é")]
    EAcute,
    #[token("í")]
    IAcute,
    #[token("ò")]
    OGrave,
    #[token("ó")]
    OAcute,
    #[token("ù")]
    UGrave,
    #[token("ú")]
    UAcute,
    #[token("ü")]
    UUmlaut,


    // konsonante
    #[token("b")]
    B,
    #[token("c")] // 👀
    C,
    #[token("d")]
    D,
    #[token("f")]
    F,
    #[token("g")]
    G,
    #[token("h")]
    H,
    #[token("j")] // 👀
    J,
    #[token("k")]
    K,
    #[token("l")]
    L,
    #[token("m")]
    M,
    #[token("n")]
    N,
    #[token("p")]
    P,
    #[token("q")] // 👀
    Q,
    #[token("r")]
    R,
    #[token("s")]
    S,
    #[token("t")]
    T,
    #[token("v")]
    V,
    #[token("w")]
    W,
    #[token("x")] // 👀
    X,
    #[token("y")]
    Y,
    #[token("z")]
    Z,

    #[error]
    Error,
}

pub fn generate(input: &str) -> Vec<Token> {
    Token::lexer(input).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_tokenize_letters() {
        let mut lexer = Token::lexer("abcdefghijklmnopqrstuvwxyz");

        assert_eq!(lexer.next(), Some(Token::A));
        assert_eq!(lexer.next(), Some(Token::B));
        assert_eq!(lexer.next(), Some(Token::C));
        assert_eq!(lexer.next(), Some(Token::D));
        assert_eq!(lexer.next(), Some(Token::E));
        assert_eq!(lexer.next(), Some(Token::F));
        assert_eq!(lexer.next(), Some(Token::G));
        assert_eq!(lexer.next(), Some(Token::H));
        assert_eq!(lexer.next(), Some(Token::I));
        assert_eq!(lexer.next(), Some(Token::J));
        assert_eq!(lexer.next(), Some(Token::K));
        assert_eq!(lexer.next(), Some(Token::L));
        assert_eq!(lexer.next(), Some(Token::M));
        assert_eq!(lexer.next(), Some(Token::N));
        assert_eq!(lexer.next(), Some(Token::O));
        assert_eq!(lexer.next(), Some(Token::P));
        assert_eq!(lexer.next(), Some(Token::Q));
        assert_eq!(lexer.next(), Some(Token::R));
        assert_eq!(lexer.next(), Some(Token::S));
        assert_eq!(lexer.next(), Some(Token::T));
        assert_eq!(lexer.next(), Some(Token::U));
        assert_eq!(lexer.next(), Some(Token::V));
        assert_eq!(lexer.next(), Some(Token::W));
        assert_eq!(lexer.next(), Some(Token::X));
        assert_eq!(lexer.next(), Some(Token::Y));
        assert_eq!(lexer.next(), Some(Token::Z));
    }

    // è é í ò ó

    #[test]
    fn it_can_tokenize_accented_letters() {
        let mut lexer = Token::lexer("èéíòó");

        assert_eq!(lexer.next(), Some(Token::EGrave));
        assert_eq!(lexer.next(), Some(Token::EAcute));
        assert_eq!(lexer.next(), Some(Token::IAcute));
        assert_eq!(lexer.next(), Some(Token::OGrave));
        assert_eq!(lexer.next(), Some(Token::OAcute));
    }

    //  CH DJ ZJ SH KS

    #[test]
    fn it_can_tokenize_ch() {
        let mut lexer = Token::lexer("ch");

        assert_eq!(lexer.next(), Some(Token::Ch));
    }

    #[test]
    fn it_can_tokenize_dj() {
        let mut lexer = Token::lexer("dj");

        assert_eq!(lexer.next(), Some(Token::Dj));
    }

    #[test]
    fn it_can_tokenize_zj() {
        let mut lexer = Token::lexer("zj");

        assert_eq!(lexer.next(), Some(Token::Zj));
    }

    #[test]
    fn it_can_tokenize_sh() {
        let mut lexer = Token::lexer("sh");

        assert_eq!(lexer.next(), Some(Token::Sh));
    }

    #[test]
    fn it_can_tokenize_ks() {
        let mut lexer = Token::lexer("ks");

        assert_eq!(lexer.next(), Some(Token::Ks));
    }
}
