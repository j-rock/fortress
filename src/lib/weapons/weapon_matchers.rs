use crate::{
    entities::Entity,
    physics::ContactMatcher,
    players::PlayerId,
    weapons::BulletId,
    world::WorldView,
};

pub struct WeaponMatchers;

impl WeaponMatchers {
    pub fn bullet_hit() -> ContactMatcher {
        ContactMatcher::new(Box::new(|entity1, entity2, _, world: &mut WorldView| {
            if let Entity::Bullet(player_id, bullet_id) = entity1 {
                Self::process_bullet(player_id, bullet_id, world);
            }

            if let Entity::Bullet(player_id, bullet_id) = entity2 {
                Self::process_bullet(player_id, bullet_id, world);
            }
        }))
    }

    fn process_bullet(player_id: PlayerId, bullet_id: BulletId, world: &mut WorldView) {
        world.players.player_mut(player_id).bullet_hit(bullet_id);
    }
}
