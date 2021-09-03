#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn to_hsl(&self) -> Hsl {
        Hsl {
            h: self.r as f32 / 255_f32,
            s: self.g as f32 / 255_f32,
            l: self.b as f32 / 255_f32,
        }
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct Hsl {
    h: f32,
    s: f32,
    l: f32,
}

impl Hsl {
    pub fn to_rgb(&self) -> Rgb {
        Rgb {
            r: self.h as u8 * 255,
            g: self.s as u8 * 255,
            b: self.l as u8 * 255,
        }
    }
}
