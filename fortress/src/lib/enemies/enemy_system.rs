use crate::{
    app::StatusOr,
    audio::AudioPlayer,
    control::{
        ControlEvent,
        Controller,
        ControllerId,
    },
    dimensions::{
        Attack,
        time::DeltaTime
    },
    enemies::{
        DamageTextWriter,
        Enemy,
        EnemyId,
        EnemySystemConfig,
        EnemyGenerator,
        EnemyGeneratorId,
        EnemyGeneratorSpawn,
    },
    file::{
        ConfigWatcher,
        SimpleConfigManager,
    },
    items::ItemSystem,
    particles::ParticleSystem,
    physics::PhysicsSimulation,
    render::{
        LightDependentSpriteRenderer,
        PointLights,
        ScreenShake,
    },
    text::TextRenderer,
};
use generational_slab::Slab;
use nalgebra::{
    Point2,
    Vector2,
};

pub struct EnemySystem {
    config_manager: SimpleConfigManager<EnemySystemConfig>,
    generator_spawns: Vec<EnemyGeneratorSpawn>,
    generators: Slab<EnemyGenerator>,
    enemies: Slab<Enemy>,
    damage_text: DamageTextWriter,
}

impl EnemySystem {
    pub fn new(config_watcher: &mut ConfigWatcher, generator_spawns: &Vec<Point2<f64>>, physics_sim: &mut PhysicsSimulation) -> StatusOr<EnemySystem> {
        let config_manager: SimpleConfigManager<EnemySystemConfig> = SimpleConfigManager::from_config_resource(config_watcher, "enemy.conf")?;

        let (generators, enemies, damage_text) = {
            let config = config_manager.get();
            let generators = Slab::with_capacity(config.generator.slab_initial_capacity_guess);
            let enemies = Slab::with_capacity(config.enemy.slab_initial_capacity_guess);
            let damage_text = DamageTextWriter::new(&config.damage_text);
            (generators, enemies, damage_text)
        };

        let generator_spawns = generator_spawns.iter()
            .map(|spawn_point| EnemyGeneratorSpawn {
                position: (spawn_point.x, spawn_point.y),
                orientation: 0.0,
            })
            .collect();

        let mut enemy_system = EnemySystem {
            config_manager,
            generator_spawns,
            generators,
            enemies,
            damage_text,
        };
        enemy_system.redeploy(physics_sim);

        Ok(enemy_system)
    }

    pub fn pre_update(&mut self, controller: &Controller, dt: DeltaTime, player_locs: Vec<Point2<f64>>, physics_sim: &mut PhysicsSimulation) {
        if self.config_manager.update() || controller.just_pressed(ControllerId::Keyboard, ControlEvent::RedeployEntities) {
            self.redeploy(physics_sim);
        }

        let config = self.config_manager.get();
        for (_key, generator) in self.generators.iter_mut() {
            generator.pre_update(config, dt, &player_locs, &mut self.enemies, physics_sim);
        }

        for (_key , enemy) in self.enemies.iter_mut() {
            enemy.pre_update(&config.enemy, dt, &player_locs);
        }

        self.damage_text.pre_update(&config.damage_text, dt);
    }

    pub fn post_update(&mut self, audio: &AudioPlayer, items: &mut ItemSystem, shake: &mut ScreenShake, physics_sim: &mut PhysicsSimulation) {
        let config = self.config_manager.get();

        self.generators.retain(|generator| {
            generator.post_update(&config.generator, items, shake, physics_sim);
            !generator.dead()
        });

        let generators = &mut self.generators;
        self.enemies.retain(|enemy| {
            enemy.post_update(&config.enemy, audio, items, physics_sim);
            let scheduled_for_deletion = enemy.dead();
            if scheduled_for_deletion {
                generators
                    .get_mut(enemy.generator_id().key())
                    .map(|generator| {
                        generator.tally_killed_enemy();
                    });
            }
            !scheduled_for_deletion
        });
    }

    pub fn populate_lights(&self, lights: &mut PointLights) {
        let config = self.config_manager.get();
        let generator_lights = self.generators
            .iter()
            .filter_map(|(_key, generator)| {
                generator.point_light(&config.generator)
            });
        lights.append(generator_lights);
    }

    pub fn queue_draw(&self, light_dependent: &mut LightDependentSpriteRenderer, text: &mut TextRenderer) {
        let config = self.config_manager.get();
        for (_key, generator) in self.generators.iter() {
            generator.queue_draw(&config.generator, light_dependent);
        }
        for (_key, enemy) in self.enemies.iter() {
            enemy.queue_draw(&config.enemy, light_dependent);
        }
        self.damage_text.queue_draw(&config.damage_text, text);
    }

    pub fn enemy_hit(&mut self,
                     enemy_id: EnemyId,
                     attack: Attack,
                     bullet_direction: Option<Vector2<f64>>,
                     particles: &mut ParticleSystem) {
        if let Some(enemy) = self.enemies.get_mut(enemy_id.key()) {
            let config = self.config_manager.get();
            enemy.take_attack(config, attack, bullet_direction, particles, &mut self.damage_text);
        }
    }

    pub fn enemy_generator_hit(&mut self, audio: &AudioPlayer, generator_id: EnemyGeneratorId, attack: Attack, particles: &mut ParticleSystem) {
        if let Some(generator) = self.generators.get_mut(generator_id.key()) {
            let config = self.config_manager.get();
            generator.take_attack(&config.generator, audio, attack, particles);
        }
    }

    pub fn respawn(&mut self, generator_spawns: &Vec<Point2<f64>>, physics_sim: &mut PhysicsSimulation) {
        self.generator_spawns = generator_spawns.iter()
            .map(|spawn| EnemyGeneratorSpawn {
                position: (spawn.x, spawn.y),
                orientation: 0.0,
            })
            .collect();
        self.redeploy(physics_sim);
    }

    fn redeploy(&mut self, physics_sim: &mut PhysicsSimulation) {
        let config = self.config_manager.get();
        self.generators.clear();
        self.enemies.clear();

        for generator_spawn in self.generator_spawns.iter() {
            let generator_entry = self.generators.vacant_entry();
            let generator_id = EnemyGeneratorId::from_key(generator_entry.key());
            let generator = EnemyGenerator::new(&config.generator, generator_id, *generator_spawn, physics_sim);
            generator_entry.insert(generator);
        }
    }
}
