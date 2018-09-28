use app::StatusOr;
use buff::{
    Buff,
    BuffBox,
    BuffBoxPlacement,
    BuffConfig,
};
use control::{
    ControlEvent,
    Controller,
    ControllerId,
};
use file::{
    ConfigWatcher,
    SimpleConfigManager,
};
use liquidfun::box2d::common::math::Vec2;
use physics::PhysicsSimulation;
use render::BoxRenderer;
use slab::Slab;

pub struct BuffSystem {
    config_manager: SimpleConfigManager<BuffConfig>,
    buffs: Slab<BuffBox>,
    buff_locations: Vec<Vec2>,
}

impl BuffSystem {
    pub fn new(config_watcher: &mut ConfigWatcher, buff_locations: Vec<Vec2>, physics_sim: &mut PhysicsSimulation) -> StatusOr<BuffSystem> {
        let config_manager: SimpleConfigManager<BuffConfig> = SimpleConfigManager::from_config_resource(config_watcher, "buff.conf")?;
        let mut buffs = BuffSystem {
            config_manager,
            buffs: Slab::with_capacity(buff_locations.len()),
            buff_locations,
        };
        buffs.redeploy(physics_sim);
        Ok(buffs)
    }

    pub fn pre_update(&mut self, controller: &Controller, physics_sim: &mut PhysicsSimulation) {
        if self.config_manager.update() || controller.just_pressed(ControllerId::Keyboard, ControlEvent::RedeployEntities) {
            self.redeploy(physics_sim);
        }
    }

    pub fn post_update(&mut self, physics_sim: &mut PhysicsSimulation) {
        let config = self.config_manager.get();
        for (_i, buff_box) in self.buffs.iter_mut() {
            buff_box.post_update(&config.buff_drop, physics_sim);
        }
    }

    pub fn draw(&self, box_renderer: &mut BoxRenderer) {
        let config = self.config_manager.get();
        for (_i, buff_box) in self.buffs.iter() {
            buff_box.draw(config, box_renderer);
        }
    }

    pub fn respawn(&mut self, buff_locations: Vec<Vec2>, physics_sim: &mut PhysicsSimulation) {
        self.buff_locations = buff_locations;
        self.redeploy(physics_sim);
    }

    fn redeploy(&mut self, physics_sim: &mut PhysicsSimulation) {
        self.buffs.clear();
        self.buffs.reserve(self.buff_locations.len());

        let config = self.config_manager.get();
        for location in self.buff_locations.iter() {
            let placement = BuffBoxPlacement {
                buff: Buff::JumpStrength,
                location: *location,
            };
            let buff_box = BuffBox::new(&config.buff_box, &placement, physics_sim);
            let idx = self.buffs.insert(buff_box);
            self.buffs.get_mut(idx).expect("BuffSystem has bad key!").register();
        }
    }
}