use crate::dimensions::GridIndex;

#[derive(Clone, Deserialize)]
pub struct MapFileCell {
    pub q: i64,
    pub r: i64,
    pub height: f32,
    pub elevation: f32,
    pub rgba_color: (f32, f32, f32, f32),
    spawn: Option<bool>,
}

impl MapFileCell {
    pub fn is_spawn(&self) -> bool {
        match self.spawn {
            Some(true) => true,
            _ => false,
        }
    }

    pub fn grid_index(&self) -> GridIndex {
        GridIndex::new(self.q, self.r)
    }
}

#[derive(Clone, Deserialize)]
pub struct MapFile {
    pub cells: Vec<MapFileCell>
}