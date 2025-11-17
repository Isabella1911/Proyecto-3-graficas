use super::framebuffer::FrameBuffer;

pub struct Draw2D<'a> {
    fb: &'a mut FrameBuffer,
}

impl<'a> Draw2D<'a> {
    pub fn new(fb: &'a mut FrameBuffer) -> Self {
        Self { fb }
    }

    pub fn filled_circle(&mut self, center: (i32, i32), radius: i32, color: u32) {
        let (cx, cy) = center;
        let r2 = radius * radius;

        for dy in -radius..=radius {
            for dx in -radius..=radius {
                if dx * dx + dy * dy <= r2 {
                    self.fb.put_pixel(cx + dx, cy + dy, color);
                }
            }
        }
    }

    pub fn circle(&mut self, center: (i32, i32), radius: i32, color: u32) {
        let (cx, cy) = center;
        let mut x = radius;
        let mut y = 0;
        let mut err = 0;

        while x >= y {
            self.fb.put_pixel(cx + x, cy + y, color);
            self.fb.put_pixel(cx + y, cy + x, color);
            self.fb.put_pixel(cx - y, cy + x, color);
            self.fb.put_pixel(cx - x, cy + y, color);
            self.fb.put_pixel(cx - x, cy - y, color);
            self.fb.put_pixel(cx - y, cy - x, color);
            self.fb.put_pixel(cx + y, cy - x, color);
            self.fb.put_pixel(cx + x, cy - y, color);

            y += 1;
            if err <= 0 {
                err += 2 * y + 1;
            } else {
                x -= 1;
                err -= 2 * x + 1;
            }
        }
    }

    pub fn line(&mut self, p0: (i32, i32), p1: (i32, i32), color: u32) {
        let (mut x0, mut y0) = p0;
        let (x1, y1) = p1;

        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            self.fb.put_pixel(x0, y0, color);

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    pub fn triangle(&mut self, p0: (i32, i32), p1: (i32, i32), p2: (i32, i32), color: u32) {
        self.line(p0, p1, color);
        self.line(p1, p2, color);
        self.line(p2, p0, color);
    }
}
