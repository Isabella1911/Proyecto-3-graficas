use std::f32::consts::PI;

use crate::input::InputState;
use crate::math::Vec3;

pub struct Camera {
    pub position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub fov_y: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: Vec3::new(0.0, 30.0, 80.0),
            yaw: 0.0,
            pitch: -0.3,
            fov_y: 60.0_f32.to_radians(),
        }
    }

    pub fn forward(&self) -> Vec3 {
        let cp = self.pitch.cos();
        let sp = self.pitch.sin();
        let cy = self.yaw.cos();
        let sy = self.yaw.sin();

        Vec3::new(sy * cp, sp, -cy * cp).normalized()
    }

    pub fn update(&mut self, dt: f32, input: &InputState) {
        let move_speed = 50.0;
        let rot_speed = 1.5;

        // Rotación
        if input.look_left {
            self.yaw += rot_speed * dt;
        }
        if input.look_right {
            self.yaw -= rot_speed * dt;
        }
        if input.look_up {
            self.pitch += rot_speed * dt;
        }
        if input.look_down {
            self.pitch -= rot_speed * dt;
        }

        // Clamp del pitch
        let max_pitch = 1.3;
        if self.pitch > max_pitch {
            self.pitch = max_pitch;
        }
        if self.pitch < -max_pitch {
            self.pitch = -max_pitch;
        }

        let forward = self.forward();
        let right = Vec3::cross(forward, Vec3::up()).normalized();
        let mut velocity = Vec3::zero();

        // Movimiento
        if input.move_forward {
            velocity = velocity + forward;
        }
        if input.move_back {
            velocity = velocity - forward;
        }
        if input.move_right {
            velocity = velocity + right;
        }
        if input.move_left {
            velocity = velocity - right;
        }
        if input.move_up {
            velocity.y += 1.0;
        }
        if input.move_down {
            velocity.y -= 1.0;
        }

        if velocity.length() > 0.0 {
            velocity = velocity.normalized() * move_speed * dt;
            self.position = self.position + velocity;
        }
    }
}
