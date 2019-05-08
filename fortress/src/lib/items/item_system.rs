use crate::{
    app::StatusOr,
    dimensions::LrDirection,
    file::{
        ConfigWatcher,
        SimpleConfigManager,
    },
    items::{
        Item,
        ItemConfig,
        ItemId,
        ItemType,
    },
    physics::PhysicsSimulation,
    render::LightDependentSpriteRenderer,
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
            Slab::with_capacity(config.items_system_initial_capacity)
        };

        Ok(ItemSystem {
            config_manager,
            items
        })
    }

    pub fn pre_update(&mut self) {
        for (_key, item) in self.items.iter_mut() {
            item.pre_update();
        }
    }

    pub fn post_update(&mut self) {
        let collected_item_keys: Vec<_> = self.items
            .iter_mut()
            .filter_map(|(item_key, item)| {
                item.post_update();
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

    pub fn queue_draw(&self, light_dependent: &mut LightDependentSpriteRenderer) {
        let config = self.config_manager.get();
        for (_key, item) in self.items.iter() {
            item.queue_draw(config, light_dependent);
        }
    }

    pub fn spawn_item(&mut self, item_type: ItemType, position: Point2<f64>, facing_dir: LrDirection, physics_sim: &mut PhysicsSimulation) {
        let config = self.config_manager.get();
        let item_entry = self.items.vacant_entry();
        let item_id = ItemId::from_key(item_entry.key());
        let item = Item::new(config, item_id, item_type, position, facing_dir, physics_sim);
        item_entry.insert(item);
    }

    pub fn collect(&mut self, item_id: ItemId) -> Option<ItemType> {
        self.items.get_mut(item_id.key())
            .and_then(|item| {
                if item.collected() {
                    return None;
                }
                item.collect();
                Some(item.item_type())
            })
    }

    pub fn respawn(&mut self) {
        self.items.clear();
    }
}