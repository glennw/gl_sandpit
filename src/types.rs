#[derive(Debug, Copy, Clone)]
pub struct ColorF {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[allow(dead_code)]
impl ColorF {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> ColorF {
        ColorF {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }

    pub fn alpha(&self, a: f32) -> ColorF {
        ColorF {
            r: self.r,
            g: self.g,
            b: self.b,
            a: a,
        }
    }
}

#[allow(dead_code)]
pub const RED: ColorF = ColorF { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
#[allow(dead_code)]
pub const GREEN: ColorF = ColorF { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
#[allow(dead_code)]
pub const BLUE: ColorF = ColorF { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
#[allow(dead_code)]
pub const YELLOW: ColorF = ColorF { r: 1.0, g: 1.0, b: 0.0, a: 1.0 };
#[allow(dead_code)]
pub const CYAN: ColorF = ColorF { r: 0.0, g: 1.0, b: 1.0, a: 1.0 };
#[allow(dead_code)]
pub const MAGENTA: ColorF = ColorF { r: 1.0, g: 0.0, b: 1.0, a: 1.0 };
#[allow(dead_code)]
pub const BLACK: ColorF = ColorF { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
#[allow(dead_code)]
pub const WHITE: ColorF = ColorF { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
