use image::{GenericImageView, ImageReader, Rgba};
use std::{env, error::Error};

mod qoi;
use qoi::encoder::{DiffHandler, QoiEncoder, RunHandler, SeenHandler};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    match args.get(1).expect("Invalid no. of arguments").as_str() {
        "--encode" | "-e" => {
            let image_path = args.get(2).expect("Image file not provided.");
            let reader = ImageReader::open(image_path).expect("Couldn't open file.");
            let image = reader.decode().expect("Couldn't decode provided file.");

            let mut qoi_buffer = QoiEncoder::new(&image);
            let mut run_handler = RunHandler::new();
            let mut seen_handler = SeenHandler::new();
            let mut diff_handler = DiffHandler::new();

            for (_, _, pixel) in image.pixels() {
                let mut handled = false;
                run_handler.handle(&mut qoi_buffer, &pixel, &mut handled);
                seen_handler.handle(&mut qoi_buffer, &pixel, &mut handled);
                diff_handler.handle(&mut qoi_buffer, &pixel, &mut handled);
            }

            qoi_buffer.end_byte_stream();
        }

        "--decode" | "-d" => {}
        "--help" | "-h" => {}
        _ => panic!("Invalid command. Expected '--encode' or '--decode' or '--help'"),
    }

    Ok(())
}
