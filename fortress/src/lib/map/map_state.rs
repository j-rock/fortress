use map::{
    Map,
    MapConfig,
    state::MapBody,
};

pub struct MapState {
    pub config: MapConfig,
    pub body: MapBody,
}

impl MapState {
    pub fn new(config: MapConfig, body: MapBody) -> MapState {
        MapState {
            config,
            body
        }
    }

    pub fn register(&mut self, map: *const Map) {
        self.body.register(map);
    }
}
