
# Ron_Way

[![crates.io](https://img.shields.io/crates/v/ron_way.svg)](https://crates.io/crates/ron_way)
[![Documentation](https://docs.rs/ron_way/badge.svg)](https://docs.rs/ron_way)

`Ron_Way` is a command-line tool designed to convert Alacritty terminal themes in TOML format to Cosmic Terminal themes in RON format. This is my first Rust project, and I hope it helps others, even if it's a bit janky.

## Features

- **TOML to RON Conversion**: Easily convert Alacritty color schemes from TOML to RON format.
- **Dynamic Color Adjustment**: Automatically generates dim color values based on the normal color values.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ron_way = "0.1.0"
```

To install `Ron_Way`, clone this repository and build the project using Cargo:

```bash
git clone https://github.com/SloKart/Ron_Way.git
cd Ron_Way
cargo install --path .
```

Alternatively, you can build and install it locally without adding it to Cargo's global install path:

```bash
cargo build --release
```

## Usage

### Command Line

Once installed, you can use `ron_way` from the command line to convert your theme files:

```bash
ron_way <input.toml> <output.ron>
```

#### Arguments

- `<input.toml>`: Path to the input TOML file containing the Alacritty theme.
- `<output.ron>`: Path to the output RON file where the converted Cosmic Terminal theme will be saved.

#### Example

```bash
ron_way ~/Downloads/gruvbox_light.toml ~/Downloads/gruvbox_light.ron
```

This command will read the `gruvbox_light.toml` file from the `Downloads` directory and output the converted RON file as `gruvbox_light.ron` in the same directory.

### As a Library

You can also use `Ron_Way` as a dependency in your Rust projects. Here’s an example of how to use it:

```rust
use ron_way;

fn main() {
    // Example code here
}
```

## Configuration

`Ron_Way` will automatically extract the name for the theme from the input TOML file. If the name is not found, it defaults to using the file name (without extension) as the theme name.

## Contributing

Feel free to open issues or submit pull requests if you have suggestions for improvements or encounter any bugs.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
```

