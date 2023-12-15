use crate::vec::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

#[allow(dead_code)]
impl Ray {
    pub fn look_at(origin: Vec3, target: Vec3) -> Self {
        Self {
            origin,
            direction: (target - origin).normalized(),
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::Ray;
    use crate::vec::Vec3;
    use float_cmp::assert_approx_eq;

    #[test]
    fn at() {
        let ray = Ray {
            origin: Vec3(0.0, 1.0, 3.0),
            direction: Vec3(1.0, 2.0, 0.0),
        };
        let t = 2.0;

        assert_approx_eq!(Vec3, ray.at(t), Vec3(2.0, 5.0, 3.0));
    }
}
