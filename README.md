# Creamer ☕️

**Creamer** is a command-line utility written in Rust that helps you encode and decode clipboard contents using Base64 or URL encoding. It’s useful for quick data transformations without leaving the terminal.

## Features

- ✅ Encode clipboard text to **Base64** or **URL**
- ✅ Decode Base64/URL text from the clipboard
- ✅ Copy output back to clipboard with `-c` flag
- ✅ Clear error handling for invalid inputs

## Installation

```bash
git clone https://github.com/yourusername/creamer.git
cd creamer
cargo build --release
```
The compiled binary will be in `target/release/creamer`.

## Usage

```bash
creamer [OPTIONS]
```

### Options

* `-e, --encode <TYPE>` — Encode clipboard contents using the specified type
* `-d, --decode <TYPE>` — Decode clipboard contents using the specified type
* `-c, --copy` — Copy the result back to clipboard after displaying
* `-h, --help` — Show help message

### Encoding/Decoding Types

* `base64` — Base64 encode/decode
* `url` — URL encode/decode

### Examples

```bash
creamer -e base64
# Encodes clipboard content to Base64 and prints it

creamer -d base64 -c
# Decodes clipboard Base64 text, prints it, and copies the result back

creamer -e url
# URL-encodes clipboard content

creamer -d url -c
# URL-decodes clipboard content and copies result to clipboard
```

## Dependencies

* [`clap`](https://crates.io/crates/clap)
* [`clipboard`](https://crates.io/crates/clipboard)
* [`base64`](https://crates.io/crates/base64)
* [`urlencoding`](https://crates.io/crates/urlencoding)

