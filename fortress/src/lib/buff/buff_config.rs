use buff::Buff;

#[derive(Clone, Deserialize)]
pub struct BuffBoxPlacement {
    pub buff: Buff,
    pub location: (f32, f32),
}

#[derive(Clone, Deserialize)]
pub struct BuffConfig {
    pub buff_box_size: (f32, f32),
    pub density: f32,
    pub friction: f32,
    pub buffs: Vec<BuffBoxPlacement>,
}
