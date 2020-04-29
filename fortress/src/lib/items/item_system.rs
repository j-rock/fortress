use crate::{
    app::StatusOr,
    file::{
        ConfigWatcher,
        SimpleConfigManager,
    },
    items::{
        barrels::{
            Barrel,
            BarrelId,
        },
        Item,
        ItemConfig,
        ItemId,
        ItemPickup,
    },
    math::RandGen,
    particles::ParticleSystem,
    physics::PhysicsSimulation,
    render::{
        FullyIlluminatedSpriteRenderer,
        PointLights,
    },
};
use generational_slab::Slab;
use nalgebra::Point2;

pub struct ItemSystem {
    config_manager: SimpleConfigManager<ItemConfig>,
    items: Slab<Item>,
    barrels: Slab<Barrel>,
}

impl ItemSystem {
    pub fn new(config_watcher: &mut ConfigWatcher,
               barrel_positions: &[Point2<f64>],
               rng: &mut RandGen,
               physics_sim: &mut PhysicsSimulation) -> StatusOr<Self> {
        let config_manager: SimpleConfigManager<ItemConfig> = SimpleConfigManager::from_config_resource(config_watcher, "item.conf")?;
        let (items, barrels) = {
            let config = config_manager.get();
            let items = Slab::with_capacity(config.system_items_initial_capacity);
            let barrels = Slab::with_capacity(config.system_barrels_initial_capacity);
            (items, barrels)
        };

        let mut item_system = ItemSystem {
            config_manager,
            items,
            barrels,
        };
        item_system.respawn(barrel_positions, rng, physics_sim);

        Ok(item_system)
    }

    pub fn pre_update(&mut self) {
        self.config_manager.update();
        self.barrels
            .iter_mut()
            .for_each(|(_idx, barrel)| {
                barrel.pre_update();
            });
    }

    pub fn post_update(&mut self) {
        self.items.retain(|item| {
            !item.collected()
        });

        self.barrels.retain(|barrel| {
            !barrel.is_expired()
        });
    }

    pub fn populate_lights(&self, point_lights: &mut PointLights) {
        let config = self.config_manager.get();
        let lights = self.items
            .iter()
            .filter_map(|(_key, item)| {
                item.point_light(config)
            });
        point_lights.append(lights);
    }

    pub fn queue_draw(&self, full_light: &mut FullyIlluminatedSpriteRenderer) {
        let config = self.config_manager.get();
        for (_key, item) in self.items.iter() {
            item.queue_draw(config, full_light);
        }
        for (_key, barrel) in self.barrels.iter() {
            barrel.queue_draw(&config.barrel, full_light);
        }
    }

    pub fn spawn_item(&mut self, item_pickup: ItemPickup, position: Point2<f64>, physics_sim: &mut PhysicsSimulation) {
        let config = self.config_manager.get();
        let item_entry = self.items.vacant_entry();
        let item_id = ItemId::from_key(item_entry.key());
        let item = Item::new(config, item_id, item_pickup, position.clone(), physics_sim);
        item_entry.insert(item);
    }

    pub fn spawn_barrel(&mut self, position: Point2<f64>, rng: &mut RandGen, physics_sim: &mut PhysicsSimulation) {
        let config = self.config_manager.get();
        let barrel_entry = self.barrels.vacant_entry();
        let barrel_id = BarrelId::from_key(barrel_entry.key());
        let barrel = Barrel::new(&config.barrel, barrel_id, position, rng, physics_sim);
        barrel_entry.insert(barrel);
    }

    pub fn collect(&mut self, item_id: ItemId) -> Option<ItemPickup> {
        self.items.get_mut(item_id.key())
            .and_then(|item| {
                if item.collected() {
                    return None;
                }
                item.collect();
                Some(item.item_pickup())
            })
    }

    pub fn try_hit_barrel(&mut self,
                          barrel_id: BarrelId,
                          particles: &mut ParticleSystem,
                          rng: &mut RandGen) {
        if let Some(barrel) = self.barrels.get_mut(barrel_id.key()) {
            let config = self.config_manager.get();
            barrel.strike(&config.barrel, particles, rng);
        }
    }

    pub fn respawn(&mut self, barrel_positions: &[Point2<f64>], rng: &mut RandGen, physics_sim: &mut PhysicsSimulation) {
        self.items.clear();
        self.barrels.clear();

        barrel_positions.iter().for_each(|position| {
            self.spawn_barrel(position.clone(), rng, physics_sim);
        });
    }

    pub fn config(&self) -> &ItemConfig {
        self.config_manager.get()
    }
}