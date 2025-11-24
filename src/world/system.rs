use std::f32::consts::PI;

use crate::camera::Camera;
use crate::math::Vec3;
use crate::renderer::Renderer;

use super::{Body, BodyKind};
use super::orbit::Orbit;

pub struct SolarSystem {
    pub bodies: Vec<Body>,
}

impl SolarSystem {
    pub fn new_demo() -> Self {
        let mut bodies = Vec::new();

        bodies.push(Body {
            name: "Sol".into(),
            kind: BodyKind::Star,
            radius: 8.0,
            color: 0xFFFFD27F,
            orbit_radius: 0.0,
            orbit_speed: 0.0,
            angle: 0.0,
            parent: None,
            use_procedural: false,
        });

        bodies.push(Body {
            name: "Mercury".into(),
            kind: BodyKind::Planet,
            radius: 3.2,
            color: 0xFFCFEFFF,
            orbit_radius: 30.0,
            orbit_speed: 0.72,
            angle: PI / 6.0,
            parent: Some(0),
            use_procedural: true,
        });

        bodies.push(Body {
            name: "Venus".into(),
            kind: BodyKind::Planet,
            radius: 4.0,
            color: 0xFFEED5A4,
            orbit_radius: 50.0,
            orbit_speed: 0.32,
            angle: PI / 3.0,
            parent: Some(0),
            use_procedural: true,
        });

        bodies.push(Body {
            name: "Super Earth (Our Home)".into(),
            kind: BodyKind::Planet,
            radius: 5.4,
            color: 0xFF8DFF8D,
            orbit_radius: 70.0,
            orbit_speed: 0.54,
            angle: PI / 2.0,
            parent: Some(0),
            use_procedural: true,
        });

        bodies.push(Body {
            name: "Moon".into(),
            kind: BodyKind::Moon,
            radius: 1.8,
            color: 0xFFCFEFFF,
            orbit_radius: 10.0,
            orbit_speed: 2.0,
            angle: PI / 4.0,
            parent: Some(3),
            use_procedural: true,
        });

        bodies.push(Body {
            name: "Mars".into(),
            kind: BodyKind::Planet,
            radius: 4.5,
            color: 0xFFFFA07A,
            orbit_radius: 95.0,
            orbit_speed: 0.28,
            angle: PI / 1.5,
            parent: Some(0),
            use_procedural: true,
        });

        Self { bodies }
    }

    pub fn update(&mut self, dt: f32) {
        for b in &mut self.bodies {
            b.update(dt);
        }
    }

    pub fn body_position(&self, index: usize) -> Vec3 {
        let b = &self.bodies[index];

        match b.parent {
            None => match b.kind {
                BodyKind::Star => Vec3::zero(),
                BodyKind::Planet | BodyKind::Moon => {
                    if b.orbit_radius == 0.0 {
                        Vec3::zero()
                    } else {
                        let orbit = Orbit::new(Vec3::zero(), b.orbit_radius, 64);
                        orbit.position_at(b.angle)
                    }
                }
            },
            Some(parent_idx) => {
                let parent_pos = self.body_position(parent_idx);

                if b.orbit_radius == 0.0 {
                    parent_pos
                } else {
                    let orbit = Orbit::new(parent_pos, b.orbit_radius, 64);
                    orbit.position_at(b.angle)
                }
            }
        }
    }

    pub fn project_body(
        &self,
        index: usize,
        renderer: &Renderer,
        camera: &Camera,
    ) -> Option<((i32, i32), i32)> {
        let b = &self.bodies[index];
        let center_world = self.body_position(index);

        if let Some((sx, sy)) = renderer.project_point(center_world, camera) {
            let sample_world = center_world + Vec3::new(b.radius, 0.0, 0.0);
            let radius_px = if let Some((sx2, sy2)) = renderer.project_point(sample_world, camera) {
                let dx = (sx2 - sx) as f32;
                let dy = (sy2 - sy) as f32;
                let r = (dx * dx + dy * dy).sqrt() as i32;
                if r < 2 { 2 } else { r }
            } else {
                4
            };

            Some(((sx, sy), radius_px))
        } else {
            None
        }
    }

    pub fn render(&self, renderer: &mut Renderer, camera: &Camera) {
        let orbit_color_planet = 0xFF20254F;
        let orbit_color_moon = 0xFF303B7A;

        for b in &self.bodies {
            match b.kind {
                BodyKind::Planet | BodyKind::Moon => {
                    if b.orbit_radius <= 0.0 {
                        continue;
                    }

                    let center_world = match b.parent {
                        None => Vec3::zero(),
                        Some(parent_idx) => self.body_position(parent_idx),
                    };

                    let segments = 64;

                    if let Some(orbit) = b.build_orbit(center_world, segments) {
                        let line_primitives = orbit.generate_line_primitives();

                        for (a_world, b_world) in line_primitives {
                            if let (Some(pa), Some(pb)) = (
                                renderer.project_point(a_world, camera),
                                renderer.project_point(b_world, camera),
                            ) {
                                let col = match b.kind {
                                    BodyKind::Planet => orbit_color_planet,
                                    BodyKind::Moon => orbit_color_moon,
                                    _ => orbit_color_planet,
                                };
                                renderer.draw_line(pa, pb, col);
                            }
                        }
                    }
                }
                BodyKind::Star => {}
            }
        }
    }
}
