use std::{fmt::Display, ops};

#[derive(Copy, Clone)]
pub struct Vec3 {
    data: [f32; 3],
}

impl Vec3 {
    pub fn new(val1: f32, val2: f32, val3: f32) -> Self {
        Vec3 {
            data: [val1, val2, val3],
        }
    }

    pub const fn x(&self) -> f32 {
        self.data[0]
    }
    pub const fn y(&self) -> f32 {
        self.data[1]
    }
    pub const fn z(&self) -> f32 {
        self.data[2]
    }
    pub const fn r(&self) -> f32 {
        self.data[0]
    }
    pub const fn g(&self) -> f32 {
        self.data[1]
    }
    pub const fn b(&self) -> f32 {
        self.data[2]
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.data[0].powi(2) + self.data[1].powi(2) + self.data[2].powi(2))
    }

    pub fn length_squared(&self) -> f32 {
        self.data[0].powi(2) + self.data[1].powi(2) + self.data[2].powi(2)
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.data[0] * other.data[0] + self.data[1] * other.data[1] + self.data[2] * other.data[2]
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.data[1] * other.data[2] - self.data[2] * other.data[1],
            -(self.data[0] * other.data[2] - self.data[2] * other.data[0]),
            self.data[0] * other.data[1] - self.data[1] * other.data[0],
        )
    }

    pub fn unit(&self) -> Vec3 {
        let k = 1.0 / self.length();
        Vec3::new(self.data[0] * k, self.data[1] * k, self.data[2] * k)
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.data[0], -self.data[1], -self.data[2])
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3::new(
            a.data[0] + b.data[0],
            a.data[1] + b.data[1],
            a.data[2] + b.data[2],
        )
});

impl_op_ex!(+= |a: &mut Vec3, b: &Vec3| {
        a.data[0] += b.data[0];
        a.data[1] += b.data[1];
        a.data[2] += b.data[2];
});

impl_op_ex!(-|a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3::new(
        a.data[0] - b.data[0],
        a.data[1] - b.data[1],
        a.data[2] - b.data[2],
    )
});

impl_op_ex!(-= |a: &mut Vec3, b: &Vec3| {
        a.data[0] -= b.data[0];
        a.data[1] -= b.data[1];
        a.data[2] -= b.data[2];
});

impl_op_ex!(*|a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3::new(
        a.data[0] * b.data[0],
        a.data[1] * b.data[1],
        a.data[2] * b.data[2],
    )
});

impl_op_ex_commutative!(*|a: &Vec3, b: &f32| -> Vec3 {
    Vec3::new(a.data[0] * b, a.data[1] * b, a.data[2] * b)
});

impl_op_ex!(*= |a: &mut Vec3, b: &Vec3| {
    a.data[0] *= b.data[0];
    a.data[1] *= b.data[1];
    a.data[2] *= b.data[2];
});

impl_op_ex!(*= |a: &mut Vec3, b: &f32| {
    a.data[0] *= b;
    a.data[1] *= b;
    a.data[2] *= b;
});

impl_op_ex!(/ |a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3::new(
        a.data[0] / b.data[0],
        a.data[1] / b.data[1],
        a.data[2] / b.data[2],
    )
});

impl_op_ex_commutative!(/ |a: &Vec3, b: &f32| -> Vec3 {
    Vec3::new(a.data[0] / b, a.data[1] / b, a.data[2] / b)
});

impl_op_ex!(/= |a: &mut Vec3, b: &Vec3| {
    a.data[0] /= b.data[0];
    a.data[1] /= b.data[1];
    a.data[2] /= b.data[2];
});

impl_op_ex!(/= |a: &mut Vec3, b: &f32| {
    a.data[0] /= b;
    a.data[1] /= b;
    a.data[2] /= b;
});

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.data[0], self.data[1], self.data[2])
    }
}
