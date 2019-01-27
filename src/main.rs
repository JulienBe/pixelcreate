extern crate png;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use png::HasParameters;

const MIN: u8 = 1;
const MAX: u8 = 5;

fn main() {
    println!("creating image");

    for r in MIN..MAX {
        for g in MIN..MAX {
            for b in MIN..MAX {
                let imgName = format!("images/{}{}{}.png", r, g, b);
                let path = Path::new(&imgName);
                let file = File::create(path).unwrap();
                let ref mut w = BufWriter::new(file);

                let mut encoder = png::Encoder::new(w, 1, 1);
                encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
                let mut writer = encoder.write_header().unwrap();

                let data = [r * (255 / (MAX - MIN)), g * (255 / (MAX - MIN)), b * (255 / (MAX - MIN)), 255]; // RGBA
                writer.write_image_data(&data).unwrap(); // Save
            }
        }
    }
}
