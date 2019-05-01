use crate::{
    audio::{
        AudioPlayer,
        Sound,
    },
    dimensions::{
        Attack,
        time::{
            DeltaTime,
            Microseconds,
        }
    },
    enemies::{
        EnemyConfig,
        EnemyState,
        state::EnemyBody,
    },
    render::{
        LightDependentSpriteData,
        LightDependentSpriteRenderer,
        NamedSpriteSheet,
        SpriteSheetFrameId,
    },
};
use glm;
use nalgebra::Point2;

pub enum EnemyStateMachine {
    Base(EnemyBody, Microseconds),
    Dying(Option<Point2<f64>>, Microseconds),
    Dead
}

impl EnemyStateMachine {
    pub fn new(body: EnemyBody) -> EnemyStateMachine {
        EnemyStateMachine::Base(body, 0)
    }

    pub fn pre_update(&mut self, config: &EnemyConfig, dt: DeltaTime, player_locs: &Vec<Point2<f64>>) -> Option<EnemyStateMachine> {
        match self {
            EnemyStateMachine::Base(body, time_elapsed) => {
                *time_elapsed += dt.as_microseconds();
                body.move_to_target(config, player_locs);
            },
            EnemyStateMachine::Dying(_, time_elapsed) => {
                *time_elapsed += dt.as_microseconds();
            },
            _ => {},
        }
        None
    }

    pub fn take_attack(&self, attack: Attack, enemy_state: &mut EnemyState) {
        if let EnemyStateMachine::Base(_, _) = self {
            enemy_state.take_attack(attack);
        }
    }

    pub fn post_update(&mut self, config: &EnemyConfig, audio: &AudioPlayer, enemy_state: &EnemyState) -> Option<EnemyStateMachine> {
        match self {
            EnemyStateMachine::Base(body, _) if !enemy_state.health().alive() => {
                audio.play_sound(Sound::Raygun);
                let position = body.position();
                Some(EnemyStateMachine::Dying(position, 0))
            },
            EnemyStateMachine::Dying(_, time_elapsed) if *time_elapsed >= config.enemy_dying_duration_micros => {
                Some(EnemyStateMachine::Dead)
            },
            _ => None
        }
    }

    pub fn queue_draw(&self, config: &EnemyConfig, sprite_renderer: &mut LightDependentSpriteRenderer) {
        let image_name = match self {
            EnemyStateMachine::Dying(_, _) => String::from("enemy1_dying.png"),
            _ => String::from("enemy1.png")
        };

        let frame = match self {
            EnemyStateMachine::Base(_, time_elapsed) => (*time_elapsed / config.enemy_walk_frame_duration_micros) as usize,
            EnemyStateMachine::Dying(_, time_elapsed) => (*time_elapsed / config.enemy_dying_frame_duration_micros) as usize,
            _ => 0,
        };

        if let Some(position) = self.position() {
            let world_bottom_center_position = glm::vec3(position.x as f32, 0.0, -position.y as f32);
            let world_half_size = glm::vec2(config.enemy_physical_radius as f32, config.enemy_physical_radius as f32) * config.enemy_render_scale;

            sprite_renderer.queue(vec![LightDependentSpriteData {
                world_bottom_center_position,
                world_half_size,
                sprite_frame_id: SpriteSheetFrameId {
                    name: image_name,
                    sprite_sheet: NamedSpriteSheet::SpriteSheet1,
                },
                frame
            }]);
        }
    }

    pub fn dead(&self) -> bool {
        if let EnemyStateMachine::Dead = self {
            true
        } else {
            false
        }
    }

    fn position(&self) -> Option<Point2<f64>> {
        match self {
            EnemyStateMachine::Base(body, _) => body.position(),
            EnemyStateMachine::Dying(position, _) => *position,
            _ => None
        }
    }
}
