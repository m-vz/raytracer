use rand::seq::SliceRandom;

use crate::vec::Vec3;

const POINT_COUNT: u32 = 256;
const MAX_INDEX: i64 = POINT_COUNT as i64 - 1;

pub struct PerlinNoise {
    points: Vec<Vec3>,
    permutations: (Vec<usize>, Vec<usize>, Vec<usize>),
}

impl PerlinNoise {
    pub fn new() -> Self {
        let mut points = Vec::with_capacity(POINT_COUNT as usize);
        let mut permutations: (Vec<usize>, Vec<usize>, Vec<usize>) = (
            (0..POINT_COUNT as usize).collect(),
            (0..POINT_COUNT as usize).collect(),
            (0..POINT_COUNT as usize).collect(),
        );

        for _ in 0..POINT_COUNT {
            points.push(Vec3::random_unit_vector());
        }
        permutations.0.shuffle(&mut rand::thread_rng());
        permutations.1.shuffle(&mut rand::thread_rng());
        permutations.2.shuffle(&mut rand::thread_rng());

        Self {
            points,
            permutations,
        }
    }

    pub fn noise(&self, point: Vec3) -> f64 {
        let mut noise = 0.0;
        let floor = point.floored();
        let offset = point - floor;
        let offset_smoothed = offset * offset * (Vec3(3.0, 3.0, 3.0) - 2.0 * offset);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                    let indices = (
                        (floor.0 as i64 + i) & MAX_INDEX,
                        (floor.1 as i64 + j) & MAX_INDEX,
                        (floor.2 as i64 + k) & MAX_INDEX,
                    );
                    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                    let sample = self.points[self.permutations.0[indices.0 as usize]
                        ^ self.permutations.1[indices.1 as usize]
                        ^ self.permutations.2[indices.2 as usize]];
                    #[allow(clippy::cast_precision_loss)]
                    let weights = Vec3(i as f64, j as f64, k as f64);
                    let surflets = [
                        weights.0.mul_add(
                            offset_smoothed.0,
                            (1.0 - weights.0) * (1.0 - offset_smoothed.0),
                        ),
                        weights.1.mul_add(
                            offset_smoothed.1,
                            (1.0 - weights.1) * (1.0 - offset_smoothed.1),
                        ),
                        weights.2.mul_add(
                            offset_smoothed.2,
                            (1.0 - weights.2) * (1.0 - offset_smoothed.2),
                        ),
                    ];

                    noise +=
                        surflets.into_iter().product::<f64>() * sample.dot(&(offset - weights));
                }
            }
        }

        noise
    }

    pub fn turbulence(&self, mut point: Vec3, depth: u32) -> f64 {
        let mut noise = 0.0;
        let mut weight = 1.0;

        for _ in 0..depth {
            noise += weight * self.noise(point);
            weight *= 0.5;
            point *= 2.0;
        }

        noise.abs()
    }
}
