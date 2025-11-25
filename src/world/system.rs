use std::f32::consts::PI;

use crate::camera::Camera;
use crate::math::{Vec3, Matrix4};
use crate::renderer::{Renderer, mesh::Mesh, pipeline::ShaderType};
use crate::texture::Texture;

use super::{Body, BodyKind};

pub struct SolarSystem {
    pub bodies: Vec<Body>,
    // Mallas compartidas para todos los cuerpos
    sphere_mesh: Mesh,
    orbit_meshes: Vec<Option<Mesh>>,
}

impl SolarSystem {
    pub fn new_demo() -> Self {
        let mut bodies = Vec::new();

        // Sol
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

        // Mercurio
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

        // Venus
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

        // Tierra
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

        // Luna
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

        // Marte
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

        
        let sphere_mesh = Mesh::create_sphere(1.0, 32, 32);
        
        
        let mut orbit_meshes = Vec::new();
        for body in &bodies {
            if body.orbit_radius > 0.0 {
                orbit_meshes.push(Some(Mesh::create_orbit_ring(body.orbit_radius, 64)));
            } else {
                orbit_meshes.push(None);
            }
        }

        Self { 
            bodies, 
            sphere_mesh,
            orbit_meshes,
        }
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
                        Vec3::new(
                            b.orbit_radius * b.angle.cos(),
                            0.0,
                            b.orbit_radius * b.angle.sin(),
                        )
                    }
                }
            },
            Some(parent_idx) => {
                let parent_pos = self.body_position(parent_idx);
                if b.orbit_radius == 0.0 {
                    parent_pos
                } else {
                    parent_pos + Vec3::new(
                        b.orbit_radius * b.angle.cos(),
                        0.0,
                        b.orbit_radius * b.angle.sin(),
                    )
                }
            }
        }
    }

    
    pub fn render_pipeline(
        &self,
        renderer: &mut Renderer,
        camera: &Camera,
        sun_texture: &Texture,
        planet_textures: &[Option<Texture>],
    ) {
        
        renderer.setup_camera(camera);
        
        
        let sun_pos = self.body_position(0);
        renderer.pipeline.uniforms.light_pos = sun_pos;
        renderer.pipeline.uniforms.light_color = Vec3::new(1.0, 0.95, 0.8);
        renderer.pipeline.uniforms.ambient_color = Vec3::new(0.1, 0.1, 0.15);
        
        
        for (i, body) in self.bodies.iter().enumerate() {
            if body.orbit_radius > 0.0 {
                let center = match body.parent {
                    None => Vec3::zero(),
                    Some(parent_idx) => self.body_position(parent_idx),
                };
                
                if let Some(orbit_mesh) = &self.orbit_meshes[i] {
                    
                    let orbit_model = Matrix4::translation(center.x, center.y, center.z);
                    
                    
                    let orbit_color = match body.kind {
                        BodyKind::Planet => Vec3::new(0.125, 0.145, 0.31),
                        BodyKind::Moon => Vec3::new(0.188, 0.231, 0.478),
                        _ => Vec3::new(0.1, 0.1, 0.1),
                    };
                    
                    
                    renderer.render_mesh_pipeline(
                        orbit_mesh,
                        orbit_model,
                        ShaderType::Basic,
                        None,
                    );
                }
            }
        }
        
        
        let mut body_indices: Vec<(usize, f32)> = Vec::new();
        for i in 0..self.bodies.len() {
            let body_pos = self.body_position(i);
            let distance = (body_pos - camera.position).length();
            body_indices.push((i, distance));
        }
        body_indices.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
    
        for (i, _) in body_indices {
            let body = &self.bodies[i];
            let position = self.body_position(i);
            
            
            let (shader_type, texture) = match body.kind {
                BodyKind::Star => {
                    
                    (ShaderType::Star, Some(sun_texture))
                },
                BodyKind::Planet | BodyKind::Moon => {
                    
                    let tex = if i > 0 && i - 1 < planet_textures.len() {
                        planet_textures[i - 1].as_ref()
                    } else {
                        None
                    };
                    
                    if tex.is_some() {
                        (ShaderType::Textured, tex)
                    } else {
                        (ShaderType::Phong, None)
                    }
                },
            };
            
            
            renderer.render_solar_body(
                position,
                body.radius,
                body.angle,
                shader_type,
                texture,
                &self.sphere_mesh,
            );
        }
        
        
        renderer.present();
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
                    let step = 2.0 * PI / segments as f32;
                    
                    for i in 0..segments {
                        let angle1 = i as f32 * step;
                        let angle2 = ((i + 1) % segments) as f32 * step;
                        
                        let p1 = center_world + Vec3::new(
                            b.orbit_radius * angle1.cos(),
                            0.0,
                            b.orbit_radius * angle1.sin(),
                        );
                        
                        let p2 = center_world + Vec3::new(
                            b.orbit_radius * angle2.cos(),
                            0.0,
                            b.orbit_radius * angle2.sin(),
                        );
                        
                        if let (Some(pa), Some(pb)) = (
                            renderer.project_point(p1, camera),
                            renderer.project_point(p2, camera),
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
                BodyKind::Star => {}
            }
        }
    }
}