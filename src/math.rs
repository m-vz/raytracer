pub mod aabb;
pub mod interval;
pub mod perlin;

pub fn clamp_repeating(x: f64) -> f64 {
    let mut rem = x % 1.0;
    
    if rem < 0.0 {
        rem += 1.0;
    }
    
    rem
}
