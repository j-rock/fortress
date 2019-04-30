use crate::{
    enemies::{
        EnemyId,
        EnemyGeneratorId,
    },
    entities::Entity,
    physics::{
        Contact,
        ContactMatcher
    },
    players::PlayerId,
    weapons::BulletId,
    world::WorldView,
};

pub struct WeaponMatchers;

impl WeaponMatchers {
    pub fn bullet_hit() -> ContactMatcher {
        ContactMatcher::new(Box::new(|contact, world: &mut WorldView| {
            if let Contact::Started(entity1, entity2) = contact {
                if let Entity::Bullet(player_id, bullet_id) = entity1 {
                    Self::process_bullet_hit(player_id, bullet_id, world);
                    match entity2 {
                        Entity::Enemy(enemy_id) => Self::bullet_hit_enemy(player_id, bullet_id, enemy_id, world),
                        Entity::EnemyGenerator(generator_id) => Self::bullet_hit_enemy_generator(player_id, bullet_id, generator_id, world),
                        _ => {}
                    }
                }

                if let Entity::Bullet(player_id, bullet_id) = entity2 {
                    Self::process_bullet_hit(player_id, bullet_id, world);
                    match entity1 {
                        Entity::Enemy(enemy_id) => Self::bullet_hit_enemy(player_id, bullet_id, enemy_id, world),
                        Entity::EnemyGenerator(generator_id) => Self::bullet_hit_enemy_generator(player_id, bullet_id, generator_id, world),
                        _ => {}
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
            world.enemies.enemy_hit(enemy_id, attack);
        }
    }

    fn bullet_hit_enemy_generator(player_id: PlayerId, bullet_id: BulletId, generator_id: EnemyGeneratorId, world: &mut WorldView) {
        if let Some(attack) = world.players.bullet_attack(player_id, bullet_id) {
            world.enemies.enemy_generator_hit(generator_id, attack);
        }
    }
}
