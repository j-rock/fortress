use crate::{
    app::StatusOr,
    audio::{
        AudioPlayer,
        Sound
    },
    control::{
        Controller,
        ControllerId,
        ControlEvent,
        ControllerEvent,
        IdentifiedController,
    },
    dimensions::{
        Attack,
        time::DeltaTime
    },
    file::{
        ConfigWatcher,
        SimpleConfigManager,
    },
    hud::{
        IndividualPlayerHudData,
        PlayerHudUpdate,
    },
    items::{
        ItemConfig,
        ItemPickup,
    },
    math::RandGen,
    physics::PhysicsSimulation,
    particles::ParticleSystem,
    players::{
        Player,
        PlayerSystemConfig,
        PlayerId,
        self,
    },
    render::{
        FullyIlluminatedSpriteRenderer,
        LightDependentSpriteRenderer,
        PointLights,
        ScreenShake,
    },
    weapons::BulletId,
};
use generational_slab::Slab;
use nalgebra::{
    Point2,
    Vector2,
};
use std::collections::HashMap;

pub struct PlayerSystem {
    config_manager: SimpleConfigManager<PlayerSystemConfig>,
    players: Slab<Player>,
    player_needs_controller: Vec<PlayerId>,
    player_to_controller: Vec<ControllerId>,
    controller_to_player: HashMap<ControllerId, PlayerId>,
    spawns: Vec<Point2<f64>>,
}

impl PlayerSystem {
    pub fn new(config_watcher: &mut ConfigWatcher, spawns: &[Point2<f64>]) -> StatusOr<PlayerSystem> {
        let config_manager = SimpleConfigManager::from_config_resource(config_watcher, "player.conf")?;
        Ok(PlayerSystem {
            config_manager,
            players: Slab::with_capacity(players::MAX_PLAYERS),
            player_needs_controller: Vec::with_capacity(players::MAX_PLAYERS),
            player_to_controller: Vec::with_capacity(players::MAX_PLAYERS),
            controller_to_player: HashMap::new(),
            spawns: spawns.iter().cloned().collect(),
        })
    }

    pub fn pre_update(&mut self,
                      audio: &AudioPlayer,
                      controller: &Controller,
                      particles: &mut ParticleSystem,
                      rng: &mut RandGen,
                      shake: &mut ScreenShake,
                      physics_sim: &mut PhysicsSimulation,
                      dt: DeltaTime) {
        let anyone_pressed_redeploy =
            self.controller_to_player
                .keys()
                .any(|controller_id| controller.just_pressed(*controller_id, ControlEvent::RedeployEntities));

        if self.config_manager.update() || anyone_pressed_redeploy {
            self.redeploy(physics_sim);
        }

        for controller_event in controller.controller_events().into_iter() {
            match controller_event {
                ControllerEvent::KeyboardUsed => {
                    let controller_id = ControllerId::Keyboard;

                    if let Some(player_id) = self.player_needs_controller.pop() {
                        self.player_to_controller[player_id.to_raw_usize()] = controller_id;
                        self.controller_to_player.insert(controller_id, player_id);
                    } else {
                        self.new_player(controller_id, audio, physics_sim);
                    }
                }
                ControllerEvent::GamepadConnected(gamepad_id) => {
                    let controller_id = ControllerId::Gamepad(gamepad_id);

                    if let Some(player_id) = self.player_needs_controller.pop() {
                        self.player_to_controller[player_id.to_raw_usize()] = controller_id;
                        self.controller_to_player.insert(controller_id, player_id);
                    } else {
                        self.new_player(controller_id, audio, physics_sim);
                    }
                },
                ControllerEvent::GamepadDisconnected(gamepad_id) => {
                    let controller_id = ControllerId::Gamepad(gamepad_id);

                    let delete_controller_id = if let Some(player_id) = self.controller_to_player.get(&controller_id) {
                        self.player_needs_controller.push(*player_id);
                        true
                    } else {
                        false
                    };

                    if delete_controller_id {
                        self.controller_to_player.remove(&controller_id);
                    }
                }
            }
        }

        let config = self.config_manager.get();
        for (player_key, player) in self.players.iter_mut() {
            let controller_id = self.player_to_controller[player_key.to_raw()];
            let identified_controller = IdentifiedController::new(controller, controller_id);
            player.pre_update(config, audio, identified_controller, dt, particles, rng, shake);
        }
    }

    pub fn post_update(&mut self) {
        for (_i, player) in self.players.iter_mut() {
            player.post_update();
        }
    }

    // Returns bullet direction.
    pub fn bullet_hit(&mut self, player_id: PlayerId, bullet_id: BulletId) -> Option<Vector2<f64>> {
        let player = self.players.get_mut(player_id.to_key())?;
        player.bullet_hit(bullet_id)
    }

    pub fn bullet_attack(&self, player_id: PlayerId, bullet_id: BulletId, rng: &mut RandGen) -> Option<Attack> {
        let player = self.players.get(player_id.to_key())?;
        let config = self.config_manager.get();
        player.bullet_attack(&config.bullet, bullet_id, rng)
    }

    pub fn populate_lights(&self, item_config: &ItemConfig, lights: &mut PointLights) {
        let config = self.config_manager.get();
        for (_i, player) in self.players.iter() {
            player.populate_lights( config, item_config, lights);
        }
    }

    pub fn queue_draw(&self, full_light: &mut FullyIlluminatedSpriteRenderer, light_dependent: &mut LightDependentSpriteRenderer) {
        let config = self.config_manager.get();
        for (_i, player) in self.players.iter() {
            player.queue_draw( config, full_light, light_dependent);
        }
    }

    pub fn respawn(&mut self, spawns: &[Point2<f64>]) {
        self.spawns = spawns.iter().cloned().collect();
        for (_i, player) in self.players.iter_mut() {
            let spawn = self.spawns[player.get_player_id().to_raw_usize()].clone();
            player.respawn(spawn);
        }
    }

    pub fn player_locs(&self) -> Vec<Point2<f64>> {
        self.players
            .iter()
            .filter_map(|(_i, player)| {
                player.position()
            })
            .collect()
    }

    pub fn prepare_player_hud_update(&self) -> PlayerHudUpdate {
        let mut hud_update = PlayerHudUpdate::new();

        self.players
            .iter()
            .for_each(|(_i, player)| {
                hud_update.append(IndividualPlayerHudData {
                    skulls_collected: player.skull_count(),
                });
            });

        hud_update
    }

    pub fn collect_item(&mut self, player_id: PlayerId, item_config: &ItemConfig, item_pickup: ItemPickup) {
        let ref config = self.config_manager.get().item;
        if let Some(player) = self.players.get_mut(player_id.to_key()) {
            player.collect_item(config, item_config, item_pickup);
        }
    }

    fn redeploy(&mut self, physics_sim: &mut PhysicsSimulation) {
        let config = self.config_manager.get();
        for (_i, player) in self.players.iter_mut() {
            player.redeploy(config, physics_sim);
        }
    }

    fn new_player(&mut self, controller_id: ControllerId, audio: &AudioPlayer, physics_sim: &mut PhysicsSimulation) {
        let player_id = {
            let player_entry = self.players.vacant_entry();
            let player_id = PlayerId::from_key(player_entry.key());
            if let Some(player_id) = player_id {
                let config = self.config_manager.get();
                let spawn = self.spawns[player_id.to_raw_usize()].clone();
                let player = Player::new(config, player_id, spawn, physics_sim);
                player_entry.insert(player);
            }
            player_id
        };

        if let Some(player_id) = player_id {
            self.player_to_controller.push(controller_id);
            self.controller_to_player.insert(controller_id, player_id);

            audio.play_sound(Sound::JoinGame);
        }
    }
}