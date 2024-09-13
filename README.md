# Ron_Way

`Ron_Way` is a command-line tool designed to convert Alacritty terminal themes in TOML format to Cosmic Terminal themes in RON format. This is my first Rust project and I hope it helps others, even if it's janky as hell.

## Features

- **TOML to RON Conversion**: Easily convert Alacritty color schemes from TOML to RON format.
- **Dynamic Color Adjustment**: Automatically generates dim color values based on the normal color values.

## Installation

To install `Ron_Way`, clone this repository and build the project using Cargo.

```bash
git clone https://github.com/yourusername/Ron_Way.git
cd Ron_Way
cargo install --path .
```

Alternatively, you can build and install it locally without adding it to Cargo's global install path:

```bash
cargo build --release
```

## Usage

Once installed, you can use `Ron_Way` from the command line to convert your theme files.

```bash
Ron_Way <input.toml> <output.ron>
```

### Arguments

- `<input.toml>`: Path to the input TOML file containing the Alacritty theme.
- `<output.ron>`: Path to the output RON file where the converted Cosmic Terminal theme will be saved.

### Example

```bash
Ron_Way ~/Downloads/gruvbox_light.toml ~/Downloads/gruvbox_light.ron
```

This command will read the `gruvbox_light.toml` file from the `Downloads` directory and output the converted RON file as `gruvbox_light.ron` in the same directory.

## Configuration

`Ron_Way` will automatically extract the name for the theme from the input TOML file. If the name is not found, it defaults to using the file name (without extension) as the theme name.

## Contributing

Feel free to open issues or submit pull requests if you have suggestions for improvements or encounter any bugs.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

.

---
