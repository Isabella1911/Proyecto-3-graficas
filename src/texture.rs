use image::GenericImageView;

pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u32>,
}

impl Texture {
    pub fn from_file(path: &str) -> Self {
        let img = image::open(path).expect("no se pudo abrir la textura");
        let img = img.to_rgba8();
        let (w, h) = img.dimensions();
        let mut pixels = Vec::with_capacity((w * h) as usize);

        for (_, _, p) in img.enumerate_pixels() {
            let [r, g, b, a] = p.0;
            let c = ((a as u32) << 24)
                | ((r as u32) << 16)
                | ((g as u32) << 8)
                | (b as u32);
            pixels.push(c);
        }

        Self {
            width: w as usize,
            height: h as usize,
            pixels,
        }
    }

    pub fn sample_uv(&self, u: f32, v: f32) -> u32 {
        let mut uu = u.fract();
        let mut vv = v.fract();

        if uu < 0.0 {
            uu += 1.0;
        }
        if vv < 0.0 {
            vv += 1.0;
        }

        let x = (uu * (self.width as f32 - 1.0)).clamp(0.0, self.width as f32 - 1.0) as usize;
        let y = (vv * (self.height as f32 - 1.0)).clamp(0.0, self.height as f32 - 1.0) as usize;

        self.pixels[y * self.width + x]
    }
}