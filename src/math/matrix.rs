use std::ops::{Mul, Index, IndexMut};
use crate::math::{Vec3, Vec4};

#[derive(Clone, Copy, Debug)]
pub struct Matrix4 {
    pub data: [[f32; 4]; 4],
}

impl Matrix4 {
    
    pub fn identity() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    
    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        let mut m = Self::identity();
        m.data[0][3] = x;
        m.data[1][3] = y;
        m.data[2][3] = z;
        m
    }

    
    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        let mut m = Self::identity();
        m.data[0][0] = x;
        m.data[1][1] = y;
        m.data[2][2] = z;
        m
    }

    
    pub fn rotation_y(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        let mut m = Self::identity();
        m.data[0][0] = c;
        m.data[0][2] = s;
        m.data[2][0] = -s;
        m.data[2][2] = c;
        m
    }

    
    pub fn rotation_x(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        let mut m = Self::identity();
        m.data[1][1] = c;
        m.data[1][2] = -s;
        m.data[2][1] = s;
        m.data[2][2] = c;
        m
    }

    
    pub fn rotation_z(angle: f32) -> Self {
        let c = angle.cos();
        let s = angle.sin();
        let mut m = Self::identity();
        m.data[0][0] = c;
        m.data[0][1] = -s;
        m.data[1][0] = s;
        m.data[1][1] = c;
        m
    }

    
    pub fn look_at(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        let f = (center - eye).normalized();
        let s = Vec3::cross(f, up).normalized();
        let u = Vec3::cross(s, f);

        let mut m = Self::identity();
        m.data[0][0] = s.x;
        m.data[0][1] = s.y;
        m.data[0][2] = s.z;
        m.data[1][0] = u.x;
        m.data[1][1] = u.y;
        m.data[1][2] = u.z;
        m.data[2][0] = -f.x;
        m.data[2][1] = -f.y;
        m.data[2][2] = -f.z;
        m.data[0][3] = -s.dot(eye);
        m.data[1][3] = -u.dot(eye);
        m.data[2][3] = f.dot(eye);
        m
    }

    
    pub fn perspective(fov_y: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fov_y / 2.0).tan();
        let mut m = Self::identity();
        
        m.data[0][0] = f / aspect;
        m.data[1][1] = f;
        m.data[2][2] = (far + near) / (near - far);
        m.data[2][3] = (2.0 * far * near) / (near - far);
        m.data[3][2] = -1.0;
        m.data[3][3] = 0.0;
        
        m
    }

    
    pub fn transpose(&self) -> Self {
        let mut result = Self::identity();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = self.data[j][i];
            }
        }
        result
    }

    
    pub fn inverse(&self) -> Self {
       
        let mut result = Self::identity();
        
        
        for i in 0..3 {
            for j in 0..3 {
                result.data[i][j] = self.data[j][i]; 
            }
        }
        
        
        let tx = -self.data[0][3];
        let ty = -self.data[1][3];
        let tz = -self.data[2][3];
        
        result.data[0][3] = result.data[0][0] * tx + result.data[0][1] * ty + result.data[0][2] * tz;
        result.data[1][3] = result.data[1][0] * tx + result.data[1][1] * ty + result.data[1][2] * tz;
        result.data[2][3] = result.data[2][0] * tx + result.data[2][1] * ty + result.data[2][2] * tz;
        
        result
    }
}

impl Mul for Matrix4 {
    type Output = Matrix4;

    fn mul(self, other: Matrix4) -> Matrix4 {
        let mut result = Matrix4::identity();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = 0.0;
                for k in 0..4 {
                    result.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        result
    }
}

impl Mul<Vec4> for Matrix4 {
    type Output = Vec4;

    fn mul(self, v: Vec4) -> Vec4 {
        Vec4::new(
            self.data[0][0] * v.x + self.data[0][1] * v.y + self.data[0][2] * v.z + self.data[0][3] * v.w,
            self.data[1][0] * v.x + self.data[1][1] * v.y + self.data[1][2] * v.z + self.data[1][3] * v.w,
            self.data[2][0] * v.x + self.data[2][1] * v.y + self.data[2][2] * v.z + self.data[2][3] * v.w,
            self.data[3][0] * v.x + self.data[3][1] * v.y + self.data[3][2] * v.z + self.data[3][3] * v.w,
        )
    }
}

impl Index<usize> for Matrix4 {
    type Output = [f32; 4];
    
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}