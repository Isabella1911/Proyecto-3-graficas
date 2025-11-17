use crate::renderer::Renderer;

/// Dibuja un fondo con gradiente + estrellas pseudo-aleatorias
pub fn draw_skybox(renderer: &mut Renderer) {
    let w = renderer.width as i32;
    let h = renderer.height as i32;

    // 1) Gradiente de fondo (de negro a azul oscuro/gris)
    for y in 0..h {
        let t = y as f32 / h as f32; // 0 arriba, 1 abajo

        let r = (5.0 * (1.0 - t)) as u8;
        let g = (8.0 * (1.0 - t)) as u8;
        let b = (20.0 + 30.0 * (1.0 - t)) as u8;

        let base_color = 0xFF000000
            | ((r as u32) << 16)
            | ((g as u32) << 8)
            | (b as u32);

        for x in 0..w {
            renderer.put_pixel(x, y, base_color);
        }
    }

    // 2) Estrellas pseudo-aleatorias con distintos brillos
    for y in 0..h {
        for x in 0..w {
            // Convertir a u32 y usar wrapping_mul para evitar overflow
            let ux = x as u32;
            let uy = y as u32;

            let v = (ux.wrapping_mul(73_856_093) ^ uy.wrapping_mul(19_349_663)) & 0xFF;

            let star_color = if v < 2 {
                Some(0xFF202020) // tenue
            } else if v == 3 {
                Some(0xFF606060) // media
            } else if v == 4 {
                Some(0xFFFFFFFF) // brillante
            } else {
                None
            };

            if let Some(c) = star_color {
                renderer.put_pixel(x, y, c);
            }
        }
    }
}
