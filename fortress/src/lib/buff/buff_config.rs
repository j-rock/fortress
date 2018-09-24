use buff::Buff;

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

#[derive(Clone, Deserialize)]
pub struct BuffBoxPlacement {
    pub buff: Buff,
    pub location: (f32, f32),
}

#[derive(Clone, Deserialize)]
pub struct BuffConfig {
    pub buff_box: BuffBoxConfig,
    pub buff_drop: BuffDropConfig,
    pub buffs: Vec<BuffBoxPlacement>,
}
