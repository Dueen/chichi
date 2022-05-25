use rodio::{Decoder, OutputStream, Sink};
use std::fs::{read_to_string, File};
use std::io::BufReader;

mod lexer;
use lexer::Lexer;

// TODO: Take a file path as an argument -> clap
fn main() {
    // Initialize the audio device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Read the file into a string
    let path = std::path::PathBuf::from(format!("{}/example.txt", env!("CARGO_MANIFEST_DIR")));
    let contents = read_to_string(path).unwrap_or_default().to_lowercase();

    // Tokenize the contents
    let mut lexer = Lexer::new();
    let tokens = lexer.tokenize(&contents);


    // queue up all the sounds
    for token in tokens.iter() {
        let file = File::open(token.path.clone()).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();
        sink.append(source);
    }

    // play all the sounds
    sink.sleep_until_end();
}
