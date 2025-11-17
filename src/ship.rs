use crate::camera::Camera;
use crate::math::Vec3;
use crate::renderer::Renderer;

pub struct Ship {
    pub position: Vec3,
}

impl Ship {
    pub fn new() -> Self {
        Ship {
            position: Vec3::zero(),
        }
    }

    pub fn update(&mut self, _dt: f32, camera: &Camera) {
        // La nave se coloca un poco delante de la cámara
        let forward = camera.forward();
        self.position = camera.position + forward * 15.0;
    }

    pub fn render(&self, renderer: &mut Renderer, camera: &Camera) {
        if let Some((cx, cy)) = renderer.project_point(self.position, camera) {
            let size = 12;

            // Cuerpo principal (blanco)
            let nose = (cx, cy - size);                // punta
            let left_bottom = (cx - size / 2, cy + size / 2);
            let right_bottom = (cx + size / 2, cy + size / 2);
            renderer.draw_triangle(nose, left_bottom, right_bottom, 0xFFFFFFFF);

            // Aleta izquierda (gris clara)
            let fin_left_top = (cx - size / 2, cy + size / 4);
            let fin_left_bottom = (cx - size, cy + size);
            renderer.draw_triangle(
                fin_left_top,
                fin_left_bottom,
                left_bottom,
                0xFFAAAAAA,
            );

            // Aleta derecha (gris clara)
            let fin_right_top = (cx + size / 2, cy + size / 4);
            let fin_right_bottom = (cx + size, cy + size);
            renderer.draw_triangle(
                fin_right_top,
                fin_right_bottom,
                right_bottom,
                0xFFAAAAAA,
            );

            // “Fuego” del motor (naranja)
            let flame_top = (cx, cy + size / 2);
            let flame_left = (cx - size / 4, cy + size + 4);
            let flame_right = (cx + size / 4, cy + size + 4);
            renderer.draw_triangle(flame_top, flame_left, flame_right, 0xFFFF9933);
        }
    }
}
