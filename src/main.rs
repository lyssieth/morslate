use clap::{App, Arg};
use morse_nostd::{decode::decode as decoder, encode::encode as encoder};

fn main() {
    let app = build_app();
    let matches = app.get_matches();

    let input = matches.value_of("input").expect("Input was not specified");

    if matches.is_present("--decode") {
        println!("{}", decode(input));
    } else if matches.is_present("--encode") {
        println!("{}", encode(input));
    } else {
        eprintln!("--encode or --decode is required.");
    }
}

fn decode(input: &str) -> String {
    if input.contains('/') {
        let mut out = String::with_capacity(input.len());
        let split = input.split('/');

        for s in split {
            out.push_str(&decoder(s).expect("invalid characters detected"));
            out.push(' ');
        }

        out
    } else {
        decoder(input).expect("invalid characters detected")
    }
}

fn encode(input: &str) -> String {
    encoder(input).expect("invalid characters detected")
}

fn build_app() -> App<'static> {
    App::new("morslate")
        .version("1.0")
        .author("Lyssieth <lyssieth@rax.ee>")
        .arg(Arg::new("input").index(1).required(true))
        .arg(
            Arg::new("--decode")
                .help("decodes the given input")
                .long("decode")
                .short('d')
                .conflicts_with("--encode"),
        )
        .arg(
            Arg::new("--encode")
                .help("encodes the given input")
                .long("encode")
                .short('e')
                .conflicts_with("--decode"),
        )
}
