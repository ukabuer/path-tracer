extern crate image;

mod vec3;
mod ray;
mod hitable;

use image::RgbImage;
use vec3::Vec3;
use ray::Ray;
use hitable::Hitable;

fn color(r: &ray::Ray, world: &Hitable) -> vec3::Vec3 {
    let mut rec = hitable::HitRecord::new();
    if world.hit(r, 0.0, 9999.9, &mut rec) {
        return Vec3::new(rec.normal.x() + 1.0, rec.normal.y() + 1.0, rec.normal.z() + 1.0) * 0.5;
    } else {
        let unit_direction = Vec3::unit_vector(r.direction());
        let t = (unit_direction.y() + 1.0) * 0.5;
        return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    let mut img = RgbImage::new(nx, ny);
    let lower_left_corner = Vec3::new(-2., -1., -1.);
    let horizaontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0., 0., 0.);
    let mut list = hitable::HitableList::new();
    list.push(Box::new(hitable::Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    }));
    list.push(Box::new(hitable::Sphere {
        center: Vec3::new(0.0, 0.-100.5, -1.0),
        radius: 100.0,
    }));
    for j in 0..ny {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new(origin, lower_left_corner + horizaontal * u + vertical * v);
            let col = color(&r, &list);
            let pixel = &mut img[(i, ny - j - 1)];
            pixel[0] = (255.99 * col[0]) as u8;
            pixel[1] = (255.99 * col[1]) as u8;
            pixel[2] = (255.99 * col[2]) as u8;
        }
    }
    img.save("output.png").expect("can not save image file");
}
