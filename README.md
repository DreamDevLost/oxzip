# OxZip

OxZip is a modern and efficient alternative to 7-Zip, written in Rust and leveraging the Slint UI library for a sleek and user-friendly experience. Designed for performance, security, and ease of use, OxZip provides powerful compression and extraction capabilities with a native, cross-platform graphical interface.

## Features
- **Fast and Secure**: Built with Rust for safety and performance.
- **Modern UI**: Uses the Slint UI library for a responsive and lightweight interface.
- **Multi-format Support**: Supports multiple compression formats, including ZIP, 7z, TAR, and more. (coming soon)
- **Cross-Platform**: Works on Windows, Linux, and macOS.
- **Command-Line Interface (CLI)**: In addition to the GUI, OxZip provides a powerful CLI for advanced users. (coming soon)
- **Multi-threaded Compression**: Leverages multi-core CPUs for faster compression and extraction. (coming soon)
- **Encryption Support**: Secure your archives with strong encryption algorithms. (coming soon)

## Installation
### Prerequisites
- Rust (latest stable version) - Install from [rustup.rs](https://rustup.rs)
- Cargo package manager (included with Rust)

### Build from Source
```sh
# Clone the repository
git clone https://github.com/DreamDevLost/oxzip.git
cd oxzip

# Build the project
cargo build --release

# Run the application
./target/release/oxzip
```

### Install via Cargo
Coming soon...

## Usage
### Graphical Interface (GUI)
Simply launch the application and use the intuitive UI to compress and extract files.

### Command-Line Interface (CLI) (Coming soon)
OxZip provides a CLI for power users:
```sh
# Compress a directory
oxzip compress -i my_folder -o archive.7z

# Extract an archive
oxzip extract -i archive.7z -o output_folder
```

## Roadmap
- [ ] Add support for more compression formats
- [ ] Implement drag-and-drop functionality
- [ ] Provide pre-built binaries for Windows, macOS, and Linux
- [ ] Optimize multi-threading performance
- [ ] Enhance encryption features

## Contributing
Contributions are welcome! Feel free to submit issues, feature requests, or pull requests on [GitHub](https://github.com/DreamDevLost/oxzip).

## License
OxZip is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments
Special thanks to the Rust and Slint communities for their amazing work!

