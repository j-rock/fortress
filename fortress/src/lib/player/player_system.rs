use app::StatusOr;
use control::{
    Controller,
    ControlEvent,
};
use dimensions::time::DeltaTime;
use file::{
    ConfigWatcher,
    SimpleConfigManager,
};
use physics::PhysicsSimulation;
use player::{
    Player,
    PlayerConfig,
    PlayerId,
};
use render::BoxRenderer;
use slab::Slab;

pub struct PlayerSystem {
    config_manager: SimpleConfigManager<PlayerConfig>,
    players: Slab<Player>,
}

impl PlayerSystem {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<PlayerSystem> {
        let config_manager = SimpleConfigManager::new(config_watcher, "player.conf")?;
        Ok(PlayerSystem {
            config_manager,
            players: Slab::with_capacity(4),
        })
    }

    pub fn pre_update(&mut self, controller: &Controller, physics_sim: &mut PhysicsSimulation, dt: DeltaTime) {
        if self.config_manager.update() || controller.just_pressed(ControlEvent::RespawnEntities) {
            self.redeploy(physics_sim);
        }

        if controller.keyboard_used_first_time() {
            self.new_player(physics_sim);
        }

        for (_i, player) in self.players.iter_mut() {
            player.pre_update(controller, dt);
        }
    }

    pub fn post_update(&mut self) {
        for (_i, player) in self.players.iter_mut() {
            player.post_update();
        }
    }

    pub fn draw(&self, box_renderer: &mut BoxRenderer) {
        for (_i, mut player) in self.players.iter() {
            player.draw(box_renderer);
        }
    }

    pub fn get_player1_pos(&self) -> (f32, f32) {
        if let Some(player1) = self.players.get(0) {
            player1.get_position()
        } else {
            (4.0, 0.0)
        }
    }

    fn redeploy(&mut self, physics_sim: &mut PhysicsSimulation) {
        let config = self.config_manager.get();
        for (_i, player) in self.players.iter_mut() {
            player.redeploy(config, physics_sim);
        }
    }

    fn new_player(&mut self, physics_sim: &mut PhysicsSimulation) {
        let player_id = {
            let player_entry = self.players.vacant_entry();
            let player_id = PlayerId::from(player_entry.key());
            if player_id.is_some() {
                let config = self.config_manager.get();
                let player = Player::new(config, physics_sim);
                player_entry.insert(player);
            }
            player_id
        };

        if let Some(player_id) = player_id {
            let key = player_id.as_usize();
            self.players.get_mut(key).expect("PlayerSystem has bad key!").register();
        }
    }
}