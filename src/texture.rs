use image::GenericImageView;

pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u32>,
}

impl Texture {
    pub fn from_file(path: &str) -> Self {
        let img = image::open(path)
            .unwrap_or_else(|e| panic!("No se pudo cargar la textura {}: {}", path, e));

        let rgba = img.to_rgba8();
        let (w, h) = rgba.dimensions();

        let mut pixels = Vec::with_capacity((w * h) as usize);

        for p in rgba.pixels() {
            let [r, g, b, a] = p.0;
            let argb = ((a as u32) << 24)
                | ((r as u32) << 16)
                | ((g as u32) << 8)
                | (b as u32);
            pixels.push(argb);
        }

        Self {
            width: w as usize,
            height: h as usize,
            pixels,
        }
    }
}
