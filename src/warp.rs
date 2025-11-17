use crate::camera::Camera;
use crate::math::Vec3;

pub struct WarpState {
    pub active: bool,
    start: Vec3,
    target: Vec3,
    t: f32,
    duration: f32,
}

impl WarpState {
    pub fn new() -> Self {
        Self {
            active: false,
            start: Vec3::zero(),
            target: Vec3::zero(),
            t: 0.0,
            duration: 1.5,
        }
    }

    pub fn start_animated(&mut self, start: Vec3, target: Vec3, duration: f32) {
        self.active = true;
        self.start = start;
        self.target = target;
        self.t = 0.0;
        self.duration = duration;
    }

    pub fn update(&mut self, dt: f32, camera: &mut Camera) {
        if !self.active {
            return;
        }

        self.t += dt;
        let alpha = (self.t / self.duration).min(1.0);
        camera.position = self.start.lerp(self.target, alpha);

        if alpha >= 1.0 {
            self.active = false;
        }
    }
}
