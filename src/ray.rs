use crate::vec::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f64,
}

#[allow(dead_code)]
impl Ray {
    pub fn look_at(origin: Vec3, target: Vec3, time: f64) -> Self {
        Self {
            origin,
            direction: origin.look_at(&target),
            time,
        }
    }

    pub fn normalized(&self) -> Self {
        Self {
            origin: self.origin,
            direction: self.direction.normalized(),
            time: self.time,
        }
    }

    pub fn normalize(&mut self) {
        self.direction.normalize();
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use crate::vec::Vec3;

    use super::Ray;

    #[test]
    fn at() {
        let ray = Ray {
            origin: Vec3(0.0, 1.0, 3.0),
            direction: Vec3(1.0, 2.0, 0.0),
            time: 0.0,
        };
        let t = 2.0;

        assert_abs_diff_eq!(ray.at(t), Vec3(2.0, 5.0, 3.0));
    }
}
