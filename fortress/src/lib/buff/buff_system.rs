use app::StatusOr;
use buff::{
    BuffBox,
    BuffConfig,
};
use control::{
    ControlEvent,
    ControlId,
    Controller,
};
use entity::EntityRegistrar;
use file::{
    ConfigWatcher,
    SimpleConfigManager,
};
use physics::PhysicsSimulation;
use render::BoxRenderer;
use slab::Slab;

pub struct BuffSystem {
    config_manager: SimpleConfigManager<BuffConfig>,
    buffs: Slab<BuffBox>,
}

impl BuffSystem {
    pub fn new(config_watcher: &mut ConfigWatcher, physics_sim: &mut PhysicsSimulation) -> StatusOr<BuffSystem> {
        let config_manager = SimpleConfigManager::new(config_watcher, "buff.conf")?;
        let mut buffs = BuffSystem {
            config_manager,
            buffs: Slab::new(),
        };
        buffs.redeploy(physics_sim);
        buffs
    }

    pub fn pre_update(&mut self, controller: &Controller, physics_sim: &mut PhysicsSimulation) {
        if self.config_manager.update() || controller.just_pressed(ControllerId::Keyboard, ControlEvent::RespawnEntities) {
            self.redeploy(physics_sim);
        }
    }

    pub fn draw(&self, box_renderer: &mut BoxRenderer) {
        self.buffs.iter().foreach(|buff_box| buff_box.draw(box_renderer));
    }

    fn redeploy(&mut self, physics_sim: &mut PhysicsSimulation) {
        self.buffs.clear();

        let config = self.config_manager.get();
        for placement in config.buffs.iter() {
            let buff_box = BuffBox::new(config, placement, physics_sim);
            let idx = self.buffs.insert(buff_box);
            self.buffs.get_mut(idx).expect("BuffSystem has bad key!").register();
        }
    }
}