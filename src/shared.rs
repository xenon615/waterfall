use bevy::prelude::*;
#[allow(dead_code)]
fn rand_sign() -> f32{
    let r = fastrand::f32();
    2. * if r > 0.5 {  r - 1. } else { r} 
}

#[allow(dead_code)]
pub fn random_pos(base: Vec3, quant: f32) -> Vec3 {
    base + Vec3::Z * rand_sign() * quant + Vec3::X * rand_sign() * quant
} 

#[allow(dead_code)]
pub fn fibonacci_sphere(count: usize) -> impl Iterator<Item = Vec3> {
    let phi = std::f32::consts::PI * (5.0_f32.sqrt() - 1.);
    (0 .. count).map(move |i| {
        let y = 1. - (i as f32 / (count - 1) as f32) * 2.;  
        let radius = (1. - y * y).sqrt();
        let theta = phi * i as f32;
        let x = theta.cos() * radius;
        let z = theta.sin() * radius;
        Vec3::new(x, y, z)
    })
} 
