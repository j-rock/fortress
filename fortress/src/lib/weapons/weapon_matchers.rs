use crate::{
    entities::Entity,
    physics::ProximityMatcher,
    players::PlayerId,
    weapons::BulletId,
    world::WorldView,
};

pub struct WeaponMatchers;

impl WeaponMatchers {
    pub fn bullet_hit_proximity_matcher() -> ProximityMatcher {
        ProximityMatcher::new(Box::new(|proximity, world: &mut WorldView| {
            if proximity.curr_type.basically_touching() {
                Self::try_resolve_hit(proximity.entity1, proximity.entity2, world);
            }
        }))
    }

    fn try_resolve_hit(entity1: Entity, entity2: Entity, world: &mut WorldView) {
        if let Entity::Bullet(player_id, bullet_id) = entity1 {
            Self::bullet_hit_something(player_id, bullet_id, entity2, world);
        } else if let Entity::Bullet(player_id, bullet_id) = entity2 {
            Self::bullet_hit_something(player_id, bullet_id, entity1, world);
        }
    }

    fn bullet_hit_something(player_id: PlayerId, bullet_id: BulletId, something: Entity, world: &mut WorldView) {
        let bullet_direction = world.players.bullet_hit(player_id, bullet_id);

        match something {
            Entity::EnemyGenerator(generator_id) => {
                if let Some(attack) = world.players.bullet_attack(player_id, bullet_id, world.rng) {
                    world.enemies.enemy_generator_hit(world.audio, generator_id, attack, world.particles);
                }
            },
            Entity::Enemy(enemy_id) => {
                if let Some(attack) = world.players.bullet_attack(player_id, bullet_id, world.rng) {
                    world.enemies.enemy_hit(enemy_id, attack, bullet_direction, world.particles);
                }
            },
            Entity::Barrel(barrel_id) => {
                world.items.try_hit_barrel(barrel_id, world.particles);
            },
            _ => {}
        }
    }
}
