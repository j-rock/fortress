use crate::buffs::Buff;
use liquidfun::box2d::common::math::Vec2;

#[derive(Clone, Deserialize)]
pub struct BuffBoxConfig {
    pub size: (f32, f32),
    pub density: f32,
    pub friction: f32,
}

#[derive(Clone, Deserialize)]
pub struct BuffDropConfig {
    pub start_position: (f32, f32),
    pub velocity: (f32, f32),
    pub size: (f32, f32),
    pub density: f32,
    pub friction: f32,
    pub restitution: f32,
}

pub struct BuffBoxPlacement {
    pub buff: Buff,
    pub location: Vec2,
}

#[derive(Clone, Deserialize)]
pub struct BuffConfig {
    pub buff_box: BuffBoxConfig,
    pub buff_drop: BuffDropConfig,
}
