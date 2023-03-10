use scuffedcopy::{fatal_err, Config, Fatal};
use std::io;
use std::io::{Read, Write};

fn main() {
    print!("Source file: ");
    io::stdout().flush().or_fatal();

    let mut from_string = String::new();
    io::stdin().read_line(&mut from_string).or_fatal();

    print!("Destination file: ");
    io::stdout().flush().or_fatal();

    let mut to_string = String::new();
    io::stdin().read_line(&mut to_string).or_fatal();

    let from_trimmed = from_string.trim().to_string();
    let to_trimmed = to_string.trim().to_string();

    let mut config = Config::from_strings(from_trimmed, to_trimmed).or_fatal();

    if config.to.exists() {
        print!("File already exists. Overwrite? [Y/N]: ");
        io::stdout().flush().or_fatal();

        let mut overwrite_input = [0u8; 1];
        io::stdin().read(&mut overwrite_input).or_fatal();

        let overwrite_input_char = overwrite_input[0] as char;
        match overwrite_input_char.to_ascii_uppercase() {
            'Y' => config.overwrite = true,
            'N' => return,
            _ => fatal_err(format!(
                "Invalid input - expected: [Y/N], found: {overwrite_input_char}"
            )),
        }
    }
    scuffedcopy::run(&config).or_fatal()
}
