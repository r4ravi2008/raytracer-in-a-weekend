mod ray;
mod vec3;
mod hittable;
mod hittableList;
mod hitRecord;
mod sphere;

use ray::Ray;
use vec3::Vec3;
use hittableList::HittableSpheres;
use sphere::Sphere;

fn color(r: &Ray, world: &HittableSpheres) -> Vec3 {
    match world.hit(r, 0.0, std::f64::MAX) {
        Some(hitRecord) =>
            0.5 * Vec3 {
                x: hitRecord.normal.x + 1.0,
                y: hitRecord.normal.y + 1.0,
                z: hitRecord.normal.z + 1.0,
            },
        None => {
            let unitDirection = r.direction().unitVector();
            let t = 0.5 * unitDirection.y + 1.0;
            (1.0 - t) * Vec3{
                x: 1.0,
                y: 1.0,
                z: 1.0
            } + t * Vec3 {
                x: 0.5,
                y: 0.7,
                z: 1.0
            }
        }
    }
}

fn main() {
    let nx: i32 = 800;
    let ny: i32 = 400;
    println!("P3\n{} {}\n255", nx, ny);

    let lowerLeftCorner = Vec3 {
        x: -2.0,
        y: -1.0,
        z: -1.0,
    };
    let horizontal = Vec3 {
        x: 4.0,
        y: 0.0,
        z: 0.0,
    };
    let origin = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vec3 {
        x: 0.0,
        y: 2.0,
        z: 0.0,
    };

    let hittable = HittableSpheres {
        spheres: vec![
            Sphere{
                center: Vec3{x:0.0, y: 0.0, z: -2.0},
                radius: 0.5
            },
            Sphere{
                center: Vec3{x:1.5, y: 0.0, z: -2.0},
                radius: 0.2
            },
            Sphere{
                center: Vec3{x:-1.4, y: -0.1, z: -4.0},
                radius: 0.4
            },
            Sphere{
                center: Vec3{x:0.0, y: -100.5, z: -1.0},
                radius: 100.0
            }
        ]
    };

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f64 / nx as f64;
            let v = j as f64 / ny as f64;

            let r = Ray {
                a: origin,
                b: lowerLeftCorner + u * horizontal + v * vertical,
            };

            let p = r.pointAtParameter(2.0);
            let col = color(&r, &hittable);
            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;

            println!("{} {} {}\n", ir, ig, ib);
        }
    }
}
