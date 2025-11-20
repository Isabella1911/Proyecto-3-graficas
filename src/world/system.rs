use std::f32::consts::PI;

use crate::camera::Camera;
use crate::math::Vec3;
use crate::renderer::Renderer;

use super::{Body, BodyKind};

pub struct SolarSystem {
    pub bodies: Vec<Body>,
}

impl SolarSystem {
    pub fn new_demo() -> Self {
        let mut bodies = Vec::new();

        // Sol (0)
        bodies.push(Body {
            name: "Sol".into(),
            kind: BodyKind::Star,
            radius: 8.0,
            color: 0xFFFFD27F,
            orbit_radius: 0.0,
            orbit_speed: 0.0,
            angle: 0.0,
            parent: None,
        });

        // Planeta 1 (1)
        bodies.push(Body {
            name: "Mercury".into(),
            kind: BodyKind::Planet,
            radius: 3.2,
            color: 0xFF5CC8FF,
            orbit_radius: 25.0,
            orbit_speed: 0.12,
            angle: 0.0,
            parent: Some(0),
        });

        // Planeta 2 (2)
        bodies.push(Body {
            name: "Venus".into(),
            kind: BodyKind::Planet,
            radius: 4.5,
            color: 0xFFFF7AC8,
            orbit_radius: 50.0,
            orbit_speed: 0.32,
            angle: PI / 3.0,
            parent: Some(0),
        });

        // Planeta 3 (3)
        bodies.push(Body {
            name: "Super Earth (Our Home)".into(),
            kind: BodyKind::Planet,
            radius: 5.4,
            color: 0xFF8DFF8D,
            orbit_radius: 70.0,
            orbit_speed: 0.54,
            angle: PI / 2.0,
            parent: Some(0),
        });

        // Luna de Verdania (4)
        bodies.push(Body {
            name: "Super Moon".into(),
            kind: BodyKind::Moon,
            radius: 1.8,
            color: 0xFFCFEFFF,
            orbit_radius: 10.0,
            orbit_speed: 2.0,
            angle: PI / 4.0,
            parent: Some(3),
        });

        bodies.push(Body {
            name: "Mars".into(),
            kind: BodyKind::Planet,
            radius: 6.0,
            color: 0xFFCFEFFF,
            orbit_radius: 90.0,
            orbit_speed: 1.0,
            angle: PI / 4.0,
            parent: Some(0),
        });


        SolarSystem { bodies }
    }

    pub fn update(&mut self, dt: f32) {
        for b in &mut self.bodies {
            b.update(dt);
        }
    }

    /// Posición global del cuerpo i
    pub fn body_position(&self, index: usize) -> Vec3 {
        let b = &self.bodies[index];

        match b.parent {
            None => match b.kind {
                BodyKind::Star => Vec3::zero(),
                BodyKind::Planet | BodyKind::Moon => {
                    let x = b.orbit_radius * b.angle.cos();
                    let z = b.orbit_radius * b.angle.sin();
                    Vec3::new(x, 0.0, z)
                }
            },
            Some(parent_idx) => {
                let parent_pos = self.body_position(parent_idx);
                if b.orbit_radius == 0.0 {
                    parent_pos
                } else {
                    let x = b.orbit_radius * b.angle.cos();
                    let z = b.orbit_radius * b.angle.sin();
                    parent_pos + Vec3::new(x, 0.0, z)
                }
            }
        }
    }

    /// Posición en pantalla + radio del cuerpo `index`, para dibujar la esfera texturizada
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

    /// Solo dibuja órbitas (los cuerpos los dibuja App con texturas)
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
                    let mut prev: Option<(i32, i32)> = None;

                    for s in 0..=segments {
                        let t = s as f32 / segments as f32 * 2.0 * PI;
                        let x = center_world.x + b.orbit_radius * t.cos();
                        let z = center_world.z + b.orbit_radius * t.sin();
                        let world = Vec3::new(x, center_world.y, z);

                        if let Some(screen) = renderer.project_point(world, camera) {
                            if let Some(prev_pt) = prev {
                                let col = match b.kind {
                                    BodyKind::Planet => orbit_color_planet,
                                    BodyKind::Moon => orbit_color_moon,
                                    _ => orbit_color_planet,
                                };
                                renderer.draw_line(prev_pt, screen, col);
                            }
                            prev = Some(screen);
                        }
                    }
                }
                BodyKind::Star => {}
            }
        }
    }
}
