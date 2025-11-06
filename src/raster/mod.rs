use std::{fs::File, io::BufWriter, path::Path};

pub struct Image {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl Image {
    pub const BYTES_PER_PIXEL: usize = 4;

    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0; Self::BYTES_PER_PIXEL * width * height],
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Rgba {
        let index = self.pixel_index(x, y);

        let r = self.data[index + 0];
        let g = self.data[index + 1];
        let b = self.data[index + 2];
        let a = self.data[index + 3];
        Rgba::new(r, g, b, a)
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Rgba) {
        if x >= self.width || y >= self.height {
            return;
        }

        let index = self.pixel_index(x, y);

        self.data[index + 0] = color.r;
        self.data[index + 1] = color.g;
        self.data[index + 2] = color.b;
        self.data[index + 3] = color.a;
    }

    fn pixel_index(&self, x: usize, y: usize) -> usize {
        (y * self.width + x) * Self::BYTES_PER_PIXEL
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn save_png(&self, path: impl AsRef<Path>) {
        assert!(self.width <= u32::MAX as usize);
        assert!(self.height <= u32::MAX as usize);

        let file = File::create(path).unwrap();
        let writer = BufWriter::new(file);

        let mut encoder = png::Encoder::new(writer, self.width as u32, self.height as u32); // Width is 2 pixels and height is 1.
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(&self.data).unwrap();
    }
}

pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}
