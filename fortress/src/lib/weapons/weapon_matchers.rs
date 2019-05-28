use crate::{
    enemies::{
        EnemyId,
        EnemyGeneratorId,
    },
    entities::Entity,
    physics::{
        Contact,
        ContactMatcher,
        ProximityMatcher,
    },
    players::PlayerId,
    weapons::BulletId,
    world::WorldView,
};

pub struct WeaponMatchers;

impl WeaponMatchers {
    pub fn bullet_hit_proximity_matcher() -> ProximityMatcher {
        ProximityMatcher::new(Box::new(|proximity, world: &mut WorldView| {
            if proximity.curr_type.basically_touching() {
                if let (Entity::Bullet(player_id, bullet_id), Entity::EnemyGenerator(generator_id)) = (proximity.entity1, proximity.entity2) {
                    Self::process_bullet_hit(player_id, bullet_id, world);
                    Self::bullet_hit_enemy_generator(player_id, bullet_id, generator_id, world);
                }
                if let (Entity::Bullet(player_id, bullet_id), Entity::EnemyGenerator(generator_id)) = (proximity.entity2, proximity.entity1) {
                    Self::process_bullet_hit(player_id, bullet_id, world);
                    Self::bullet_hit_enemy_generator(player_id, bullet_id, generator_id, world);
                }
            }
        }))
    }

    pub fn bullet_hit_contact_matcher() -> ContactMatcher {
        ContactMatcher::new(Box::new(|contact, world: &mut WorldView| {
            if let Contact::Started(entity1, entity2) = contact {
                if let Entity::Bullet(player_id, bullet_id) = entity1 {
                    Self::process_bullet_hit(player_id, bullet_id, world);
                    if let Entity::Enemy(enemy_id) = entity2 {
                        Self::bullet_hit_enemy(player_id, bullet_id, enemy_id, world);
                    }
                }

                if let Entity::Bullet(player_id, bullet_id) = entity2 {
                    Self::process_bullet_hit(player_id, bullet_id, world);
                    if let Entity::Enemy(enemy_id) = entity1 {
                        Self::bullet_hit_enemy(player_id, bullet_id, enemy_id, world);
                    }
                }
            }
        }))
    }

    fn process_bullet_hit(player_id: PlayerId, bullet_id: BulletId, world: &mut WorldView) {
        world.players.bullet_hit(player_id, bullet_id);
    }

    fn bullet_hit_enemy(player_id: PlayerId, bullet_id: BulletId, enemy_id: EnemyId, world: &mut WorldView) {
        if let Some(attack) = world.players.bullet_attack(player_id, bullet_id) {
            world.enemies.enemy_hit(enemy_id, attack, world.particles);
        }
    }

    fn bullet_hit_enemy_generator(player_id: PlayerId, bullet_id: BulletId, generator_id: EnemyGeneratorId, world: &mut WorldView) {
        if let Some(attack) = world.players.bullet_attack(player_id, bullet_id) {
            world.enemies.enemy_generator_hit(generator_id, attack);
        }
    }
}
