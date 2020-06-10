use std::ops::{Add, Div, Mul, Sub};

/**
    MATRIX4
*/
#[derive(Debug, Clone)]
pub struct Matrix4 {
    m: Vec<Vec<f64>>,
}

impl Matrix4 {
    pub fn new() -> Self {
        Self {
            m: vec![vec![0.; 4]; 4],
        }
    }

    pub fn init_identity() -> Self {
        let mut m_: Vec<Vec<f64>> = vec![vec![0.; 4]; 4];
        m_[0][0] = 1.;
        m_[0][1] = 0.;
        m_[0][2] = 0.;
        m_[0][3] = 0.;
        m_[1][0] = 0.;
        m_[1][1] = 1.;
        m_[1][2] = 0.;
        m_[1][3] = 0.;
        m_[2][0] = 0.;
        m_[2][1] = 0.;
        m_[2][2] = 1.;
        m_[2][3] = 0.;
        m_[3][0] = 0.;
        m_[3][1] = 0.;
        m_[3][2] = 0.;
        m_[3][3] = 1.;

        Self { m: m_ }
    }
    pub fn init_screen_space_transform(half_width: f64, half_height: f64) -> Self {
        let mut m_ = vec![vec![0.; 4]; 4];

        m_[0][0] = half_width;
        m_[0][1] = 0.;
        m_[0][2] = 0.;
        m_[0][3] = half_width;
        m_[1][0] = 0.;
        m_[1][1] = -half_height;
        m_[1][2] = 0.;
        m_[1][3] = half_height;
        m_[2][0] = 0.;
        m_[2][1] = 0.;
        m_[2][2] = 1.;
        m_[2][3] = 0.;
        m_[3][0] = 0.;
        m_[3][1] = 0.;
        m_[3][2] = 0.;
        m_[3][3] = 1.;
        Self { m: m_ }
    }

    pub fn init_translation(x: f64, y: f64, z: f64) -> Self {
        let mut m_ = vec![vec![0.; 4]; 4];
        m_[0][0] = 1.;
        m_[0][1] = 0.;
        m_[0][2] = 0.;
        m_[0][3] = x;
        m_[1][0] = 0.;
        m_[1][1] = 1.;
        m_[1][2] = 0.;
        m_[1][3] = y;
        m_[2][0] = 0.;
        m_[2][1] = 0.;
        m_[2][2] = 1.;
        m_[2][3] = z;
        m_[3][0] = 0.;
        m_[3][1] = 0.;
        m_[3][2] = 0.;
        m_[3][3] = 1.;

        Self { m: m_ }
    }

    pub fn init_rotation_angle(x: f64, y: f64, z: f64, angle: f64) -> Self {
        let mut m_ = vec![vec![0.; 4]; 4];

        let sin = angle.sin();
        let cos = angle.cos();

        m_[0][0] = cos + x * x * (1. - cos);
        m_[0][1] = x * y * (1. - cos) - z * sin;
        m_[0][2] = x * z * (1. - cos) + y * sin;
        m_[0][3] = 0.;
        m_[1][0] = y * x * (1. - cos) + z * sin;
        m_[1][1] = cos + y * y * (1. - cos);
        m_[1][2] = y * z * (1. - cos) - x * sin;
        m_[1][3] = 0.;
        m_[2][0] = z * x * (1. - cos) - y * sin;
        m_[2][1] = z * y * (1. - cos) + x * sin;
        m_[2][2] = cos + z * z * (1. - cos);
        m_[2][3] = 0.;
        m_[3][0] = 0.;
        m_[3][1] = 0.;
        m_[3][2] = 0.;
        m_[3][3] = 1.;

        Self { m: m_ }
    }

    pub fn init_rotation(x: f64, y: f64, z: f64) -> Self {
        let mut rx = Matrix4::new();
        let mut ry = Matrix4::new();
        let mut rz = Matrix4::new();

        rz.m[0][0] = z.cos();
        rz.m[0][1] = -z.sin();
        rz.m[0][2] = 0.;
        rz.m[0][3] = 0.;
        rz.m[1][0] = z.sin();
        rz.m[1][1] = z.cos();
        rz.m[1][2] = 0.;
        rz.m[1][3] = 0.;
        rz.m[2][0] = 0.;
        rz.m[2][1] = 0.;
        rz.m[2][2] = 1.;
        rz.m[2][3] = 0.;
        rz.m[3][0] = 0.;
        rz.m[3][1] = 0.;
        rz.m[3][2] = 0.;
        rz.m[3][3] = 1.;

        rx.m[0][0] = 1.;
        rx.m[0][1] = 0.;
        rx.m[0][2] = 0.;
        rx.m[0][3] = 0.;
        rx.m[1][0] = 0.;
        rx.m[1][1] = x.cos();
        rx.m[1][2] = -x.sin();
        rx.m[1][3] = 0.;
        rx.m[2][0] = 0.;
        rx.m[2][1] = x.sin();
        rx.m[2][2] = x.cos();
        rx.m[2][3] = 0.;
        rx.m[3][0] = 0.;
        rx.m[3][1] = 0.;
        rx.m[3][2] = 0.;
        rx.m[3][3] = 1.;

        ry.m[0][0] = y.cos();
        ry.m[0][1] = 0.;
        ry.m[0][2] = -y.sin();
        ry.m[0][3] = 0.;
        ry.m[1][0] = 0.;
        ry.m[1][1] = 1.;
        ry.m[1][2] = 0.;
        ry.m[1][3] = 0.;
        ry.m[2][0] = y.sin();
        ry.m[2][1] = 0.;
        ry.m[2][2] = y.cos();
        ry.m[2][3] = 0.;
        ry.m[3][0] = 0.;
        ry.m[3][1] = 0.;
        ry.m[3][2] = 0.;
        ry.m[3][3] = 1.;

        let mtrx = rz * (ry * rx);

        Self { m: mtrx.m }
    }

    pub fn init_rotation_vec(forward: Vector4, up: Vector4, right: Vector4) -> Self {
        let f = forward;
        let r = right;
        let u = up;

        let mut m_ = vec![vec![0.; 4]; 4];

        m_[0][0] = r.x;
        m_[0][1] = r.y;
        m_[0][2] = r.z;
        m_[0][3] = 0.;
        m_[1][0] = u.x;
        m_[1][1] = u.y;
        m_[1][2] = u.z;
        m_[1][3] = 0.;
        m_[2][0] = f.x;
        m_[2][1] = f.y;
        m_[2][2] = f.z;
        m_[2][3] = 0.;
        m_[3][0] = 0.;
        m_[3][1] = 0.;
        m_[3][2] = 0.;
        m_[3][3] = 1.;

        Self { m: m_ }
    }

    pub fn init_rotation_fu(forward: Vector4, up: Vector4) -> Matrix4 {
        let f = forward.normalized();
        let r = up.normalized();
        let gr = r.cross(f);

        let u = f.cross(gr);

        Matrix4::init_rotation_vec(f, u, r)
    }

    pub fn init_scale(x: f64, y: f64, z: f64) -> Self {
        let mut m_ = vec![vec![0.; 4]; 4];

        m_[0][0] = x;
        m_[0][1] = 0.;
        m_[0][2] = 0.;
        m_[0][3] = 0.;
        m_[1][0] = 0.;
        m_[1][1] = y;
        m_[1][2] = 0.;
        m_[1][3] = 0.;
        m_[2][0] = 0.;
        m_[2][1] = 0.;
        m_[2][2] = z;
        m_[2][3] = 0.;
        m_[3][0] = 0.;
        m_[3][1] = 0.;
        m_[3][2] = 0.;
        m_[3][3] = 1.;

        Self { m: m_ }
    }

    pub fn init_perspective(fov: f64, aspect_ratio: f64, z_near: f64, z_far: f64) -> Self {
        let tan_half_fov = (fov / 2.).tan();
        let z_range = z_near - z_far;

        let mut m_ = vec![vec![0.; 4]; 4];

        m_[0][0] = 1. / (tan_half_fov * aspect_ratio);
        m_[0][1] = 0.;
        m_[0][2] = 0.;
        m_[0][3] = 0.;
        m_[1][0] = 0.;
        m_[1][1] = 1. / tan_half_fov;
        m_[1][2] = 0.;
        m_[1][3] = 0.;
        m_[2][0] = 0.;
        m_[2][1] = 0.;
        m_[2][2] = (-z_near - z_far) / z_range;
        m_[2][3] = 2. * z_far * z_near / z_range;
        m_[3][0] = 0.;
        m_[3][1] = 0.;
        m_[3][2] = 1.;
        m_[3][3] = 0.;

        Self { m: m_ }
    }

    pub fn init_orthographic(
        left: f64,
        right: f64,
        bottom: f64,
        top: f64,
        near: f64,
        far: f64,
    ) -> Self {
        let width = right - left;
        let height = top - bottom;
        let depth = far - near;

        let mut m_ = vec![vec![0.; 4]; 4];

        m_[0][0] = 2. / width;
        m_[0][1] = 0.;
        m_[0][2] = 0.;
        m_[0][3] = -(right + left) / width;
        m_[1][0] = 0.;
        m_[1][1] = 2. / height;
        m_[1][2] = 0.;
        m_[1][3] = -(top + bottom) / height;
        m_[2][0] = 0.;
        m_[2][1] = 0.;
        m_[2][2] = -2. / depth;
        m_[2][3] = -(far + near) / depth;
        m_[3][0] = 0.;
        m_[3][1] = 0.;
        m_[3][2] = 0.;
        m_[3][3] = 1.;

        Self { m: m_ }
    }

    pub fn transform(&self, r: Vector4) -> Vector4 {
        Vector4::new(
            self.m[0][0] * r.x + self.m[0][1] * r.y + self.m[0][2] * r.z + self.m[0][3] * r.w,
            self.m[1][0] * r.x + self.m[1][1] * r.y + self.m[1][2] * r.z + self.m[1][3] * r.w,
            self.m[2][0] * r.x + self.m[2][1] * r.y + self.m[2][2] * r.z + self.m[2][3] * r.w,
            self.m[3][0] * r.x + self.m[3][1] * r.y + self.m[3][2] * r.z + self.m[3][3] * r.w,
        )
    }

    pub fn get(&mut self, x: usize, y: usize) -> f64 {
        self.m[x][y]
    }

    pub fn set(&mut self, x: usize, y: usize, value: f64) {
        self.m[x][y] = value;
    }

    pub fn to_string(self) -> String {
        self.m[0][0].to_string()
            + " "
            + &self.m[0][1].to_string()
            + " "
            + &self.m[0][2].to_string()
            + " "
            + &self.m[0][3].to_string()
            + "\n"
            + &self.m[1][0].to_string()
            + " "
            + &self.m[1][1].to_string()
            + " "
            + &self.m[1][2].to_string()
            + " "
            + &self.m[1][3].to_string()
            + "\n"
            + &self.m[2][0].to_string()
            + " "
            + &self.m[2][1].to_string()
            + " "
            + &self.m[2][2].to_string()
            + " "
            + &self.m[2][3].to_string()
            + "\n"
            + &self.m[3][0].to_string()
            + " "
            + &self.m[3][1].to_string()
            + " "
            + &self.m[3][2].to_string()
            + " "
            + &self.m[3][3].to_string()
    }
}

impl Mul for Matrix4 {
    type Output = Matrix4;

    fn mul(self, mtr: Matrix4) -> Matrix4 {
        let mut res = Matrix4::new();
        let mut other = mtr.clone();

        for i in 0..4 {
            for j in 0..4 {
                res.set(
                    i,
                    j,
                    self.m[i][0] * other.get(0, j)
                        + self.m[i][1] * other.get(1, j)
                        + self.m[i][2] * other.get(2, j)
                        + self.m[i][3] * other.get(3, j),
                );
            }
        }

        res
    }
}

/**
    VECTOR4
*/
#[derive(Debug, Clone, Copy)]
pub struct Vector4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vector4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    pub fn length(&self) -> f64 {
        let val = self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;

        val.sqrt()
    }

    pub fn max(&self) -> f64 {
        f64::max(self.x.max(self.y), self.z.max(self.w))
    }

    pub fn dot(&self, r: Vector4) -> f64 {
        self.x * r.x + self.y * r.y + self.z * r.z + self.w * r.w
    }

    pub fn cross(&self, r: Vector4) -> Vector4 {
        let x_: f64 = self.y * r.z - self.z * r.y;
        let y_: f64 = self.z * r.x - self.x * r.z;
        let z_: f64 = self.x * r.y - self.y * r.x;

        Vector4::new(x_, y_, z_, 0.)
    }

    pub fn normalized(&self) -> Vector4 {
        let len = self.length();

        Vector4::new(self.x / len, self.y / len, self.z / len, self.w / len)
    }

    pub fn rotate_quaternion(self, rotation: Quaternion) -> Vector4 {
        let conjugate = rotation.conjugate();

        let w: Quaternion = rotation * self * conjugate;

        Vector4::new(w.m_x, w.m_y, w.m_z, 0.)
    }

    pub fn rotate(self, axis: Vector4, angle: f64) -> Vector4 {
        let sin_angle: f64 = -angle.sin();
        let cos_angle: f64 = -angle.cos();

        let t: Vector4 = axis * sin_angle;
        let a: Vector4 = self.cross(t);
        let b: f64 = self.dot(axis * (1. - cos_angle));

        a + (self * cos_angle) + b
    }

    pub fn lerp(self, dest: Vector4, factor: f64) -> Vector4 {
        dest - self * factor + self
    }

    pub fn abs(self) -> Vector4 {
        Vector4::new(self.x.abs(), self.y.abs(), self.z.abs(), self.w.abs())
    }

    pub fn to_string(&self) -> String {
        [
            "x: ",
            &self.x.to_string()[..],
            " y: ",
            &self.y.to_string()[..],
            " z: ",
            &self.z.to_string()[..],
            " w: ",
            &self.w.to_string()[..],
        ]
        .concat()
    }
}

impl Add for Vector4 {
    type Output = Vector4;

    fn add(self, other: Vector4) -> Vector4 {
        Vector4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}
impl Add<f64> for Vector4 {
    type Output = Vector4;

    fn add(self, other: f64) -> Vector4 {
        Vector4 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
            w: self.w + other,
        }
    }
}

impl Sub for Vector4 {
    type Output = Vector4;

    fn sub(self, other: Vector4) -> Vector4 {
        Vector4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}
impl Sub<f64> for Vector4 {
    type Output = Vector4;

    fn sub(self, value: f64) -> Vector4 {
        Vector4 {
            x: self.x - value,
            y: self.y - value,
            z: self.z - value,
            w: self.w - value,
        }
    }
}

impl Mul for Vector4 {
    type Output = Vector4;

    fn mul(self, other: Vector4) -> Vector4 {
        Vector4 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}
impl Mul<f64> for Vector4 {
    type Output = Self;

    fn mul(self, value: f64) -> Self {
        Self {
            x: self.x * value,
            y: self.y * value,
            z: self.z * value,
            w: self.w * value,
        }
    }
}

impl Div for Vector4 {
    type Output = Vector4;

    fn div(self, other: Vector4) -> Vector4 {
        Vector4 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w,
        }
    }
}
impl Div<f64> for Vector4 {
    type Output = Self;

    fn div(self, value: f64) -> Self {
        Self {
            x: self.x / value,
            y: self.y / value,
            z: self.z / value,
            w: self.w / value,
        }
    }
}

impl PartialEq for Vector4 {
    fn eq(&self, other: &Self) -> bool {
        &self.length() == &other.length()
    }
}

/**
    QUATERNION
*/
#[derive(Debug, Clone, Copy)]
struct Quaternion {
    pub m_x: f64,
    m_y: f64,
    m_z: f64,
    m_w: f64,
}

impl Quaternion {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self {
            m_x: x,
            m_y: y,
            m_z: z,
            m_w: w,
        }
    }

    pub fn new_vec_angle(axis: Vector4, angle: f64) -> Self {
        let sin_half_angle = (angle / 2.).sin();
        let cos_half_angle = (angle / 2.).cos();

        Self {
            m_x: axis.x * sin_half_angle,
            m_y: axis.y * sin_half_angle,
            m_z: axis.z * sin_half_angle,
            m_w: cos_half_angle,
        }
    }

    pub fn new_mat(rot: &mut Matrix4) -> Self {
        let trace = rot.get(0, 0) + rot.get(1, 1) + rot.get(2, 2);

        let mut mw: f64;
        let mut mx: f64;
        let mut my: f64;
        let mut mz: f64;

        if trace > 0. {
            let s = 0.5 / (trace + 1.).sqrt();
            mw = 0.25 / s;
            mx = (rot.get(1, 2) - rot.get(2, 1)) * s;
            my = (rot.get(2, 0) - rot.get(0, 2)) * s;
            mz = (rot.get(0, 1) - rot.get(1, 0)) * s;
        } else {
            if rot.get(0, 0) > rot.get(1, 1) && rot.get(0, 0) > rot.get(2, 2) {
                let s = 2. * (1. + rot.get(0, 0) - rot.get(1, 1) - rot.get(2, 2)).sqrt();
                mw = (rot.get(1, 2) - rot.get(2, 1)) / s;
                mx = 0.25 * s;
                my = (rot.get(1, 0) + rot.get(0, 1)) / s;
                mz = (rot.get(2, 0) + rot.get(0, 2)) / s;
            } else if rot.get(1, 1) > rot.get(2, 2) {
                let s = 2. * (1. + rot.get(1, 1) - rot.get(0, 0) - rot.get(2, 2)).sqrt();
                mw = (rot.get(2, 0) - rot.get(0, 2)) / s;
                mx = (rot.get(1, 0) + rot.get(0, 1)) / s;
                my = 0.25 * s;
                mz = (rot.get(2, 1) + rot.get(1, 2)) / s;
            } else {
                let s = 2. * (1. + rot.get(2, 2) - rot.get(0, 0) - rot.get(1, 1)).sqrt();
                mw = (rot.get(0, 1) - rot.get(1, 0)) / s;
                mx = (rot.get(2, 0) + rot.get(0, 2)) / s;
                my = (rot.get(1, 2) + rot.get(2, 1)) / s;
                mz = 0.25 * s;
            }
        }

        let length = (mx * mx + my * my + mz * mz + mw * mw).sqrt();
        mx /= length;
        my /= length;
        mz /= length;
        mw /= length;

        Self {
            m_w: mw,
            m_x: mx,
            m_y: my,
            m_z: mz,
        }
    }

    pub fn length(&self) -> f64 {
        (self.m_x * self.m_x + self.m_y * self.m_y + self.m_z * self.m_z + self.m_w * self.m_w)
            .sqrt()
    }

    pub fn normalized(&self) -> Quaternion {
        let len = self.length();

        Quaternion::new(
            self.m_x / len,
            self.m_y / len,
            self.m_z / len,
            self.m_w / len,
        )
    }

    pub fn conjugate(&self) -> Quaternion {
        Quaternion::new(-self.m_x, -self.m_y, -self.m_z, self.m_w)
    }

    pub fn to_rotation_matrix(&self) -> Matrix4 {
        let forward = Vector4::new(
            2. * (self.m_x * self.m_z - self.m_w * self.m_y),
            2. * (self.m_y * self.m_z + self.m_w * self.m_x),
            1. - 2. * (self.m_x * self.m_x + self.m_y * self.m_y),
            1.,
        );
        let up = Vector4::new(
            2. * (self.m_x * self.m_y + self.m_w * self.m_z),
            1. - 2. * (self.m_x * self.m_x + self.m_z * self.m_z),
            2. * (self.m_y * self.m_z - self.m_w * self.m_x),
            1.,
        );
        let right = Vector4::new(
            1. - 2. * (self.m_y * self.m_y + self.m_z * self.m_z),
            2. * (self.m_x * self.m_y - self.m_w * self.m_z),
            2. * (self.m_x * self.m_z + self.m_w * self.m_y),
            1.,
        );

        Matrix4::init_rotation_vec(forward, up, right)
    }

    pub fn dot(&self, r: Quaternion) -> f64 {
        self.m_x * r.m_x + self.m_y * r.m_y + self.m_z * r.m_z + self.m_w * r.m_w
    }

    pub fn n_lerp(self, dest: Quaternion, lerp_factor: f64, shortest: bool) -> Quaternion {
        let mut corrected_dest = dest;

        if shortest && self.dot(dest) < 0. {
            corrected_dest = Quaternion::new(-dest.m_x, -dest.m_y, -dest.m_z, -dest.m_w);
        }

        (corrected_dest - self * lerp_factor + self).normalized()
    }

    pub fn s_lerp(self, dest: Quaternion, lerp_factor: f64, shortest: bool) -> Quaternion {
        let EPSILON = 1e3;

        let mut cos = self.dot(dest);
        let mut corrected_dest: Quaternion = dest;

        if shortest && cos < 0. {
            cos = -cos;
            corrected_dest = Quaternion::new(-dest.m_x, -dest.m_y, -dest.m_z, -dest.m_w);
        }

        if cos.abs() >= 1. - EPSILON {
            return self.n_lerp(corrected_dest, lerp_factor, false);
        }

        let sin = (1. - cos * cos).sqrt();
        let angle = sin.atan2(cos);
        let inv_sin = 1.0 / sin;

        let src_factor = ((1. - lerp_factor) * angle).sin() * inv_sin;
        let dest_factor = ((lerp_factor) * angle).sin() * inv_sin;

        self * src_factor + corrected_dest * dest_factor
    }

    pub fn get_forward(self) -> Vector4 {
        Vector4::new(0., 0., 1., 0.).rotate_quaternion(self)
    }

    pub fn get_back(self) -> Vector4 {
        Vector4::new(0., 0., -1., 0.).rotate_quaternion(self)
    }

    pub fn get_up(self) -> Vector4 {
        Vector4::new(0., 1., 0., 0.).rotate_quaternion(self)
    }

    pub fn get_down(self) -> Vector4 {
        Vector4::new(0., -1., 0., 0.).rotate_quaternion(self)
    }

    pub fn get_right(self) -> Vector4 {
        Vector4::new(1., 0., 0., 0.).rotate_quaternion(self)
    }

    pub fn get_left(self) -> Vector4 {
        Vector4::new(-1., 0., 0., 0.).rotate_quaternion(self)
    }

    pub fn set_f(&mut self, x: f64, y: f64, z: f64, w: f64) {
        self.m_x = x;
        self.m_y = y;
        self.m_z = z;
        self.m_w = w;
    }

    pub fn set_q(&mut self, r: Quaternion) {
        self.set_f(r.m_x, r.m_y, r.m_z, r.m_w);
    }
}

impl Add for Quaternion {
    type Output = Quaternion;

    fn add(self, other: Quaternion) -> Quaternion {
        Quaternion {
            m_x: self.m_x + other.m_x,
            m_y: self.m_y + other.m_y,
            m_z: self.m_z + other.m_z,
            m_w: self.m_w + other.m_w,
        }
    }
}

impl Sub for Quaternion {
    type Output = Quaternion;

    fn sub(self, other: Quaternion) -> Quaternion {
        Quaternion {
            m_x: self.m_x - other.m_x,
            m_y: self.m_y - other.m_y,
            m_z: self.m_z - other.m_z,
            m_w: self.m_w - other.m_w,
        }
    }
}

impl Mul for Quaternion {
    type Output = Quaternion;

    fn mul(self, r: Quaternion) -> Quaternion {
        let w_ = self.m_w * r.m_w - self.m_x * r.m_x - self.m_y * r.m_y - self.m_z * r.m_z;
        let x_ = self.m_x * r.m_w + self.m_w * r.m_x + self.m_y * r.m_z - self.m_z * r.m_y;
        let y_ = self.m_y * r.m_w + self.m_w * r.m_y + self.m_z * r.m_x - self.m_x * r.m_z;
        let z_ = self.m_z * r.m_w + self.m_w * r.m_z + self.m_x * r.m_y - self.m_y * r.m_x;
        Quaternion {
            m_x: x_,
            m_y: y_,
            m_z: z_,
            m_w: w_,
        }
    }
}
impl Mul<f64> for Quaternion {
    type Output = Quaternion;

    fn mul(self, value: f64) -> Quaternion {
        Quaternion {
            m_x: self.m_x * value,
            m_y: self.m_y * value,
            m_z: self.m_z * value,
            m_w: self.m_w * value,
        }
    }
}
impl Mul<Vector4> for Quaternion {
    type Output = Quaternion;

    fn mul(self, r: Vector4) -> Quaternion {
        let w_ = -self.m_x * r.x - self.m_y * r.y - self.m_z * r.z;
        let x_ = self.m_w * r.x + self.m_y * r.z - self.m_z * r.y;
        let y_ = self.m_w * r.y + self.m_z * r.x - self.m_x * r.z;
        let z_ = self.m_w * r.z + self.m_x * r.y - self.m_y * r.x;
        Quaternion {
            m_x: x_,
            m_y: y_,
            m_z: z_,
            m_w: w_,
        }
    }
}

impl PartialEq for Quaternion {
    fn eq(&self, other: &Quaternion) -> bool {
        self.m_x == other.m_x
            && self.m_y == other.m_y
            && self.m_z == other.m_z
            && self.m_w == other.m_w
    }
}
