use crate::{
    app::StatusOr,
    control::{
        ControlEvent,
        Controller,
        ControllerId,
    },
    file::{
        ConfigWatcher,
        SimpleConfigManager,
    },
    physics::PhysicsSimulation,
    render::{
        FullyIlluminatedSpriteRenderer,
        LightDependentSpriteRenderer,
        PointLight,
    },
    treasures::{
        TreasureChest,
        TreasureChestId,
        TreasureConfig,
    },
    world::RandGen
};
use generational_slab::Slab;
use nalgebra::Point2;

pub struct TreasureSystem {
    config: SimpleConfigManager<TreasureConfig>,
    treasure_chests: Slab<TreasureChest>,
    cached_locations: Vec<Point2<f64>>,
}

impl TreasureSystem {
    pub fn new(config_watcher: &mut ConfigWatcher, locations: Vec<Point2<f64>>, rng: &mut RandGen, physics_sim: &mut PhysicsSimulation) -> StatusOr<TreasureSystem> {
        let config: SimpleConfigManager<TreasureConfig> = SimpleConfigManager::from_config_resource(config_watcher, "treasure.conf")?;
        let mut treasures = TreasureSystem {
            config,
            treasure_chests: Slab::with_capacity(locations.len()),
            cached_locations: locations,
        };
        treasures.redeploy(rng, physics_sim);
        Ok(treasures)
    }

    pub fn pre_update(&mut self, controller: &Controller, rng: &mut RandGen, physics_sim: &mut PhysicsSimulation) {
        if self.config.update() || controller.just_pressed(ControllerId::Keyboard, ControlEvent::RedeployEntities) {
            self.redeploy(rng, physics_sim);
        }
    }

    pub fn post_update(&mut self, physics_sim: &mut PhysicsSimulation) {
        let config = self.config.get();
        for (_i, treasure_chest) in self.treasure_chests.iter_mut() {
            treasure_chest.post_update(config, physics_sim);
        }
    }

    pub fn populate_lights(&self, lights: &mut Vec<PointLight>) {
        for (_i, treasure_chest) in self.treasure_chests.iter() {
            treasure_chest.populate_lights(lights);
        }
    }

    pub fn queue_draw(&self, full_light_sprite: &mut FullyIlluminatedSpriteRenderer, light_dependent_sprite: &mut LightDependentSpriteRenderer) {
        let config = self.config.get();
        for (_i, treasure_chest) in self.treasure_chests.iter() {
            treasure_chest.queue_draw(config, full_light_sprite, light_dependent_sprite);
        }
    }

    pub fn respawn(&mut self, locations: Vec<Point2<f64>>, rng: &mut RandGen, physics_sim: &mut PhysicsSimulation) {
        self.cached_locations = locations;
        self.redeploy(rng, physics_sim);
    }

    fn redeploy(&mut self, rng: &mut RandGen, physics_sim: &mut PhysicsSimulation) {
        self.treasure_chests.clear();
        self.treasure_chests.reserve(self.cached_locations.len());

        let config = self.config.get();
        for location in self.cached_locations.iter() {
            let vacant_entry = self.treasure_chests.vacant_entry();
            let chest_id = TreasureChestId::new(vacant_entry.key());
            let treasure_chest = TreasureChest::new(config, chest_id, *location, rng, physics_sim);
            vacant_entry.insert(treasure_chest);
        }
    }
}
