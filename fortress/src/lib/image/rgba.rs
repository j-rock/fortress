#[derive(Copy, Clone)]
#[repr(C)]
pub struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Rgba {
    pub fn from_bytes(r: u8, g: u8, b: u8, a: u8) -> Rgba {
       Rgba {
           r,
           g,
           b,
           a,
       }
    }

    pub fn as_byte_vec(&self) -> Vec<u8> {
       vec!(self.r,
            self.g,
            self.b,
            self.a)
    }
}