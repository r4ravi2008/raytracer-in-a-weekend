extern crate rand;
mod ray;
mod vec3;
mod hittable;
mod hittableList;
mod hitRecord;
mod sphere;
mod camera;

use ray::Ray;
use vec3::Vec3;
use hittableList::HittableSpheres;
use sphere::Sphere;
use hittable::Hittable;
use rand::Rng;
use crate::camera::Camera;

fn random() -> f64 {
    rand::thread_rng().gen::<f64>()
}

fn randomInUnitSphere() -> Vec3 {
    let mut p: Vec3 = Vec3::zero();
    let mut done = false;
    while !done{
        p = 2.0 * Vec3::new(
            random(),
            random(),
            random()
        ) - Vec3::new(1.0, 1.0, 1.0);
        let squaredLength = p.squaredLength();
//        println!("squaredLength is: {}", squaredLength);
        if squaredLength <= 1.0 {
            done = true;
        }
    }
    p
}

fn color(r: &Ray, world: &HittableSpheres) -> Vec3 {
//    print!("Calculating color for ray: {:?}", r);
//    let mut b = randomInUnitSphere();
//    println!("b is {:?}", b);
    match world.hit(r, 0.0, std::f64::MAX) {
        Some(hitRecord) => {
            let target = hitRecord.p + hitRecord.normal + randomInUnitSphere();
            0.5 * color(
                &Ray {
                    a: hitRecord.p,
                    b: target - hitRecord.p
                }, world
            )
        }


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
    let ns: i32 = 10;
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
                center: Vec3{x:0.0, y: 0.0, z: -1.0},
                radius: 0.5
            },
            Sphere{
                center: Vec3{x:0.0, y: -100.5, z: -1.0},
                radius: 100.0
            }
        ]
    };

    let cam = Camera::new();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new (0.0, 0.0, 0.0);
            for s in 0..ns {
                let u = (i as f64 + random()) / nx as f64;
                let v = (j as f64 + random()) / ny as f64;
//                println!("{}s {}j {}i th iteration for camera", s, j, i);
                let r = cam.getRay(u, v);
                col = col + color(&r, &hittable)
            }
            col = col/ ns as f64;

            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;

            println!("{} {} {}\n", ir, ig, ib);
        }
    }
}
