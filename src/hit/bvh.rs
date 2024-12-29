use std::cmp::Ordering;
use std::sync::Arc;

use rand::Rng;

use crate::hit::{Hit, HitResult};
use crate::math::aabb::Aabb;
use crate::math::interval::Interval;
use crate::ray::Ray;

pub struct BvhNode {
    left: Arc<dyn Hit>,
    right: Arc<dyn Hit>,
    bounding_box: Aabb,
}

impl BvhNode {
    pub fn new(mut objects: Vec<Arc<dyn Hit>>) -> Self {
        let axis = rand::thread_rng().gen_range(0..3);
        let comparator = match axis {
            0 => Self::box_compare_x,
            1 => Self::box_compare_y,
            2 => Self::box_compare_z,
            _ => unreachable!(),
        };
        let left;
        let right;

        match objects.len() {
            1 => {
                left = objects.swap_remove(0);
                right = left.clone();
            }
            2 => {
                let first = objects.swap_remove(0);
                let second = objects.swap_remove(0);

                if comparator(&first, &second) == Ordering::Less {
                    left = first;
                    right = second;
                } else {
                    left = second;
                    right = first;
                }
            }
            _ => {
                objects.sort_unstable_by(comparator);

                right = Arc::new(Self::new(objects.split_off(objects.len() / 2)));
                left = Arc::new(Self::new(objects));
            }
        }

        Self {
            bounding_box: left.bounding_box().combined(right.bounding_box()),
            left,
            right,
        }
    }

    fn box_compare(a: &Arc<dyn Hit>, b: &Arc<dyn Hit>, axis: u32) -> Ordering {
        a.bounding_box()
            .axis(axis)
            .start()
            .total_cmp(&b.bounding_box().axis(axis).start())
    }

    fn box_compare_x(a: &Arc<dyn Hit>, b: &Arc<dyn Hit>) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_compare_y(a: &Arc<dyn Hit>, b: &Arc<dyn Hit>) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_compare_z(a: &Arc<dyn Hit>, b: &Arc<dyn Hit>) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hit for BvhNode {
    fn hit(&self, ray: &Ray, mut t_interval: Interval) -> Option<HitResult> {
        if !self.bounding_box.hit(ray, &mut t_interval) {
            return None;
        }

        let left = self.left.hit(ray, t_interval.clone());
        let right = self.right.hit(
            ray,
            Interval(
                t_interval.start()..if let Some(left) = &left {
                    left.t
                } else {
                    t_interval.end()
                },
            ),
        );

        right.or(left)
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bounding_box
    }
}
