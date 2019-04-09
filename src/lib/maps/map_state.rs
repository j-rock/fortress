use crate::maps::{
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
}
