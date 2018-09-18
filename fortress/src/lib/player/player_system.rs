use app::StatusOr;
use control::{
    Controller,
    ControllerId,
    ControlEvent,
    ControllerEvent,
};
use dimensions::time::DeltaTime;
use file::{
    ConfigWatcher,
    SimpleConfigManager,
};
use glm;
use physics::PhysicsSimulation;
use player::{
    Player,
    PlayerConfig,
    PlayerId,
    self,
};
use render::{
    BoxRenderer,
    CameraView,
    Viewport,
};
use slab::Slab;

pub struct PlayerSystem {
    config_manager: SimpleConfigManager<PlayerConfig>,
    players: Slab<Player>,
    player_to_controller: Vec<ControllerId>,
}

impl PlayerSystem {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<PlayerSystem> {
        let config_manager = SimpleConfigManager::new(config_watcher, "player.conf")?;
        Ok(PlayerSystem {
            config_manager,
            players: Slab::with_capacity(player::MAX_PLAYERS),
            player_to_controller: Vec::with_capacity(player::MAX_PLAYERS),
        })
    }

    pub fn pre_update(&mut self, controller: &Controller, physics_sim: &mut PhysicsSimulation, dt: DeltaTime) {
        if self.config_manager.update() || controller.just_pressed(ControllerId::Keyboard, ControlEvent::RespawnEntities) {
            self.redeploy(physics_sim);
        }

        for controller_event in controller.controller_events().into_iter() {
            match controller_event {
                ControllerEvent::KeyboardUsed => {
                    self.new_player(ControllerId::Keyboard, physics_sim);
                }
                ControllerEvent::GamepadConnected(gamepad_id) => {
                    self.new_player(ControllerId::Gamepad(gamepad_id), physics_sim);
                },
            }
        }

        for (player_idx, player) in self.players.iter_mut() {
            let controller_id = self.player_to_controller[player_idx];
            player.pre_update(controller_id, controller, dt);
        }
    }

    pub fn post_update(&mut self) {
        for (_i, player) in self.players.iter_mut() {
            player.post_update();
        }
    }

    pub fn draw(&self, box_renderer: &mut BoxRenderer) {
        for (_i, player) in self.players.iter() {
            player.draw(box_renderer);
        }
    }

    pub fn get_views(&self, screen_size: &glm::IVec2) -> Vec<CameraView> {
        let player_positions: Vec<glm::Vec2> = self.players.iter().map(|(_, player)| player.get_position()).collect();
        match player_positions.len() {
            1 => {
                vec!(CameraView {
                    eye: player_positions[0],
                    scale: glm::vec2(1.0, 1.0),
                    viewport: Viewport::default(screen_size),
                })
            },
            2 => {
                let middle_x = screen_size.x / 2;
                vec!(
                    CameraView {
                        eye: player_positions[0],
                        scale: glm::vec2(0.5, 1.0),
                        viewport: Viewport {
                            bottom_left: glm::ivec2(0, 0),
                            viewport_size: glm::ivec2(middle_x, screen_size.y),
                        }
                    },
                    CameraView {
                        eye: player_positions[1],
                        scale: glm::vec2(0.5, 1.0),
                        viewport: Viewport {
                            bottom_left: glm::ivec2(middle_x, 0),
                            viewport_size: glm::ivec2(screen_size.x - middle_x, screen_size.y),
                        }
                    }
                )
            },
            _ => {
                vec!(
                    CameraView {
                        eye: glm::vec2(4.0, 0.0),
                        scale: glm::vec2(1.0, 1.0),
                        viewport: Viewport::default(screen_size),
                    }
                )
            }
        }
    }

    fn redeploy(&mut self, physics_sim: &mut PhysicsSimulation) {
        let config = self.config_manager.get();
        for (_i, player) in self.players.iter_mut() {
            player.redeploy(config, physics_sim);
        }
    }

    fn new_player(&mut self, controller_id: ControllerId, physics_sim: &mut PhysicsSimulation) {
        let player_id = {
            let player_entry = self.players.vacant_entry();
            let player_id = PlayerId::from_usize(player_entry.key());
            if let Some(player_id) = player_id {
                let config = self.config_manager.get();
                let player = Player::new(config, player_id, physics_sim);
                player_entry.insert(player);
            }
            player_id
        };

        if let Some(player_id) = player_id {
            let raw_player_id = player_id.as_usize();
            self.players.get_mut(raw_player_id).expect("PlayerSystem has bad key!").register();
            self.player_to_controller.push(controller_id);
        }
    }
}