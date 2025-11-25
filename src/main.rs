mod decoder;
mod encoder;
mod errors;
mod state;
mod tags;

use errors::QOIError;
use state::QOIState;

fn main() -> Result<(), crate::errors::QOIError> {
    use crate::errors::QOIError;

    let args: Vec<String> = std::env::args().collect();

    fn input_image_path(args: &Vec<String>) -> Result<String, QOIError> {
        args.get(2).cloned().ok_or(QOIError::InvalidArgs(
            "Invalid arguments: Input image path not specified".to_owned(),
        ))
    }

    fn output_image_path(args: &Vec<String>) -> Result<String, QOIError> {
        args.get(3).cloned().ok_or(QOIError::InvalidArgs(
            "Invalid arguments: Output image path not specified".to_owned(),
        ))
    }

    match args.get(1).expect("Invalid no. of arguments").as_str() {
        "--encode" | "-e" => {
            let input = input_image_path(&args)?;
            let output = output_image_path(&args)?;
            encoder::encode_file(&input)?.write(&output)?
        }

        "--decode" | "-d" => {
            let input = input_image_path(&args)?;
            let output = output_image_path(&args)?;
            decoder::decode_file(&input)?
                .save(output)
                .map_err(|_| QOIError::FileWriteError)?
        }

        "--help" | "-h" => {
            println!("qoi-codec v0.0.1");
            println!();
            println!("USAGE:");
            println!("    qoi-codec <OPTIONS> <INPUT_FILE> <OUTPUT_FILE>");
            println!();
            println!("OPTIONS:");
            println!("    -e, --encode  Convert image to QOI.");
            println!("    -d, --decode  Convert QOI image to another format.");
            println!("                  ( format inferred from extension )");
            println!("    -h, --help    Display this help screen.");
            println!();
            println!("INPUT_FILE:");
            println!("    Path to the image file.");
            println!();
            println!("OUTPUT_FILE:");
            println!("    Path of image file to export to");
            println!("    ( Will be overwritten if already exists )");
        }

        _ => panic!("Invalid command. Expected '--encode' or '--decode' or '--help'"),
    }

    Ok(())
}
