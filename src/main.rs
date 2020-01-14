mod vec3;
mod ray;

use vec3::Vec3;
use ray::Ray;

fn hitSphere(center: Vec3, radius: f32, ray: Ray) -> bool {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * oc.dot(ray.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant > 0.0 {
        true
    } else {
        false
    }
}

fn color(r: Ray) -> Vec3 {
    let center = Vec3 {
        x: 0.0,
        y: 0.0,
        z: -1.0
    };

    if(hitSphere(center, 0.5, r)) {
        Vec3 {x: 1.0, y: 0.0, z: 0.0 }
    } else {
        let unitDirection = r.direction().unitVector();
        let t = 0.5 * (unitDirection.y + 1.0);
        (1.0 - t ) * Vec3 {x:1.0, y:1.0, z:1.0} + t * Vec3{x:0.5, y:0.7, z:1.0}
    }
}

fn main() {
    let nx: i32 = 800;
    let ny: i32 = 400;
    println!("P3\n{} {}\n255", nx, ny);

    let lowerLeftCorner = Vec3{x: -2.0, y: -1.0, z: -1.0};
    let horizontal = Vec3{x: 4.0, y: 0.0, z: 0.0};
    let origin = Vec3{x: 0.0, y: 0.0, z: 0.0};
    let vertical = Vec3{x: 0.0, y: 2.0, z: 0.0};
   
    for j in (0..ny).rev() {
        for i in 0..nx {

            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            let r = Ray {
                A: origin,
                B: lowerLeftCorner + u * horizontal + v * vertical
            };

            let col = color(r);

            let r = ray::Ray {A: origin, B: lowerLeftCorner + u * horizontal + v * vertical };
            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;

            println!("{} {} {}\n", ir, ig, ib);

        }
    }
}
