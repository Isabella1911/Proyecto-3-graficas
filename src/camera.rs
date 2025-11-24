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

    
    pub fn basis(&self) -> (Vec3, Vec3, Vec3) {
        let forward = self.forward();
        let right = Vec3::cross(forward, Vec3::up()).normalized();
        let up = Vec3::cross(right, forward).normalized();
        (right, up, forward)
    }

    
    pub fn world_to_camera(&self, world: Vec3) -> Vec3 {
        let (right, up, forward) = self.basis();
        let v = world - self.position;

        
        let x = v.dot(right);
        let y = v.dot(up);
        let z = -v.dot(forward);

        Vec3::new(x, y, z)
    }

    
    pub fn project_to_ndc(&self, world: Vec3, aspect: f32) -> Option<(f32, f32, f32)> {
        let cam = self.world_to_camera(world);

        
        if cam.z <= 0.01 {
            return None;
        }

        let tan_half_fov = (self.fov_y * 0.5).tan();

        let y_ndc = cam.y / (cam.z * tan_half_fov);
        let x_ndc = cam.x / (cam.z * tan_half_fov * aspect);

        Some((x_ndc, y_ndc, cam.z))
    }

    
    pub fn update(&mut self, dt: f32, input: &InputState) {
        let move_speed = 50.0;
        let rot_speed = 1.5;

        
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