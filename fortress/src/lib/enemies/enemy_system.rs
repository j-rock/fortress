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
    file::{
        ConfigWatcher,
        SimpleConfigManager,
    },
    enemies::{
        Enemy,
        EnemyId,
        EnemyConfig,
        EnemyGenerator,
        EnemyGeneratorId,
        EnemyGeneratorSpawn,
    },
    physics::PhysicsSimulation,
    render::LightDependentSpriteRenderer,
};
use generational_slab::Slab;

pub struct EnemySystem {
    config_manager: SimpleConfigManager<EnemyConfig>,
    generator_spawns: Vec<EnemyGeneratorSpawn>,
    generators: Slab<EnemyGenerator>,
    enemies: Slab<Enemy>,
}

impl EnemySystem {
    pub fn new(config_watcher: &mut ConfigWatcher, generator_spawns: Vec<EnemyGeneratorSpawn>, physics_sim: &mut PhysicsSimulation) -> StatusOr<EnemySystem> {
        let config_manager: SimpleConfigManager<EnemyConfig> = SimpleConfigManager::from_config_resource(config_watcher, "enemy.conf")?;

        let (generators, enemies) = {
            let config = config_manager.get();
            let generators = Slab::with_capacity(config.generators_slab_initial_capacity_guess);
            let enemies = Slab::with_capacity(config.enemies_slab_initial_capacity_guess);
            (generators, enemies)
        };

        let mut enemy_system = EnemySystem {
            config_manager,
            generator_spawns,
            generators,
            enemies,
        };
        enemy_system.redeploy(physics_sim);

        Ok(enemy_system)
    }

    pub fn pre_update(&mut self, controller: &Controller, dt: DeltaTime, physics_sim: &mut PhysicsSimulation) {
        if self.config_manager.update() || controller.just_pressed(ControllerId::Keyboard, ControlEvent::RedeployEntities) {
            self.redeploy(physics_sim);
        }

        let config = self.config_manager.get();
        for (_key, generator) in self.generators.iter_mut() {
            generator.pre_update(config, dt, &mut self.enemies, physics_sim);
        }

        for (_key , enemy) in self.enemies.iter_mut() {
            enemy.pre_update(dt);
        }
    }

    pub fn post_update(&mut self, audio: &AudioPlayer) {
        let config = self.config_manager.get();

        for (_key, generator) in self.generators.iter_mut() {
            generator.post_update(audio);
        }

        let dead_enemy_keys: Vec<_> = self.enemies
            .iter_mut()
            .filter_map(|(enemy_key, enemy)| {
                enemy.post_update(config, audio);
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

    pub fn queue_draw(&self, light_dependent: &mut LightDependentSpriteRenderer) {
        let config = self.config_manager.get();
        for (_key, generator) in self.generators.iter() {
            generator.queue_draw(config, light_dependent);
        }
        for (_key, enemy) in self.enemies.iter() {
            enemy.queue_draw(config, light_dependent);
        }
    }

    pub fn enemy_hit(&mut self, enemy_id: EnemyId, attack: Attack) {
        if let Some(enemy) = self.enemies.get_mut(enemy_id.key()) {
            enemy.take_attack(attack);
        }
    }

    pub fn enemy_generator_hit(&mut self, generator_id: EnemyGeneratorId, attack: Attack) {
        if let Some(generator) = self.generators.get_mut(generator_id.key()) {
            generator.take_attack(attack);
        }
    }

    pub fn respawn(&mut self, generator_spawns: Vec<EnemyGeneratorSpawn>, physics_sim: &mut PhysicsSimulation) {
        self.generator_spawns = generator_spawns;
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
