use crate::{
    audio::AudioPlayer,
    control::{
        ControlEvent,
        Controller,
        ControllerId,
    },
    dimensions::{
        Attack,
        OctoDirection,
        time::{
            DeltaTime,
            Microseconds,
        },
        UpDownLeftRight,
    },
    players::{
        PlayerConfig,
        state::PlayerState,
    },
    render::{
        FullyIlluminatedSpriteRenderer,
        LightDependentSpriteData,
        LightDependentSpriteRenderer,
        NamedSpriteSheet,
        PointLight,
        SpriteSheetFrameId,
    },
    weapons::BulletId,
};
use nalgebra::Point2;

pub enum PlayerStateMachine {
    Idle(Microseconds),
    Walking(Microseconds),
}

impl PlayerStateMachine {
    pub fn new() -> PlayerStateMachine {
        PlayerStateMachine::Idle(0)
    }

    pub fn pre_update(&mut self, audio: &AudioPlayer, controller_id: ControllerId, controller: &Controller, dt: DeltaTime, player_state: &mut PlayerState) -> Option<PlayerStateMachine> {
        let move_direction = Self::compute_move_direction(controller_id, controller);
        player_state.pre_update(dt);
        player_state.set_velocity(move_direction);

        if controller.is_pressed(controller_id, ControlEvent::PlayerFireWeapon) {
            player_state.try_fire(audio);
        }

        match self {
            PlayerStateMachine::Idle(time_elapsed) => {
                *time_elapsed += dt.as_microseconds();
                if move_direction.is_some() {
                    return Some(PlayerStateMachine::Walking(0));
                }
            },
            PlayerStateMachine::Walking(time_elapsed) => {
                *time_elapsed += dt.as_microseconds();
                if move_direction.is_none() {
                    return Some(PlayerStateMachine::Idle(0));
                }
            }
        }

        None
    }

    pub fn post_update(&self, player_state: &mut PlayerState, _audio: &AudioPlayer) -> Option<PlayerStateMachine> {
        player_state.post_update();
        None
    }

    pub fn populate_lights(&self, config: &PlayerConfig, player_state: &PlayerState, lights: &mut Vec<PointLight>) {
        player_state.populate_lights(config, lights);
    }

    pub fn queue_draw(&self, config: &PlayerConfig, player_state: &PlayerState, full_light: &mut FullyIlluminatedSpriteRenderer, light_dependent: &mut LightDependentSpriteRenderer) {
        if let Some(position) = player_state.position() {
            let world_half_size = glm::vec2(config.physical_radius as f32 * config.player_render_scale.0, config.physical_radius as f32 * config.player_render_scale.1);
            let world_center_position = glm::vec3(position.x as f32 + config.player_render_offset.0, world_half_size.y, -(position.y as f32 + config.player_render_offset.1));

            let image_name = match self {
                PlayerStateMachine::Idle(_) => String::from("warrior_idle.png"),
                PlayerStateMachine::Walking(_) => String::from("warrior_run.png"),
            };

            let frame = match self {
                PlayerStateMachine::Idle(time_elapsed) => (*time_elapsed / config.player_idle_frame_duration_micros) as usize,
                PlayerStateMachine::Walking(time_elapsed) => (*time_elapsed / config.player_running_frame_duration_micros) as usize,
            };

            light_dependent.queue(vec![LightDependentSpriteData {
                world_center_position,
                world_half_size,
                sprite_frame_id: SpriteSheetFrameId {
                    name: image_name,
                    sprite_sheet: NamedSpriteSheet::SpriteSheet1,
                },
                frame,
                rotation: 0.0,
            }]);

            player_state.queue_draw_weapon(config, full_light);
        }
    }

    pub fn bullet_hit(&self, bullet_id: BulletId, player_state: &mut PlayerState) {
        player_state.bullet_hit(bullet_id);
    }

    pub fn bullet_attack(&self, player_state: &PlayerState, bullet_id: BulletId) -> Option<Attack> {
        player_state.bullet_attack(bullet_id)
    }

    pub fn position(&self, player_state: &PlayerState) -> Option<Point2<f64>> {
        player_state.position()
    }

    fn compute_move_direction(controller_id: ControllerId, controller: &Controller) -> Option<OctoDirection> {
        let up = controller.is_pressed(controller_id, ControlEvent::PlayerMove(UpDownLeftRight::Up));
        let down = controller.is_pressed(controller_id, ControlEvent::PlayerMove(UpDownLeftRight::Down));
        let left = controller.is_pressed(controller_id, ControlEvent::PlayerMove(UpDownLeftRight::Left));
        let right = controller.is_pressed(controller_id, ControlEvent::PlayerMove(UpDownLeftRight::Right));

        OctoDirection::from(up, down, left, right)
    }

}