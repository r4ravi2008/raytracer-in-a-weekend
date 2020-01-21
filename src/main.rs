extern crate rand;
mod ray;
mod vec3;
mod hittable;
mod hittableList;
mod hitRecord;
mod sphere;
mod camera;
mod material;
mod lambertian;
mod metal;

use ray::Ray;
use vec3::Vec3;
use hittableList::HittableSpheres;
use sphere::Sphere;
use hittable::Hittable;
use rand::Rng;
use camera::Camera;
use lambertian::Lambertian;
use material::Material;
use material::Scatter;
use metal::Metal;

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
        if squaredLength <= 1.0 {
            done = true;
        }
    }
    p
}

fn color(r: &Ray, world: &HittableSpheres, depth: i32) -> Vec3 {
    match world.hit(r, 0.0001, std::f64::MAX) {

        Some(hitRecord) => {
            let optional = hitRecord.material.scatter(r, &hitRecord);
            if(depth < 50 && optional.is_some()) {
                let (attenuation, scattered) = optional.unwrap();
                return attenuation * color(&scattered, world, depth + 1)
            } else {
                return Vec3::new(0.0, 0.0, 0.0)
            }
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
                radius: 0.5,
                material: Material::Lambertian(Lambertian::new(Vec3::new(0.8, 0.3, 0.3)))
            },
            Sphere{
                center: Vec3{x:0.0, y: -100.5, z: -1.0},
                radius: 100.0,
                material: Material::Lambertian(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)))
            },
            Sphere{
                center: Vec3{x: 1.0, y: 0.0 , z: -1.0},
                radius: 000.5,
                material: Material::Metal(Metal::new(Vec3::new(0.8, 0.2, 0.2)))
            },
            Sphere{
                center: Vec3{x: -1.0, y: 000.0, z: -1.0},
                radius: 000.5,
                material: Material::Metal(Metal::new(Vec3::new(0.8, 0.8, 0.8)))
            },
        ]
    };

    let cam = Camera::new();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new (0.0, 0.0, 0.0);
            for s in 0..ns {
                let u = (i as f64 + random()) / nx as f64;
                let v = (j as f64 + random()) / ny as f64;
                let r = cam.getRay(u, v);
                col = col + color(&r, &hittable, 0)
            }
            col = col/ ns as f64;
            col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;

            println!("{} {} {}\n", ir, ig, ib);
        }
    }
}
