use minifb::{Key, Window, WindowOptions};

fn main() {
    let width: usize = 640;
    let height: usize = 480;

    let mut window = Window::new(
        "Test Minifb Fondo Verde",
        width,
        height,
        WindowOptions::default(),
    )
    .expect("No se pudo crear la ventana");

    // Fondo verde fosforescente
    window.set_background_color(0, 255, 0);

    // Bucle principal
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Solo actualizamos eventos y repintado del fondo
        window.update();
    }
}
