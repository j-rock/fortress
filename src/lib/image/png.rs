use crate::{
    app::StatusOr,
    image::Rgba,
};
use lodepng;
use std::path::PathBuf;

pub struct Png {
    img: Vec<Rgba>,
    width: usize,
    height: usize,
}

impl Png {
    pub fn empty(width: usize, height: usize) -> Png {
        let empty = Rgba::new(0.0, 0.0, 0.0, 0.0);
        let len = width * height;

        Png {
            img: vec![empty; len],
            width,
            height
        }
    }

    pub fn from_file(path: &PathBuf) -> StatusOr<Png> {
        let bitmap = lodepng::decode32_file(path)
            .map_err(|err| format!("Failed to open PNG path {:?}: {}", path, err))?;

        let bitmap_len = bitmap.width * bitmap.height;
        let mut img = Vec::with_capacity(bitmap_len);
        for idx in 0..bitmap_len {
            let rgba = bitmap.buffer[idx];
            img.push(Rgba::from_bytes(rgba.r, rgba.g, rgba.b, rgba.a));
        }
        Ok(Png {
            img,
            width: bitmap.width,
            height: bitmap.height,
        })
    }

    pub fn flattened_copy_bytes(&self) -> Vec<u8> {
        let (width, height) = self.size();
        let mut out = Vec::with_capacity(4 * width * height);
        for pixel in self.img.iter() {
            let mut v = pixel.as_byte_vec();
            out.append(&mut v);
        }
        out
    }

    pub fn save(&self, path: &PathBuf) -> StatusOr<()> {
        let (width, height) = self.size();
        let bytes = self.flattened_copy_bytes();
        lodepng::encode32_file(path, bytes.as_slice(), width, height)
            .map_err(|err| format!("Problem saving png to {:?}: {}", path, err))
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn overwrite(&mut self, other: Png, top_left_x: usize, top_left_y: usize) -> StatusOr<()> {
        let (self_width, self_height) = self.size();
        let (other_width, other_height) = other.size();

        if self_width < other_width {
            return Err(format!("Cannot overwrite PNG with wider image."));
        }
        if self_height < other_height {
            return Err(format!("Cannot overwrite PNG with taller image."));
        }

        for y in 0..other_height {
            for x in 0..other_width {
                let other_idx = y * other_width + x;
                let self_idx = (y + top_left_y) * self_width + x + top_left_x;
                self.img[self_idx] = other.img[other_idx];
            }
        }

        Ok(())
    }
}