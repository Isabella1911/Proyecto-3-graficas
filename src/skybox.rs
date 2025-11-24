use crate::camera::Camera;
use crate::renderer::Renderer;
use crate::texture::Texture;

pub fn draw_skybox(renderer: &mut Renderer, _camera: &Camera, tex: &Texture) {
    let width = renderer.width as i32;
    let height = renderer.height as i32;

    for y in 0..height {
        let v = y as f32 / (height - 1) as f32;

        for x in 0..width {
            let u = x as f32 / (width - 1) as f32;

            let color = tex.sample_uv(u, v);
            renderer.put_pixel(x, y, color);
        }
    }
}