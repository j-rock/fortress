use crate::{
    audio::Sound,
    items::ItemId,
    entities::Entity,
    physics::ProximityMatcher,
    players::PlayerId,
    world::WorldView,
};

pub struct PlayerMatchers;

impl PlayerMatchers {
    pub fn player_collected_item() -> ProximityMatcher {
        ProximityMatcher::new(Box::new(|proximity, world: &mut WorldView| {
            if proximity.curr_type.basically_touching() {
                if let (Entity::Item(item_id), Entity::Player(player_id)) = (proximity.entity1, proximity.entity2) {
                    Self::process_player_collected_item(item_id, player_id, world);
                }
                if let (Entity::Item(item_id), Entity::Player(player_id)) = (proximity.entity2, proximity.entity1) {
                    Self::process_player_collected_item(item_id, player_id, world);
                }
            }
        }))
    }

    fn process_player_collected_item(item_id: ItemId, player_id: PlayerId, world: &mut WorldView) {
        if let Some(item_pickup) = world.items.collect(item_id) {
            world.audio.play_sound(Sound::CollectItem);
            world.players.collect_item(player_id, item_pickup);
        }
    }
}
