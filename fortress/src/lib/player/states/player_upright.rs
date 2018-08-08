use control::{
    Controller,
    events::ControlEvent::PlayerMove,
};
use dimensions::{
    LrDirection,
    time::DeltaTime,
};
use player::{
    PlayerBody,
    PlayerConfig,
    states::PlayerState
};

pub struct PlayerUpright {
    player_body: PlayerBody,
    move_speed: f32,
}

impl PlayerState for PlayerUpright {
    fn update(mut self, controller: &Controller, _dt: DeltaTime) -> Box<dyn PlayerState> {
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
    pub fn new(config: &PlayerConfig, body: PlayerBody) -> PlayerUpright {
        PlayerUpright {
            player_body: body,
            move_speed: config.move_speed,
        }
    }
}
