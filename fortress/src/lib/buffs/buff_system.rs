use app::StatusOr;
use buffs::{
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
use rand::Rng;
use render::BoxRenderer;
use slab::Slab;
use world::RandGen;

pub struct BuffSystem {
    config_manager: SimpleConfigManager<BuffConfig>,
    buffs: Slab<BuffBox>,
    buff_locations: Vec<Vec2>,
}

impl BuffSystem {
    pub fn new(config_watcher: &mut ConfigWatcher, buff_locations: Vec<Vec2>, rng: &mut RandGen, physics_sim: &mut PhysicsSimulation) -> StatusOr<BuffSystem> {
        let config_manager: SimpleConfigManager<BuffConfig> = SimpleConfigManager::from_config_resource(config_watcher, "buff.conf")?;
        let mut buffs = BuffSystem {
            config_manager,
            buffs: Slab::with_capacity(buff_locations.len()),
            buff_locations,
        };
        buffs.redeploy(rng, physics_sim);
        Ok(buffs)
    }

    pub fn pre_update(&mut self, controller: &Controller, rng: &mut RandGen, physics_sim: &mut PhysicsSimulation) {
        if self.config_manager.update() || controller.just_pressed(ControllerId::Keyboard, ControlEvent::RedeployEntities) {
            self.redeploy(rng, physics_sim);
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

    pub fn respawn(&mut self, buff_locations: Vec<Vec2>, rng: &mut RandGen, physics_sim: &mut PhysicsSimulation) {
        self.buff_locations = buff_locations;
        self.redeploy(rng, physics_sim);
    }

    fn redeploy(&mut self, rng: &mut RandGen, physics_sim: &mut PhysicsSimulation) {
        self.buffs.clear();
        self.buffs.reserve(self.buff_locations.len());

        let config = self.config_manager.get();
        for location in self.buff_locations.iter() {
            let placement = BuffBoxPlacement {
                buff: rng.rng.gen(),
                location: Vec2 {
                    x: location.x + config.buff_box.size.0 / 2.0,
                    y: location.y - config.buff_box.size.1 / 2.0,
                }
            };
            let buff_box = BuffBox::new(&config.buff_box, &placement, physics_sim);
            let idx = self.buffs.insert(buff_box);
            self.buffs.get_mut(idx).expect("BuffSystem has bad key!").register();
        }
    }
}