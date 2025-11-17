use crate::renderer::Renderer;

pub fn draw_skybox(renderer: &mut Renderer) {
    let w = renderer.width as i32;
    let h = renderer.height as i32;
    let step_x = 40;
    let step_y = 30;

    for y in (0..h).step_by(step_y as usize) {
        for x in (0..w).step_by(step_x as usize) {
            let jx = x + ((y * 13) % 17);
            let jy = y + ((x * 7) % 11);

            renderer.put_pixel(jx, jy, 0xFFFFFFFF);
        }
    }
}
