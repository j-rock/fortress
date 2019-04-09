use crate::{
    app::StatusOr,
    color::Rgba,
    dimensions::Pixels
};
use lodepng;
use std::path::PathBuf;

pub struct Png {
    pub img: Vec<Vec<Rgba>>,
}

impl Png {
    pub fn from_file(path: &str) -> StatusOr<Png> {
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

    pub fn save(&self, path: &PathBuf) -> StatusOr<()> {
        let (width, height) = self.size();
        let flat_rgba = self.flattened_copy();
        lodepng::encode32_file(path, flat_rgba.as_slice(), width, height)
            .map_err(|err| format!("Problem saving png to {:?}: {}", path, err))
    }

    pub fn size(&self) -> (Pixels, Pixels) {
        (self.img[0].len(), self.img.len())
    }
}