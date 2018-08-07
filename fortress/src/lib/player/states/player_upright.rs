use control::{
    Controller,
    events::ControlEvent::PlayerMove,
};
use dimensions::{
    LrDirection,
    time::DeltaTime,
};
use entity::EntityRegistrar;
use physics::PhysicsSimulation;
use player::{
    PlayerBody,
    PlayerBodyConfig,
    states::PlayerState
};

#[derive(Copy, Clone)]
pub struct PlayerUprightConfig {
    move_speed: f32,
}

pub struct PlayerUpright {
    player_body: PlayerBody,
    move_speed: f32,
}

impl PlayerState for PlayerUpright {
    fn update(mut self, controller: &Controller, _registrar: &mut EntityRegistrar, _dt: DeltaTime) -> Box<dyn PlayerState> {
        let move_dir = if controller.is_pressed(PlayerMove(LrDirection::Left)) {
            Some(LrDirection::Left)
        } else if controller.is_pressed(PlayerMove(LrDirection::Right)) {
            Some(LrDirection::Right)
        } else {
            None
        };
        self.player_body.move_horizontal(self.move_speed, move_dir);

        Box::new(self)
    }
}

impl PlayerUpright {
    pub fn brand_new(upright_config: PlayerUprightConfig, body_config: &PlayerBodyConfig, physics_sim: &mut PhysicsSimulation) -> PlayerUpright {
        PlayerUpright {
            player_body: PlayerBody::new(body_config, physics_sim),
            move_speed: config.move_speed,
        }
    }

    pub fn new(config: &PlayerUprightConfig, body: PlayerBody) -> PlayerUpright {
        PlayerUpright {
            player_body: body,
            move_speed: config.move_speed,
        }
    }
}
