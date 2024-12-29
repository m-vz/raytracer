use std::sync::Arc;

use crate::hit::bvh::Node;
use crate::hit::quad::Quad;
use crate::hit::{Hit, HitResult};
use crate::material::Material;
use crate::math::aabb::Aabb;
use crate::math::interval::Interval;
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Box {
    sides: Node,
}

impl Box {
    pub fn new(a: Vec3, b: Vec3, material: Arc<dyn Material>) -> Self {
        let size = b - a;

        Self {
            sides: Node::new(vec![
                // front
                Arc::new(Quad::new(a, size.x(), size.y(), material.clone())),
                // right
                Arc::new(Quad::new(
                    a + size.x(),
                    size.z(),
                    size.y(),
                    material.clone(),
                )),
                // back
                Arc::new(Quad::new(
                    a + size.xz(),
                    -size.x(),
                    size.y(),
                    material.clone(),
                )),
                // left
                Arc::new(Quad::new(
                    a + size.z(),
                    -size.z(),
                    size.y(),
                    material.clone(),
                )),
                // top
                Arc::new(Quad::new(
                    a + size.y(),
                    size.x(),
                    size.z(),
                    material.clone(),
                )),
                // bottom
                Arc::new(Quad::new(a + size.z(), size.x(), -size.z(), material)),
            ]),
        }
    }
}

impl Hit for Box {
    fn hit(&self, ray: &Ray, t_interval: Interval) -> Option<HitResult> {
        self.sides.hit(ray, t_interval)
    }

    fn bounding_box(&self) -> &Aabb {
        self.sides.bounding_box()
    }
}
