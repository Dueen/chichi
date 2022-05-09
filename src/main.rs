use rodio::{Decoder, OutputStream, Sink};
use std::fs::{read_to_string, File};
use std::io::BufReader;

mod lexer;
use lexer::generate_tokens;

mod store;
use store::get_sound_path;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let path = std::path::PathBuf::from(format!("{}/example.txt", env!("CARGO_MANIFEST_DIR")));
    let contents = read_to_string(path).unwrap_or_default().to_lowercase();

    let tokens = generate_tokens(&contents);

    // queue up all the sounds
    // TODO: use a match pattern to play the correct sound and catch errors
    for token in tokens {
        let path = get_sound_path(&token);
        let file = File::open(path).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();
        sink.append(source);
    }

    sink.sleep_until_end();
}
