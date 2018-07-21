#[derive(Clone, Copy)]
#[repr(C)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Rgba {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Rgba {
        Rgba {
            r, g, b, a
        }
    }

    fn u8_to_f32(u: u8) -> f32 {
        (u as f32) / 255.0
    }

    pub fn from_bytes(red: u8, green: u8, blue: u8, alpha: u8) -> Rgba {
       Rgba {
           r: Rgba::u8_to_f32(red),
           g: Rgba::u8_to_f32(green),
           b: Rgba::u8_to_f32(blue),
           a: Rgba::u8_to_f32(alpha),
       }
    }

    fn f32_to_u8(f: f32) -> u8 {
        let f255 = f * 255.0;
        if f255 < 0.0 {
            return 0;
        }
        if f255 > 255.0 {
            return 255;
        }
        f255.round() as u8
    }

    pub fn as_byte_vec(&self) -> Vec<u8> {
       vec!(Rgba::f32_to_u8(self.r),
            Rgba::f32_to_u8(self.g),
            Rgba::f32_to_u8(self.b),
            Rgba::f32_to_u8(self.a))
    }
}