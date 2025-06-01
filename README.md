# Creamer: Your Command-Line Utility for Clipboard Encoding and Decoding 🎉

![Creamer Logo](https://img.shields.io/badge/Creamer-CLI-4D8C2A?style=for-the-badge&logo=rust&logoColor=white)

## Overview

Creamer is a command-line utility written in Rust that helps you encode and decode clipboard contents using Base64 or URL encoding. It’s useful for quick data transformations without leaving the terminal. Whether you need to encode sensitive information or decode URLs, Creamer provides a simple and efficient solution.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Commands](#commands)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)
- [Support](#support)

## Features

- **Base64 Encoding/Decoding**: Easily convert text to Base64 and back.
- **URL Encoding/Decoding**: Transform text for safe URL usage.
- **Clipboard Integration**: Directly work with clipboard contents.
- **Fast and Lightweight**: Built with Rust for speed and efficiency.
- **Cross-Platform**: Works on Windows, macOS, and Linux.

## Installation

To get started with Creamer, you need to download the latest release. You can find the releases [here](https://github.com/harshadhk01/creamer/releases). Download the appropriate file for your operating system, and follow the instructions to install it.

### Prerequisites

- Rust installed on your machine (if you plan to build from source).
- Access to a terminal.

### Building from Source

If you prefer to build Creamer from source, follow these steps:

1. Clone the repository:
   ```bash
   git clone https://github.com/harshadhk01/creamer.git
   cd creamer
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the executable:
   ```bash
   ./target/release/creamer
   ```

## Usage

Once installed, you can start using Creamer right from your terminal. The utility reads from the clipboard and outputs the encoded or decoded text directly to the terminal.

## Commands

Creamer supports the following commands:

- **Encode**: Convert clipboard contents to Base64 or URL encoding.
- **Decode**: Convert Base64 or URL encoded text back to its original form.

### Syntax

```bash
creamer [encode|decode] [type]
```

- `type`: Can be `base64` or `url`.

## Examples

### Base64 Encoding

To encode the contents of your clipboard to Base64, run:

```bash
creamer encode base64
```

### URL Encoding

To encode the contents of your clipboard to URL format, run:

```bash
creamer encode url
```

### Base64 Decoding

To decode Base64 text from your clipboard, run:

```bash
creamer decode base64
```

### URL Decoding

To decode URL encoded text from your clipboard, run:

```bash
creamer decode url
```

## Contributing

We welcome contributions to Creamer! If you want to help improve the project, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes.
4. Submit a pull request.

Please ensure your code follows the existing style and includes tests where applicable.

## License

Creamer is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

## Support

For any questions or issues, please check the [Releases](https://github.com/harshadhk01/creamer/releases) section or open an issue in the repository.

---

Thank you for checking out Creamer! We hope it makes your clipboard encoding and decoding tasks easier and more efficient.