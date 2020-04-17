use crate::{
    app::StatusOr,
    file::{
        ConfigWatcher,
        SimpleConfigManager,
    },
    items::{
        Item,
        ItemConfig,
        ItemId,
        ItemPickup,
    },
    physics::PhysicsSimulation,
    render::FullyIlluminatedSpriteRenderer,
};
use generational_slab::Slab;
use nalgebra::Point2;

pub struct ItemSystem {
    config_manager: SimpleConfigManager<ItemConfig>,
    items: Slab<Item>,
}

impl ItemSystem {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<ItemSystem> {
        let config_manager: SimpleConfigManager<ItemConfig> = SimpleConfigManager::from_config_resource(config_watcher, "item.conf")?;
        let items = {
            let config = config_manager.get();
            Slab::with_capacity(config.system_initial_capacity)
        };

        Ok(ItemSystem {
            config_manager,
            items
        })
    }

    pub fn pre_update(&mut self) {
        self.config_manager.update();
    }

    pub fn post_update(&mut self) {
        let collected_item_keys: Vec<_> = self.items
            .iter_mut()
            .filter_map(|(item_key, item)| {
                if !item.collected() {
                    return None;
                }
                Some(item_key)
            })
            .collect();

        for item_key in collected_item_keys.into_iter() {
            self.items.remove(item_key);
        }
    }

    pub fn queue_draw(&self, renderer: &mut FullyIlluminatedSpriteRenderer) {
        let config = self.config_manager.get();
        for (_key, item) in self.items.iter() {
            item.queue_draw(config, renderer);
        }
    }

    pub fn spawn_item(&mut self, item_pickup: ItemPickup, position: Point2<f64>, physics_sim: &mut PhysicsSimulation) {
        let config = self.config_manager.get();
        let item_entry = self.items.vacant_entry();
        let item_id = ItemId::from_key(item_entry.key());
        let item = Item::new(config, item_id, item_pickup, position, physics_sim);
        item_entry.insert(item);
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

    pub fn respawn(&mut self) {
        self.items.clear();
    }
}