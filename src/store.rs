use crate::lexer::Token;
use std::collections::HashMap;

pub fn get_sounds() -> HashMap<Token, String> {
    HashMap::from([
        (Token::A, "assets/A.wav".to_string()),
        (Token::B, "assets/B.wav".to_string()),
        (Token::C, "assets/C.wav".to_string()),
        (Token::D, "assets/D.wav".to_string()),
        (Token::E, "assets/E.wav".to_string()),
        (Token::F, "assets/F.wav".to_string()),
        (Token::G, "assets/G.wav".to_string()),
        (Token::H, "assets/H.wav".to_string()),
        (Token::I, "assets/I.wav".to_string()),
        (Token::J, "assets/J.wav".to_string()),
        (Token::K, "assets/K.wav".to_string()),
        (Token::L, "assets/L.wav".to_string()),
        (Token::M, "assets/M.wav".to_string()),
        (Token::N, "assets/N.wav".to_string()),
        (Token::O, "assets/O.wav".to_string()),
        (Token::P, "assets/P.wav".to_string()),
        (Token::Q, "assets/Q.wav".to_string()),
        (Token::R, "assets/R.wav".to_string()),
        (Token::S, "assets/S.wav".to_string()),
        (Token::T, "assets/T.wav".to_string()),
        (Token::U, "assets/U.wav".to_string()),
        (Token::V, "assets/V.wav".to_string()),
        (Token::W, "assets/W.wav".to_string()),
        (Token::X, "assets/X.wav".to_string()),
        (Token::Y, "assets/Y.wav".to_string()),
        (Token::Z, "assets/Z.wav".to_string()),
    ])
}

mod tests {

    #[test]
    fn it_can_read_wav_files() {
        use super::*;
        use std::io::BufReader;

        let sounds = get_sounds();

        // test all the wav files in the assets directory
        for (_, path) in sounds {
            let file = std::fs::File::open(path).unwrap();
            let mut decoder = rodio::Decoder::new(BufReader::new(file)).unwrap();
            assert!(decoder.any(|x| x != 0)); // Assert not all zeros
        }
    }
}
