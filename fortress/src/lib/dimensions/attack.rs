use dimensions::{
    Damage,
    LrDirection,
};

pub struct Attack {
    pub damage: Damage,
    pub knockback_strength: f32,
    pub knockback_dir: LrDirection,
}
