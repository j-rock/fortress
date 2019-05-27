use crate::{
    app::StatusOr,
    file,
};
use png;
use std::path::PathBuf;

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
        let buf_reader = file::util::reader(path)?;
        let decoder = png::Decoder::new(buf_reader);
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
            for x in 0..other_width {
                let other_idx = 4 * (y * other_width + x);
                let self_idx = 4 * ((y + top_left_y) * self_width + x + top_left_x);
                for i in 0..4 {
                    self.img[self_idx + i] = other.img[other_idx + i];
                }
            }
        }

        Ok(())
    }
}