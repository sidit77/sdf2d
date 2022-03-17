use glam::Vec2;
use image::{GrayImage, Luma};
use sdf2d::{Ops, Sdf, Shapes};

fn main() {
    let sdf = Shapes::circle(0.45)
        .subtract(Shapes::circle(0.25))
        .union(Shapes::rectangle(0.1, 0.25)
            .translate(0.0, -0.5)
            .rotate(f32::to_radians(30.0)));


    let img = rasterize(512, 512, -3.0, sdf);
    img.save("output1.png").expect("Could not write img");

    let f = Vec2::new(512.0, 512.0) * 0.5;
    let underlay = Shapes::hexagon(0.75).rotate(f32::to_radians(90.0));
    let img2 = GrayImage::from_fn(512, 512, |x, y| {
        let p = (Vec2::new(x as f32, y as f32) - f) / f;
        if img.get_pixel(x, y).0[0] > u8::MAX / 2 {
            Luma([255])
        } else if underlay.density(p) < 0.0 {
            Luma([128])
        } else {
            Luma([0])
        }
    });

    img2.save("output2.png").expect("Could not write img");
}

fn rasterize(width: u32, height: u32, factor: f32, sdf: impl Sdf) -> GrayImage {
    let f = Vec2::new(width as f32, height as f32) * 0.5;
    GrayImage::from_fn(width, height, |x, y| {
        let p = (Vec2::new(x as f32, y as f32) - f) / f;
        let d = factor * sdf.density(p);
        let h = u8::MAX as f32 * 0.5;
        Luma([(h + d * h).clamp(u8::MIN as f32, u8::MAX as f32) as u8])
    })
}

