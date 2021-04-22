use super::vec3;

pub type Color = vec3::Vec3;

pub fn color_pixel(r: f64, g: f64, b: f64) -> Color {
    Color {
        e: [255.999 * r, 255.999 * g, 255.999 * b]
    }
}