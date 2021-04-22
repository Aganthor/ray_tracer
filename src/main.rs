use geefr_ppm::Ppm;
mod vec3;
use vec3::*;
mod color;
use color::Color;
mod ray;

fn ray_color(r: ray::Ray) -> Color {
    let unit_direction: vec3::Vec3 = unit_vector(r.dir);
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: ray::Point3, radius: f64, r: ray::Ray) -> bool {
    
}

fn main() {
    // Image information
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    // Camera information
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_lenght = 1.0;

    let origin: ray::Point3 = ray::Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_lenght);

    let mut ray_trace = Ppm::new(image_width, image_height);

    println!("Generating ray trace image...");
    for j in (0..image_height).rev() {
        println!("\rScanlines remaining {}...", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width as f64 - 1.0);
            let v = j as f64 / (image_height as f64 - 1.0);
            let r = ray::Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);            

            let pixel_color = ray_color(r);
            let c = color::color_pixel(pixel_color.x(), pixel_color.y(), pixel_color.z());
            ray_trace.set_pixel(i, j, c.x() as u8, c.y() as u8, c.z() as u8);
        }
    }

    ray_trace.write(String::from("ray_trace.ppm")).expect("Failed to save PPM.")
}
