use crate::hit::Hit;

pub struct Scene {
    pub objects: Vec<Box<dyn Hit>>,
}
