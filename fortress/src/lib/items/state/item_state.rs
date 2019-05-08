use crate::{
    dimensions::LrDirection,
    items::{
        ItemType,
        state::ItemBody,
    }
};
use nalgebra::Point2;

pub struct ItemState {
    body: ItemBody,
    item_type: ItemType,
    facing_dir: LrDirection,
}

impl ItemState {
    pub fn new(body: ItemBody, item_type: ItemType, facing_dir: LrDirection) -> ItemState {
        ItemState {
            body,
            item_type,
            facing_dir,
        }
    }

    pub fn item_type(&self) -> ItemType {
        self.item_type
    }

    pub fn position(&self) -> Option<Point2<f64>> {
        self.body.position()
    }

    pub fn facing_dir(&self) -> LrDirection {
        self.facing_dir
    }
}