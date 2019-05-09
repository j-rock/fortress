use crate::items::{
    ItemPickup,
    state::ItemBody,
};
use nalgebra::Point2;

pub struct ItemState {
    body: ItemBody,
    item_pickup: ItemPickup,
}

impl ItemState {
    pub fn new(body: ItemBody, item_pickup: ItemPickup) -> ItemState {
        ItemState {
            body,
            item_pickup,
        }
    }

    pub fn item_pickup(&self) -> ItemPickup {
        self.item_pickup
    }

    pub fn position(&self) -> Option<Point2<f64>> {
        self.body.position()
    }
}