use std::fs::read_to_string;
use std::io::BufReader;

mod lexer;
use lexer::generate;

mod store;
use store::get_sounds;

fn main() {
    let sounds = get_sounds();

    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();

    let path = std::path::PathBuf::from(format!("{}/example.txt", env!("CARGO_MANIFEST_DIR")));
    let contents = read_to_string(path).unwrap().to_lowercase();

    let tokens = generate(&contents);

    // queue up all the sounds
    // TODO: use a match pattern to play the correct sound and catch errors
    for token in tokens {
        let file = std::fs::File::open(sounds.get(&token).unwrap()).unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        sink.append(source);
    }

    sink.sleep_until_end();
}
