use color::Rgba;
use dimensions::Pixels;
use lodepng;
use std;

pub struct ImageRect {
    top_left_row: Pixels,
    top_left_column: Pixels,
    width: Pixels,
    height: Pixels,
}

impl ImageRect {
    pub fn new(top_left_row: Pixels, top_left_column: Pixels, width: Pixels, height: Pixels) -> ImageRect {
        ImageRect {
            top_left_row,
            top_left_column,
            width,
            height
        }
    }
}

pub struct Png {
    pub img: Vec<Vec<Rgba>>,
}

impl Png {
    pub fn new(width: Pixels, height: Pixels, color: &Rgba) -> Png {
        let mut img = Vec::with_capacity(height);
        let row: Vec<Rgba> = vec![color.clone(); width];
        for _ in 0..height {
            img.push(row.clone());
        }
        Png {
            img
        }
    }

    pub fn from_file(path: &str) -> std::result::Result<Png, String> {
        let bitmap = lodepng::decode32_file(path)
            .map_err(|err| format!("Failed to open PNG path {}: {}", path, err))?;

        let mut img = Vec::with_capacity(bitmap.height);
        for i in 0..bitmap.height {
            let mut row = Vec::with_capacity(bitmap.width);
            for j in 0..bitmap.width {
                let idx = i * bitmap.width + j;
                let rgba = bitmap.buffer[idx];
                row.push(Rgba::from_bytes(rgba.r, rgba.g, rgba.b, rgba.a));
            }
            img.push(row);
        }
        Ok(Png { img })
    }

    pub fn copy_sub_image(&self, irect: ImageRect) -> Png {
        let black = Rgba::new(0.0, 0.0, 0.0, 1.0);
        let mut out = Png::new(irect.width, irect.height, &black);
        for r in irect.top_left_row..(irect.top_left_row + irect.height) {
            for c in irect.top_left_column..(irect.top_left_column + irect.width) {
                out.img[r - irect.top_left_row][c - irect.top_left_column] = self.img[r][c];
            }
        }
        out
    }

    pub fn flattened_copy(&self) -> Vec<Rgba> {
        let (width, height) = self.size();
        let mut out: Vec<Rgba> = Vec::with_capacity(width * height);
        for image_row in self.img.iter() {
            for pixel in image_row.iter() {
                out.push(*pixel);
            }
        }
        out
    }

    pub fn flattened_copy_bytes(&self) -> Vec<u8> {
        let (width, height) = self.size();
        let mut out = Vec::with_capacity(4 * width * height);
        for image_row in self.img.iter() {
            for pixel in image_row.iter() {
                let mut v = pixel.as_byte_vec();
                out.append(&mut v);
            }
        }
        out
    }

    pub fn save(&self, path: &'static str) -> std::result::Result<(), String> {
        let (width, height) = self.size();
        let flat_rgba = self.flattened_copy();
        lodepng::encode32_file(path, flat_rgba.as_slice(), width, height)
            .map_err(|err| format!("Problem saving png to {}: {}", path, err))
    }

    pub fn size(&self) -> (Pixels, Pixels) {
        (self.img[0].len(), self.img.len())
    }
}