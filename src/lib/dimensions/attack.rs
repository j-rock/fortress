use crate::dimensions::Damage;
use nalgebra::Vector2;

pub struct Attack {
    pub damage: Damage,
    pub knockback_strength: f64,
    pub knockback_dir: Vector2<f64>,
}
