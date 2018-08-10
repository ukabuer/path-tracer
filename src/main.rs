extern crate image;

use image::RgbImage;

fn main() {
    let nx = 200;
    let ny = 100;
    let mut img = RgbImage::new(nx, ny);
    for j in 0..ny {
        for i in 0..nx {
            let r: f32 = i as f32 / nx as f32;
            let g: f32 = j as f32 / ny as f32;
            let b = 0.2;
            let pixel = &mut img[(i, j)];
            pixel[0] = (255.99 * r) as u8;
            pixel[1] = (255.99 * g) as u8;
            pixel[2] = (255.99 * b) as u8;
        }
    }
    img.save("output.png").expect("can not save image file");
}
