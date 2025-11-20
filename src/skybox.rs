use crate::camera::Camera;
use crate::math::Vec3;
use crate::renderer::Renderer;
use crate::texture::Texture;
use std::f32::consts::PI;

pub fn draw_skybox(renderer: &mut Renderer, camera: &Camera, skybox_texture: &Texture) {
    let w = renderer.width as i32;
    let h = renderer.height as i32;

    for y in 0..h {
        for x in 0..w {
            let screen_x = (x as f32 / w as f32) * 2.0 - 1.0;
            let screen_y = 1.0 - (y as f32 / h as f32) * 2.0;

            let aspect = w as f32 / h as f32;

            let tan_half_fov = (camera.fov_y * 0.5).tan();
            let ray_camera_x = screen_x * aspect * tan_half_fov;
            let ray_camera_y = screen_y * tan_half_fov;
            let ray_camera_z = -1.0;

            let ray_dir_camera = Vec3::new(ray_camera_x, ray_camera_y, ray_camera_z).normalized();

            let ray_dir_world = transform_ray_to_world(ray_dir_camera, camera);

            let theta = ray_dir_world.z.atan2(ray_dir_world.x);
            let phi = ray_dir_world.y.asin();

            let u = (theta + PI) / (2.0 * PI);
            let v = (phi + PI / 2.0) / PI;

            let color = sample_texture(skybox_texture, u, v);

            renderer.put_pixel(x, y, color);
        }
    }
}

fn transform_ray_to_world(ray_camera: Vec3, camera: &Camera) -> Vec3 {
    let forward = camera.forward();

    // cross como función asociada
    let right = Vec3::cross(forward, Vec3::up()).normalized();
    let up = Vec3::cross(right, forward).normalized();

    right * ray_camera.x + up * ray_camera.y - forward * ray_camera.z
}

fn sample_texture(texture: &Texture, u: f32, v: f32) -> u32 {
    let u = u.clamp(0.0, 1.0);
    let v = v.clamp(0.0, 1.0);

    let x = (u * (texture.width - 1) as f32) as usize;
    let y = (v * (texture.height - 1) as f32) as usize;

    texture.pixels[y * texture.width + x]
}
