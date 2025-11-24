use crate::errors::QOIError;
use std::env;

mod decoder;
mod encoder;
mod errors;

fn main() -> Result<(), QOIError> {
    let args: Vec<String> = env::args().collect();
    let input_image = args.get(2).ok_or(QOIError::InvalidArgs)?;
    let output_image = args.get(3).ok_or(QOIError::InvalidArgs)?;

    match args.get(1).expect("Invalid no. of arguments").as_str() {
        "--encode" | "-e" => encoder::encode_file(&input_image)?.write(&output_image)?,

        "--decode" | "-d" => {}
        "--help" | "-h" => {}
        _ => panic!("Invalid command. Expected '--encode' or '--decode' or '--help'"),
    }

    Ok(())
}
