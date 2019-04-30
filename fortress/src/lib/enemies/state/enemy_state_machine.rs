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
        EnemyState
    },
    render::{
        LightDependentSpriteData,
        LightDependentSpriteRenderer,
        NamedSpriteSheet,
        SpriteSheetFrameId,
    },
};
use glm;

#[derive(PartialEq)]
pub enum EnemyStateMachine {
    Base,
    Dying(Microseconds),
    Dead
}

impl Default for EnemyStateMachine {
    fn default() -> EnemyStateMachine {
        EnemyStateMachine::Base
    }
}

impl EnemyStateMachine {
    pub fn pre_update(&mut self, dt: DeltaTime) -> Option<EnemyStateMachine> {
        match self {
            EnemyStateMachine::Dying(dying_so_far) => {
                Some(EnemyStateMachine::Dying(*dying_so_far - dt.as_microseconds()))
            },
            _ => None
        }
    }

    pub fn take_attack(&self, attack: Attack, enemy_state: &mut EnemyState) {
        if let EnemyStateMachine::Base = self {
            enemy_state.take_attack(attack);
        }
    }

    pub fn post_update(&mut self, config: &EnemyConfig, audio: &AudioPlayer, enemy_state: &EnemyState) -> Option<EnemyStateMachine> {
        match self {
            EnemyStateMachine::Base if !enemy_state.health().alive() => {
                Some(EnemyStateMachine::Dying(config.enemy_dying_duration_micros))
            },
            EnemyStateMachine::Dying(dying_elapsed) if *dying_elapsed <= 0 => {
                audio.play_sound(Sound::Raygun);
                Some(EnemyStateMachine::Dead)
            },
            _ => None
        }
    }

    pub fn queue_draw(&self, config: &EnemyConfig, enemy_state: &EnemyState, sprite_renderer: &mut LightDependentSpriteRenderer) {
        let image_name = match self {
            EnemyStateMachine::Dying(_) => String::from("enemy_dying.png"),
            _ => String::from("enemy.png")
        };

        if let Some(position) = enemy_state.position() {
            let world_bottom_center_position = glm::vec3(position.x as f32, 0.0, -position.y as f32);
            let world_half_size = glm::vec2(config.enemy_physical_radius as f32, 2.0 * config.enemy_physical_radius as f32);

            sprite_renderer.queue(vec![LightDependentSpriteData {
                world_bottom_center_position,
                world_half_size,
                sprite_frame_id: SpriteSheetFrameId {
                    name: image_name,
                    sprite_sheet: NamedSpriteSheet::SpriteSheet1,
                },
                frame: 0,
            }]);
        }
    }
}
