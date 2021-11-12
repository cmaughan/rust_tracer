pub use glam::Vec3;
pub use std::sync::Arc;
pub use std::clone::Clone;

//pub type Point3 = glam::Vec3;
pub type Rect = glam::UVec4;
pub type Color = glam::Vec3;
pub type PackedColor = u32;

pub use rand::Rng;
pub use rand_xoshiro::rand_core::SeedableRng;
pub use rand_xoshiro::Xoshiro128Plus;

pub struct FastRepRand {
    rng: Xoshiro128Plus,
}

impl FastRepRand {
    pub fn new(seed: u64) -> FastRepRand {
        FastRepRand {
            rng: Xoshiro128Plus::seed_from_u64(seed),
        }
    }

    pub fn gen_range(&mut self, range: std::ops::Range<f32>) -> f32 {
        self.rng.gen_range(range)
    }
}

pub fn rect_width(rect : &Rect) -> u32
{
    return rect.z - rect.x;
}

pub fn rect_height(rect : &Rect) -> u32
{
    return rect.w - rect.y;
}

pub fn index_from_xy(image_width: u32, _image_height: u32, x: u32, y: u32) -> usize {
    (y * image_width + x) as usize
}

pub fn packed_color_from_u8_rgb(r: u8, g: u8, b: u8) -> PackedColor {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

pub fn packed_color_from_f32_rgb(r: f32, g: f32, b: f32) -> PackedColor {
    packed_color_from_u8_rgb((255.0 * r) as u8, (255.0 * g) as u8, (255.0 * b) as u8)
}

pub fn packed_color_from_color(c: Color) -> PackedColor {
    let gamma = 1.0 / 2.2;
    let col_gamma = Color::new(c.x.powf(gamma), c.y.powf(gamma), c.z.powf(gamma));
    packed_color_from_f32_rgb(col_gamma.x, col_gamma.y, col_gamma.z)
}

pub fn color_random(rng: &mut FastRepRand) -> Color {
    color_random_range(rng, 0.0..1.0)
}

pub fn color_random_range(rng: &mut FastRepRand, range: std::ops::Range<f32>) -> Color {
    Color::new(
        rng.gen_range(range.clone()),
        rng.gen_range(range.clone()),
        rng.gen_range(range),
    )
}
