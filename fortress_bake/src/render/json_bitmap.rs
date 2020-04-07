use crate::{
    app::StatusOr,
    file,
};
use font_atlas;
use serde_json;
use std::{
    fs::File,
    io::Write,
    path::PathBuf
};

#[derive(Serialize, Deserialize)]
pub struct JsonBitmap {
    img: Vec<u8>,
    width: usize,
    height: usize,
}

impl JsonBitmap {
    pub fn empty(width: usize, height: usize) -> Self {
        JsonBitmap {
            img: vec![0; width * height],
            width,
            height
        }
    }

    pub fn from(bitmap: font_atlas::rasterize::Bitmap) -> Self {
        let (width, height) = (bitmap.width(), bitmap.height());
        JsonBitmap {
            img: bitmap.into_raw(),
            width,
            height,
        }
    }

    pub fn from_file(path: &PathBuf) -> StatusOr<Self> {
        let mmapped_file = file::util::mmap(path)?;
        Self::from_slice(mmapped_file.bytes())
    }

    pub fn from_slice(slice: &[u8]) -> StatusOr<Self> {
        serde_json::de::from_slice(slice)
            .map_err(|e| format!("Bitmap read err: {:?}", e))
    }

    pub fn image_bytes(&self) -> &[u8] {
        self.img.as_slice()
    }

    pub fn save_to_file(&self, path: PathBuf) -> StatusOr<()> {
        let out = serde_json::to_string(&self)
            .map_err(|e| format!("Bitmap serialize err: {:?}", e))?;

        let mut file = File::create(path)
            .map_err(|e| format!("Err opening bitmap outfile {:?}", e))?;
        file.write_all(out.as_bytes())
            .map_err(|e| format!("Err writing bitmap: {:?}", e))
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn overwrite(&mut self, other: JsonBitmap, top_left_x: usize, top_left_y: usize) -> StatusOr<()> {
        let (self_width, self_height) = self.size();
        let (other_width, other_height) = other.size();

        if self_width < other_width {
            return Err(format!("Cannot overwrite Bitmap with wider image."));
        }
        if self_height < other_height {
            return Err(format!("Cannot overwrite Bitmap with taller image."));
        }

        for y in 0..other_height {
            let other_idx_start = y * other_width;
            let other_idx_end = other_idx_start + other_width - 1;
            let self_idx_start = (y + top_left_y) * self_width + top_left_x;
            let self_idx_end = self_idx_start + other_width - 1;
            self.img[self_idx_start..=self_idx_end].copy_from_slice(&other.img[other_idx_start..=other_idx_end]);
        }

        Ok(())
    }
}
