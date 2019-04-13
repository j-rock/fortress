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
    },
    dimensions::time::DeltaTime,
    file::{
        ConfigWatcher,
        SimpleConfigManager,
    },
    physics::PhysicsSimulation,
    players::{
        Player,
        PlayerConfig,
        PlayerId,
        self,
    },
};
use nalgebra::Point2;
use slab::Slab;
use hashbrown::HashMap;

pub struct PlayerSystem {
    config_manager: SimpleConfigManager<PlayerConfig>,
    players: Slab<Player>,
    player_needs_controller: Vec<PlayerId>,
    player_to_controller: Vec<ControllerId>,
    controller_to_player: HashMap<ControllerId, PlayerId>,
    spawns: Vec<Point2<f64>>,
}

impl PlayerSystem {
    pub fn new(config_watcher: &mut ConfigWatcher, spawns: Vec<Point2<f64>>) -> StatusOr<PlayerSystem> {
        let config_manager = SimpleConfigManager::from_config_resource(config_watcher, "player.conf")?;
        Ok(PlayerSystem {
            config_manager,
            players: Slab::with_capacity(players::MAX_PLAYERS),
            player_needs_controller: Vec::with_capacity(players::MAX_PLAYERS),
            player_to_controller: Vec::with_capacity(players::MAX_PLAYERS),
            controller_to_player: HashMap::new(),
            spawns,
        })
    }

    pub fn pre_update(&mut self, audio: &AudioPlayer, controller: &Controller, physics_sim: &mut PhysicsSimulation, dt: DeltaTime) {
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
                        self.player_to_controller[player_id.as_usize()] = controller_id;
                        self.controller_to_player.insert(controller_id, player_id);
                    } else {
                        self.new_player(controller_id, audio, physics_sim);
                    }
                }
                ControllerEvent::GamepadConnected(gamepad_id) => {
                    let controller_id = ControllerId::Gamepad(gamepad_id);

                    if let Some(player_id) = self.player_needs_controller.pop() {
                        self.player_to_controller[player_id.as_usize()] = controller_id;
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

        for (player_idx, player) in self.players.iter_mut() {
            let controller_id = self.player_to_controller[player_idx];
            player.pre_update(audio, controller_id, controller, dt);
        }
    }

    pub fn post_update(&mut self, audio: &AudioPlayer) {
        for (_i, player) in self.players.iter_mut() {
            player.post_update(audio);
        }
    }

    pub fn draw(&self) {
        let config = self.config_manager.get();
        for (_i, player) in self.players.iter() {
            player.draw( config);
        }
    }

    pub fn respawn(&mut self, spawns: Vec<Point2<f64>>) {
        self.spawns = spawns;
        for (_i, player) in self.players.iter_mut() {
            let spawn = self.spawns[player.get_player_id().as_usize()];
            player.respawn(spawn);
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
            let player_id = PlayerId::from_usize(player_entry.key());
            if let Some(player_id) = player_id {
                let config = self.config_manager.get();
                let spawn = self.spawns[player_id.as_usize()].clone();
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