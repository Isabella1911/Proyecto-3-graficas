mod renderer;
mod world;
mod camera;
mod texture;
mod skybox;
mod math;
mod input;

use renderer::Renderer;
use world::SolarSystem;
use camera::Camera;
use texture::Texture;

use image::{ImageBuffer, Rgba};

fn main() {
    let width: usize = 960;
    let height: usize = 540;

    let mut renderer = Renderer::new(width, height);
    let mut system = SolarSystem::new_demo();
    let camera = Camera::new();

    let textura_sol = Texture::from_file("assets/textures/sun.jpg");
    let textura_planeta1 = Texture::from_file("assets/textures/mercury.jpg");
    let textura_planeta2 = Texture::from_file("assets/textures/venus.jpg");
    let textura_planeta3 = Texture::from_file("assets/textures/earth.jpg");
    let textura_planeta4 = Texture::from_file("assets/textures/moon.jpg");
    let textura_luna = Texture::from_file("assets/textures/mars.jpg");
    let textura_cielo = Texture::from_file("assets/textures/stars.jpg");

    renderer.clear(0xFF000000);

    skybox::draw_skybox(&mut renderer, &camera, &textura_cielo);

    system.update(0.0);

    let mut body_indices: Vec<(usize, f32)> = Vec::new();

    for i in 0..system.bodies.len() {
        let body_pos = system.body_position(i);
        let distance = (body_pos - camera.position).length();
        body_indices.push((i, distance));
    }

    body_indices.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    for (i, _) in body_indices {
        if let Some(((sx, sy), radius_px)) =
            system.project_body(i, &renderer, &camera)
        {
            let tex = match i {
                0 => &textura_sol,
                1 => &textura_planeta1,
                2 => &textura_planeta2,
                3 => &textura_planeta3,
                4 => &textura_planeta4,
                5 => &textura_luna,
                _ => continue,
            };

            let rotation = system.bodies[i].angle;

            renderer.draw_textured_sphere(tex, (sx, sy), radius_px, rotation);
        }
    }

    let buf = renderer.buffer();

    let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::new(width as u32, height as u32);

    for y in 0..height {
        for x in 0..width {
            let c = buf[y * width + x];
            let a = ((c >> 24) & 0xFF) as u8;
            let r = ((c >> 16) & 0xFF) as u8;
            let g = ((c >> 8) & 0xFF) as u8;
            let b = (c & 0xFF) as u8;

            img.put_pixel(x as u32, y as u32, Rgba([r, g, b, a]));
        }
    }

    img.save("solar_frame.png")
        .expect("No se pudo guardar solar_frame.png");

    println!("Listo: solar_frame.png generado");
}
