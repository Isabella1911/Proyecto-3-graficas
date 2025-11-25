use std::ops::{Add, Sub, Mul, Div};

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn add(self, other: Vec2) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }

    pub fn sub(self, other: Vec2) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }

    pub fn mul(self, s: f32) -> Self {
        Self::new(self.x * s, self.y * s)
    }

    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalized(self) -> Self {
        let len = self.length();
        if len == 0.0 {
            Self::zero()
        } else {
            self.mul(1.0 / len)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn up() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalized(self) -> Self {
        let len = self.length();
        if len == 0.0 {
            Self::zero()
        } else {
            self / len
        }
    }

    pub fn dot(self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
        Vec3::new(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        )
    }

    pub fn crossm(self, other: Vec3) -> Vec3 {
        Vec3::cross(self, other)
    }

    pub fn lerp(self, target: Vec3, t: f32) -> Vec3 {
        self + (target - self) * t
    }
    
    
    pub fn to_vec4_point(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, 1.0)
    }
    
    
    pub fn to_vec4_dir(self) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, 0.0)
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, s: f32) -> Vec3 {
        Vec3::new(self.x * s, self.y * s, self.z * s)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, s: f32) -> Vec3 {
        Vec3::new(self.x / s, self.y / s, self.z / s)
    }
}


#[derive(Clone, Copy, Debug)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
    }

    
    pub fn to_vec3(self) -> Vec3 {
        if self.w != 0.0 {
            Vec3::new(self.x / self.w, self.y / self.w, self.z / self.w)
        } else {
            Vec3::new(self.x, self.y, self.z)
        }
    }

    
    pub fn perspective_divide(self) -> Self {
        if self.w != 0.0 {
            Vec4::new(self.x / self.w, self.y / self.w, self.z / self.w, 1.0)
        } else {
            self
        }
    }

    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn dot(self, other: Vec4) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
}

impl Add for Vec4 {
    type Output = Vec4;
    fn add(self, other: Vec4) -> Vec4 {
        Vec4::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

impl Sub for Vec4 {
    type Output = Vec4;
    fn sub(self, other: Vec4) -> Vec4 {
        Vec4::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }
}

impl Mul<f32> for Vec4 {
    type Output = Vec4;
    fn mul(self, s: f32) -> Vec4 {
        Vec4::new(self.x * s, self.y * s, self.z * s, self.w * s)
    }
}

impl Div<f32> for Vec4 {
    type Output = Vec4;
    fn div(self, s: f32) -> Vec4 {
        Vec4::new(self.x / s, self.y / s, self.z / s, self.w / s)
    }
}