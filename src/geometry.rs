use std::f32;

pub type Vec3f = (f32, f32, f32);

/// Element-wise subtraction of two vectors
#[macro_export]
macro_rules! vec_sub {
    ($a: expr, $b: expr) => {
        ($a.0 - $b.0, $a.1 - $b.1, $a.2 - $b.2)
    };
}

/// Element-wise addition of two vectors
#[macro_export]
macro_rules! vec_add {
    ($n: expr, $m: expr) => {
        ($n.0 + $m.0, $n.1 + $m.1, $n.2 + $m.2)
    };
}

/// Element-wise multiplication of two vectors
#[macro_export]
macro_rules! vec_mul {
    ($n: expr, $m: expr) => {
        ($n.0 * $m.0, $n.1 * $m.1, $n.2 * $m.2)
    };
}

/// Dot-product (sine of angle) of two vectors
#[macro_export]
macro_rules! dot {
    ($a: expr, $b: expr) => {
        $a.0 * $b.0 + $a.1 * $b.1 + $a.2 * $b.2
    };
}

/// Bar operator -- magnitude of a single vector
#[macro_export]
macro_rules! bar {
    ($v: expr) => {
        f32::sqrt($v.0 * $v.0 + $v.1 * $v.1 + $v.2 * $v.2)
    };
}

/// Mul a vector by a scalar
#[macro_export]
macro_rules! mul_scalar {
    ($a: expr, $b: expr) => {
        ($a.0 * $b, $a.1 * $b, $a.2 * $b)
    };
}


/// Normalize a vector - set its length to 1.
pub fn normalize(vector: Vec3f) -> Vec3f {
    let bar = bar!(vector);
    (vector.0 / bar, vector.1 / bar, vector.2 / bar)
}

pub struct Sphere {
    pub center: Vec3f,
    pub radius: f32,
}

impl Sphere {
    pub fn update_z(&mut self, z: f32) {
        self.center = (self.center.0, self.center.1, z);
    }

    /// Find the point-of-intersection if a ray (defined by origin and dir)
    /// intersects this sphere. If so, return the distance and normal at that point
    pub fn intersect(&self, origin: Vec3f, dir: Vec3f) -> Option<(f32, Vec3f)> {
        let u = vec_sub!(self.center, origin);
        let qp_0 = mul_scalar!(dir, dot!(dir, u) / bar!(dir));
        let qp = vec_add!(origin, qp_0);
        let dist = bar!(vec_sub!(self.center, qp));

        if dist > self.radius {
            None
        } else {
            let range =
                bar!(vec_sub!(qp, origin)) - f32::sqrt(self.radius * self.radius + dist * dist);
            let pt = vec_add!(origin, mul_scalar!(dir, range));
            let normal = normalize(vec_sub!(pt, self.center));
            Some((range, normal))
        }
    }
}
