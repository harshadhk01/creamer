use clap::{ArgGroup, Parser, ValueEnum};
use clipboard::{ClipboardContext, ClipboardProvider};
use base64::{engine::general_purpose, Engine as _};
use std::process;

/// creamer: encode/decode clipboard text as Base64 or URL encoding, with optional copy-back
#[derive(Parser, Debug)]
#[clap(author, version, about)]
#[clap(group(
    ArgGroup::new("action")
        .required(true)
        .args(&["encode", "decode"]),
))]
struct Cli {
    /// Encode clipboard text
    #[clap(short = 'e', long = "encode", value_enum)]
    encode: Option<EncodingType>,

    /// Decode clipboard text
    #[clap(short = 'd', long = "decode", value_enum)]
    decode: Option<EncodingType>,

    /// Also copy the output back to clipboard
    #[clap(short = 'c', long = "copy")]
    copy: bool,
}

/// Supported encoding types
#[derive(ValueEnum, Clone, Debug)]
enum EncodingType {
    Base64,
    Url,
}

fn main() {
    let cli = Cli::parse();

    // Initialize clipboard context
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap_or_else(|err| {
        eprintln!("Failed to access clipboard: {}", err);
        process::exit(1);
    });

    // Read clipboard
    let input = ctx.get_contents().unwrap_or_else(|err| {
        eprintln!("Failed to read clipboard contents: {}", err);
        process::exit(1);
    });

    // Perform action
    let result = if let Some(enc) = cli.encode.clone() {
        match enc {
            EncodingType::Base64 => general_purpose::STANDARD.encode(&input),
            EncodingType::Url => urlencoding::encode(&input).into_owned(),
        }
    } else if let Some(dec) = cli.decode.clone() {
        match dec {
            EncodingType::Base64 => general_purpose::STANDARD
                .decode(&input)
                .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
                .unwrap_or_else(|err| {
                    eprintln!("Base64 decode failed or invalid UTF-8: {}", err);
                    process::exit(1);
                }),
            EncodingType::Url => urlencoding::decode(&input)
                .map(|s| s.into_owned())
                .unwrap_or_else(|err| {
                    eprintln!("URL decode failed: {}", err);
                    process::exit(1);
                }),
        }
    } else {
        // Clap enforces one action, but just in case
        eprintln!("Either --encode or --decode must be specified");
        process::exit(1);
    };

    // Print result
    println!("{}", result);

    // Optionally copy back to clipboard
    if cli.copy {
        ctx.set_contents(result).unwrap_or_else(|err| {
            eprintln!("Failed to copy result to clipboard: {}", err);
            process::exit(1);
        });
    }
}
