use crate::raster::{Image, Rgba};

pub mod raster;

fn main() {
    let mut image = Image::new(512, 512);

    for y in 0..512 {
        for x in 0..512 {
            image.set_pixel(x, y, Rgba::new((x ^ y) as u8, 0, 255, 255));
        }
    }

    image.save_png("test.png");
}
