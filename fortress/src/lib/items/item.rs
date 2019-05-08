use crate::{
    dimensions::LrDirection,
    items::{
        ItemConfig,
        ItemId,
        ItemType,
        state::{
            ItemBody,
            ItemState,
            ItemStateMachine,
        }
    },
    physics::PhysicsSimulation,
    render::LightDependentSpriteRenderer,
};
use nalgebra::Point2;

pub struct Item {
    item_state: ItemState,
    item_state_machine: ItemStateMachine,
}

impl Item {
    pub fn new(config: &ItemConfig, item_id: ItemId, item_type: ItemType, spawn: Point2<f64>, facing_dir: LrDirection, physics_sim: &mut PhysicsSimulation) -> Item {
        let item_body = ItemBody::new(config, item_id, spawn, physics_sim);
        let item_state = ItemState::new(item_body, item_type, facing_dir);
        let item_state_machine = ItemStateMachine::default();
        Item {
            item_state,
            item_state_machine
        }
    }

    pub fn pre_update(&mut self) {
        if let Some(item_state_machine) = self.item_state_machine.pre_update(&self.item_state) {
            self.item_state_machine = item_state_machine;
        }
    }

    pub fn post_update(&mut self) {
        if let Some(item_state_machine) = self.item_state_machine.post_update(&self.item_state) {
            self.item_state_machine = item_state_machine;
        }
    }

    pub fn queue_draw(&self, config: &ItemConfig, sprite_renderer: &mut LightDependentSpriteRenderer) {
        self.item_state_machine.queue_draw(config, &self.item_state, sprite_renderer);
    }

    pub fn collect(&mut self) {
        self.item_state_machine.collect();
    }

    pub fn collected(&self) -> bool {
        self.item_state_machine.collected()
    }

    pub fn item_type(&self) -> ItemType {
        self.item_state_machine.item_type(&self.item_state)
    }
}
