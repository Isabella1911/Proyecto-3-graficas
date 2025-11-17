use crate::camera::Camera;
use crate::math::Vec3;
use crate::world::{BodyKind, SolarSystem};

pub fn resolve_collisions(system: &SolarSystem, camera: &mut Camera) {
    let margin = 2.0;

    for i in 0..system.bodies.len() {
        let b = &system.bodies[i];

        match b.kind {
            BodyKind::Star | BodyKind::Planet | BodyKind::Moon => {
                let center = system.body_position(i);
                let to_cam = camera.position - center;
                let dist = to_cam.length();
                let min_dist = b.radius + margin;

                if dist < min_dist {
                    let dir = if dist == 0.0 {
                        Vec3::up()
                    } else {
                        to_cam / dist
                    };
                    camera.position = center + dir * min_dist;
                }
            }
        }
    }
}
