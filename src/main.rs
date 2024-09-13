use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::process;
use regex::Regex;

#[derive(Deserialize, Serialize, Debug)]
struct ColorsWrapper {
    colors: Colors,
}

#[derive(Deserialize, Serialize, Debug)]
struct Colors {
    primary: Option<PrimaryColors>,
    normal: Option<ColorSet>,
    bright: Option<ColorSet>,
}

#[derive(Deserialize, Serialize, Debug)]
struct PrimaryColors {
    background: String,
    foreground: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct ColorSet {
    black: String,
    red: String,
    green: String,
    yellow: String,
    blue: String,
    magenta: String,
    cyan: String,
    white: String,
}

#[derive(Serialize, Debug)]
struct CosmicColors {
    name: String,
    foreground: String,
    cursor: String,
    bright_foreground: String,
    dim_foreground: String,
    normal: ColorSet,
    bright: ColorSet,
    dim: ColorSet,
}

// Function to read the TOML file and parse it into ColorsWrapper struct
fn read_toml(file_path: &str) -> ColorsWrapper {
    let toml_str = fs::read_to_string(file_path).expect("Unable to read file");
    let colors: ColorsWrapper = toml::from_str(&toml_str).expect("Error parsing TOML");
    colors
}

// Function to darken a color
fn darken_color(hex: &str, factor: f32) -> String {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).expect("Invalid hex color");
    let g = u8::from_str_radix(&hex[2..4], 16).expect("Invalid hex color");
    let b = u8::from_str_radix(&hex[4..6], 16).expect("Invalid hex color");

    let r = (r as f32 * factor).max(0.0).min(255.0) as u8;
    let g = (g as f32 * factor).max(0.0).min(255.0) as u8;
    let b = (b as f32 * factor).max(0.0).min(255.0) as u8;

    format!("#{:02x}{:02x}{:02x}", r, g, b).to_uppercase()
}

// Function to extract the name from the TOML file or use the file name
fn extract_name(file_path: &str) -> String {
    let toml_str = fs::read_to_string(file_path).expect("Unable to read file");

    let name_re = Regex::new(r"#\s*Colors\s*\(([^)]+)\)").unwrap();
    if let Some(caps) = name_re.captures(&toml_str) {
        if let Some(name) = caps.get(1) {
            return name.as_str().trim().to_string();
        }
    }

    // Fallback to the file name if name is not found
    file_path.split('/').last().unwrap_or("Unnamed").replace(".toml", "")
}

// Function to convert Colors struct into CosmicColors for RON format
fn convert_to_cosmic_colors(colors_wrapper: &ColorsWrapper, name: &str) -> CosmicColors {
    let primary = colors_wrapper.colors.primary.as_ref().expect("Primary colors missing");
    let normal = colors_wrapper.colors.normal.as_ref().expect("Normal colors missing");
    let bright = colors_wrapper.colors.bright.as_ref().expect("Bright colors missing");

    let dim_factor = 0.2;

    let dim_colors = ColorSet {
        black: darken_color(&normal.black, dim_factor),
        red: darken_color(&normal.red, dim_factor),
        green: darken_color(&normal.green, dim_factor),
        yellow: darken_color(&normal.yellow, dim_factor),
        blue: darken_color(&normal.blue, dim_factor),
        magenta: darken_color(&normal.magenta, dim_factor),
        cyan: darken_color(&normal.cyan, dim_factor),
        white: darken_color(&normal.white, dim_factor),
    };

    CosmicColors {
        name: name.to_string(),
        foreground: primary.foreground.clone(),
        cursor: primary.foreground.clone(),
        bright_foreground: bright.white.clone(),
        dim_foreground: dim_colors.black.clone(),
        normal: normal.clone(),
        bright: bright.clone(),
        dim: dim_colors,
    }
}

// Function to convert CosmicColors struct into RON format
fn convert_to_ron(cosmic_colors: &CosmicColors) -> String {
    let ron_str = ron::ser::to_string_pretty(cosmic_colors, ron::ser::PrettyConfig::default())
        .expect("Error converting to RON");
    ron_str
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input.toml> <output.ron>", args[0]);
        process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];

    let name = extract_name(input_file);
    let colors_wrapper = read_toml(input_file);
    let cosmic_colors = convert_to_cosmic_colors(&colors_wrapper, &name);
    let ron_output = convert_to_ron(&cosmic_colors);

    fs::write(output_file, ron_output).expect("Unable to write RON file");

    println!("Conversion successful! RON file saved at: {}", output_file);
}
