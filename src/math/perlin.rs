use rand::seq::SliceRandom;

use crate::vec::Vec3;

const POINT_COUNT: usize = 256;
const MAX_INDEX: i32 = POINT_COUNT as i32 - 1;

pub struct PerlinNoise {
    points: Vec<Vec3>,
    permutations: (Vec<usize>, Vec<usize>, Vec<usize>),
}

impl PerlinNoise {
    pub fn new() -> Self {
        let mut points = Vec::with_capacity(POINT_COUNT);
        let mut permutations: (Vec<usize>, Vec<usize>, Vec<usize>) = (
            (0..POINT_COUNT).collect(),
            (0..POINT_COUNT).collect(),
            (0..POINT_COUNT).collect(),
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
                    let sample = self.points[self.permutations.0
                        [((floor.0 as i32 + i) & MAX_INDEX) as usize]
                        ^ self.permutations.1[((floor.1 as i32 + j) & MAX_INDEX) as usize]
                        ^ self.permutations.2[((floor.2 as i32 + k) & MAX_INDEX) as usize]];
                    let ijk = Vec3(i as f64, j as f64, k as f64);

                    noise += (ijk.0 * offset_smoothed.0
                        + (1.0 - ijk.0) * (1.0 - offset_smoothed.0))
                        * (ijk.1 * offset_smoothed.1 + (1.0 - ijk.1) * (1.0 - offset_smoothed.1))
                        * (ijk.2 * offset_smoothed.2 + (1.0 - ijk.2) * (1.0 - offset_smoothed.2))
                        * sample.dot(&(offset - ijk));
                }
            }
        }

        noise
    }
}
