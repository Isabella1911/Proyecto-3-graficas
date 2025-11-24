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
                    let x = cx + dx;
                    let y = cy + dy;
                    self.fb.put_pixel(x, y, color);
                }
            }
        }
    }

    pub fn circle(&mut self, center: (i32, i32), radius: i32, color: u32) {
        let (cx, cy) = center;
        let r2 = radius * radius;
        let inner = (radius - 1).max(0);
        let inner2 = inner * inner;

        for dy in -radius..=radius {
            for dx in -radius..=radius {
                let d2 = dx * dx + dy * dy;
                if d2 <= r2 && d2 >= inner2 {
                    let x = cx + dx;
                    let y = cy + dy;
                    self.fb.put_pixel(x, y, color);
                }
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
        let (x0, y0) = p0;
        let (x1, y1) = p1;
        let (x2, y2) = p2;

        let min_x = x0.min(x1).min(x2).max(0);
        let max_x = x0.max(x1).max(x2).min(self.fb.width as i32 - 1);
        let min_y = y0.min(y1).min(y2).max(0);
        let max_y = y0.max(y1).max(y2).min(self.fb.height as i32 - 1);

        if min_x > max_x || min_y > max_y {
            return;
        }

        let edge = |ax: i32, ay: i32, bx: i32, by: i32, px: i32, py: i32| {
            (px - ax) * (by - ay) - (py - ay) * (bx - ax)
        };

        let area = edge(x0, y0, x1, y1, x2, y2);
        if area == 0 {
            return;
        }

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let w0 = edge(x1, y1, x2, y2, x, y);
                let w1 = edge(x2, y2, x0, y0, x, y);
                let w2 = edge(x0, y0, x1, y1, x, y);

                if area > 0 {
                    if w0 >= 0 && w1 >= 0 && w2 >= 0 {
                        self.fb.put_pixel(x, y, color);
                    }
                } else {
                    if w0 <= 0 && w1 <= 0 && w2 <= 0 {
                        self.fb.put_pixel(x, y, color);
                    }
                }
            }
        }
    }
}
