use maps::{
    Map,
    state::MapBody,
};

pub struct MapState {
    pub body: MapBody,
}

impl MapState {
    pub fn new(body: MapBody) -> MapState {
        MapState {
            body
        }
    }

    pub fn register(&mut self, map: *const Map) {
        self.body.register(map);
    }
}
