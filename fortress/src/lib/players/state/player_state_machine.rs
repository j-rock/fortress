use crate::{
    app::RandGen,
    audio::AudioPlayer,
    control::{
        ControlEvent,
        IdentifiedController,
    },
    dimensions::{
        Attack,
        OctoDirection,
        Reverse,
        time::{
            DeltaTime,
            Microseconds,
        },
        UpDownLeftRight,
    },
    items::{
        ItemConfig,
        ItemPickup,
    },
    particles::ParticleSystem,
    players::{
        PlayerItemConfig,
        PlayerSystemConfig,
        state::PlayerState,
    },
    render::{
        FullyIlluminatedSpriteData,
        FullyIlluminatedSpriteRenderer,
        LightDependentSpriteData,
        LightDependentSpriteRenderer,
        NamedSpriteSheet,
        PointLights,
        ScreenShake,
        SpriteSheetFrameId,
    },
    weapons::BulletId,
};
use nalgebra::{
    Point2,
    Vector2,
};

pub enum PlayerStateMachine {
    Idle(Microseconds),
    Walking(Microseconds),
}

impl PlayerStateMachine {
    pub fn new() -> PlayerStateMachine {
        PlayerStateMachine::Idle(0)
    }

    pub fn pre_update<'a>(&mut self,
                          config: &PlayerSystemConfig,
                          audio: &AudioPlayer,
                          controller: IdentifiedController<'a>,
                          dt: DeltaTime,
                          particles: &mut ParticleSystem,
                          rng: &mut RandGen,
                          shake: &mut ScreenShake,
                          player_state: &mut PlayerState) -> Option<PlayerStateMachine> {
        let move_direction = Self::compute_move_direction(controller);
        player_state.pre_update(dt);
        let velocity_was_set = player_state.try_set_velocity(config, move_direction);

        if controller.is_pressed(ControlEvent::PlayerFireSpecial) {
            player_state.try_fire_special(config, audio, rng, shake);
        }
        if controller.is_pressed(ControlEvent::PlayerFireWeapon) {
            player_state.try_fire(config, audio, rng);
        }
        if controller.is_pressed(ControlEvent::PlayerSwitchHero) {
            player_state.try_switch_hero(&config.player, audio, particles, shake);
        }

        match self {
            Self::Idle(time_elapsed) => {
                *time_elapsed += dt.as_microseconds();
                if move_direction.is_some() && velocity_was_set {
                    return Some(Self::Walking(0));
                }
            },
            Self::Walking(time_elapsed) => {
                *time_elapsed += dt.as_microseconds();
                if move_direction.is_none() || !velocity_was_set {
                    return Some(Self::Idle(0));
                }
            },
        }

        None
    }

    pub fn post_update(&self, player_state: &mut PlayerState) -> Option<PlayerStateMachine> {
        player_state.post_update();
        None
    }

    pub fn populate_lights(&self,
                           config: &PlayerSystemConfig,
                           item_config: &ItemConfig,
                           player_state: &PlayerState,
                           lights: &mut PointLights) {
        player_state.populate_lights(config, item_config, lights);
    }

    pub fn queue_draw(&self, config: &PlayerSystemConfig, player_state: &PlayerState, full_light: &mut FullyIlluminatedSpriteRenderer, light_dependent: &mut LightDependentSpriteRenderer) {
        player_state.queue_draw(config, full_light);

        if let Some(position) = player_state.position() {
            if let Some(hero_config) = config.hero.get(&player_state.hero()) {
                let (reverse, render_offset) = if player_state.lr_dir().is_left() {
                    (Reverse::horizontally(), glm::vec2(-hero_config.render_offset.0, hero_config.render_offset.1))
                } else {
                    (Reverse::none(), glm::vec2(hero_config.render_offset.0, hero_config.render_offset.1))
                };

                let world_half_size =
                    glm::vec2(config.player.physical_radius as f32 * hero_config.render_scale.0,
                              config.player.physical_radius as f32 * hero_config.render_scale.1);
                let world_center_position = glm::vec3(position.x as f32 + render_offset.x, world_half_size.y, -(position.y as f32 + render_offset.y));

                let image_name = match self {
                    PlayerStateMachine::Idle(_) => hero_config.idle_image_name.clone(),
                    PlayerStateMachine::Walking(_) => hero_config.walking_image_name.clone(),
                };

                let frame = match self {
                    PlayerStateMachine::Idle(time_elapsed) => (*time_elapsed / hero_config.idle_frame_duration_micros) as usize,
                    PlayerStateMachine::Walking(time_elapsed) => (*time_elapsed / hero_config.walking_frame_duration_micros) as usize,
                };

                light_dependent.queue(LightDependentSpriteData {
                    world_center_position,
                    world_half_size,
                    sprite_frame_id: SpriteSheetFrameId::new(image_name, NamedSpriteSheet::Heroes),
                    frame,
                    unit_world_rotation: Vector2::new(0.0, 0.0),
                    reverse,
                });

                if let Some(ref render_extra_config) = hero_config.render_extra {
                    let image_extra_name = match self {
                        PlayerStateMachine::Idle(_) => render_extra_config.idle_image_extra_name.clone(),
                        PlayerStateMachine::Walking(_) => render_extra_config.walking_image_extra_name.clone(),
                    };

                    full_light.queue(Some(FullyIlluminatedSpriteData {
                        world_center_position,
                        world_half_size,
                        sprite_frame_id: SpriteSheetFrameId::new(image_extra_name, NamedSpriteSheet::Heroes),
                        frame,
                        unit_world_rotation: Vector2::new(0.0, 0.0),
                        reverse,
                        bloom_intensity: render_extra_config.bloom_intensity,
                    }));
                }
            }
        }
    }

    // Returns bullet direction.
    pub fn bullet_hit(&self, bullet_id: BulletId, player_state: &mut PlayerState) -> Option<Vector2<f64>> {
        player_state.bullet_hit(bullet_id)
    }

    pub fn bullet_attack(&self, player_state: &PlayerState, bullet_id: BulletId) -> Option<Attack> {
        player_state.bullet_attack(bullet_id)
    }

    pub fn position(&self, player_state: &PlayerState) -> Option<Point2<f64>> {
        player_state.position()
    }

    pub fn collect_item(&self, config: &PlayerItemConfig, item_pickup: ItemPickup, player_state: &mut PlayerState) {
        player_state.collect_item(config, item_pickup);
    }

    fn compute_move_direction(controller: IdentifiedController) -> Option<OctoDirection> {
        let up = controller.is_pressed(ControlEvent::PlayerMove(UpDownLeftRight::Up));
        let down = controller.is_pressed(ControlEvent::PlayerMove(UpDownLeftRight::Down));
        let left = controller.is_pressed(ControlEvent::PlayerMove(UpDownLeftRight::Left));
        let right = controller.is_pressed(ControlEvent::PlayerMove(UpDownLeftRight::Right));

        OctoDirection::from(up, down, left, right)
    }
}