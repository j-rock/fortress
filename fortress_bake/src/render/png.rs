use crate::{
    app::StatusOr,
    file,
};
use png;
use std::{
    fs::File,
    io::BufWriter,
    path::PathBuf
};

pub struct Png {
    img: Vec<u8>,
    width: usize,
    height: usize,
}

impl Png {
    pub fn empty(width: usize, height: usize) -> Png {
        let len = 4 * width * height;

        Png {
            img: vec![0; len],
            width,
            height
        }
    }

    pub fn from_file(path: &PathBuf) -> StatusOr<Png> {
        let mmapped_file = file::util::mmap(path)?;
        let decoder = png::Decoder::new(mmapped_file.bytes());
        let (info, mut reader) = decoder.read_info()
            .map_err(|err| format!("Couldn't read png file: {}", err))?;
        let mut buf = vec![0; info.buffer_size()];
        reader.next_frame(&mut buf)
            .map_err(|err| format!("Couldn't read next frame: {}", err))?;

        Ok(Png {
            img: buf,
            width: info.width as usize,
            height: info.height as usize,
        })
    }

    pub fn save_to_file(&self, path: PathBuf) -> StatusOr<()> {
        let file = File::create(path)
            .map_err(|e| format!("{:?}", e))?;
        let w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::RGBA);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer =
            encoder
                .write_header()
                .map_err(|e| format!("{:?}", e))?;
        writer
            .write_image_data(self.bytes())
            .map_err(|e| format!("{:?}", e))
    }

    pub fn bytes(&self) -> &Vec<u8> {
        &self.img
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
            let other_idx_start = 4 * y * other_width;
            let other_idx_end = other_idx_start + 4 * other_width - 1;
            let self_idx_start = 4 * ((y + top_left_y) * self_width + top_left_x);
            let self_idx_end = self_idx_start + 4 * other_width - 1;
            self.img[self_idx_start..=self_idx_end].copy_from_slice(&other.img[other_idx_start..=other_idx_end]);
        }

        Ok(())
    }
}
