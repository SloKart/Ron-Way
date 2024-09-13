use ron_way::{read_toml, extract_name, convert_to_cosmic_colors, convert_to_ron};
use std::env;
use std::fs;
use std::process;

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
