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
        Enemy,
        EnemyId,
        EnemyConfig,
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
    },
};
use generational_slab::Slab;
use nalgebra::Point2;

pub struct EnemySystem {
    config_manager: SimpleConfigManager<EnemyConfig>,
    generator_spawns: Vec<EnemyGeneratorSpawn>,
    generators: Slab<EnemyGenerator>,
    dead_generators: Vec<Point2<f64>>,
    enemies: Slab<Enemy>,
}

impl EnemySystem {
    pub fn new(config_watcher: &mut ConfigWatcher, generator_spawns: &Vec<Point2<f64>>, physics_sim: &mut PhysicsSimulation) -> StatusOr<EnemySystem> {
        let config_manager: SimpleConfigManager<EnemyConfig> = SimpleConfigManager::from_config_resource(config_watcher, "enemy.conf")?;

        let (generators, enemies) = {
            let config = config_manager.get();
            let generators = Slab::with_capacity(config.generators_slab_initial_capacity_guess);
            let enemies = Slab::with_capacity(config.enemies_slab_initial_capacity_guess);
            (generators, enemies)
        };

        let generator_spawns: Vec<_> = generator_spawns.iter()
            .map(|spawn_point| EnemyGeneratorSpawn {
                position: (spawn_point.x, spawn_point.y),
                orientation: 0.0,
            })
            .collect();

        let dead_generators = Vec::with_capacity(generator_spawns.len());

        let mut enemy_system = EnemySystem {
            config_manager,
            generator_spawns,
            generators,
            dead_generators,
            enemies,
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
            enemy.pre_update(config, dt, &player_locs);
        }
    }

    pub fn post_update(&mut self, audio: &AudioPlayer, items: &mut ItemSystem, physics_sim: &mut PhysicsSimulation) {
        let config = self.config_manager.get();

        let dead_enemy_generator_keys: Vec<_> = self.generators
            .iter_mut()
            .filter_map(|(generator_key, generator)| {
                generator.post_update(items, physics_sim);
                if !generator.dead() {
                    return None;
                }
                Some(generator_key)
            })
            .collect();

        for generator_key in dead_enemy_generator_keys.into_iter() {
            if let Some(generator) = self.generators.remove(generator_key) {
                if let Some(position) = generator.position() {
                    self.dead_generators.push(position);
                }
            }
        }

        let dead_enemy_keys: Vec<_> = self.enemies
            .iter_mut()
            .filter_map(|(enemy_key, enemy)| {
                enemy.post_update(config, audio, items, physics_sim);
                if !enemy.dead() {
                    return None;
                }
                Some(enemy_key)
            })
            .collect();

        for enemy_key in dead_enemy_keys.into_iter() {
            self.enemies.remove(enemy_key);
        }
    }

    pub fn populate_lights(&self, lights: &mut PointLights) {
        let config = self.config_manager.get();
        let generator_lights = self.generators
            .iter()
            .filter_map(|(_key, generator)| {
                generator.point_light(config)
            });
        lights.append(generator_lights);

        let enemy_lights = self.enemies
            .iter()
            .filter_map(|(_key, enemy)| {
                enemy.point_light(config)
            });
        lights.append(enemy_lights);
    }

    pub fn queue_draw(&self, light_dependent: &mut LightDependentSpriteRenderer) {
        let config = self.config_manager.get();
        for (_key, generator) in self.generators.iter() {
            generator.queue_draw(config, light_dependent);
        }
        EnemyGenerator::queue_draw_dead(config, self.dead_generators.as_slice(), light_dependent);
        for (_key, enemy) in self.enemies.iter() {
            enemy.queue_draw(config, light_dependent);
        }
    }

    pub fn enemy_hit(&mut self, enemy_id: EnemyId, attack: Attack, particles: &mut ParticleSystem) {
        if let Some(enemy) = self.enemies.get_mut(enemy_id.key()) {
            let config = self.config_manager.get();
            enemy.take_attack(config, attack, particles);
        }
    }

    pub fn enemy_generator_hit(&mut self, audio: &AudioPlayer, generator_id: EnemyGeneratorId, attack: Attack, particles: &mut ParticleSystem) {
        if let Some(generator) = self.generators.get_mut(generator_id.key()) {
            let config = self.config_manager.get();
            generator.take_attack(config, audio, attack, particles);
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
            let generator = EnemyGenerator::new(config, generator_id, *generator_spawn, physics_sim);
            generator_entry.insert(generator);
        }
    }
}
