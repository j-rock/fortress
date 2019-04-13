use crate::{
    dimensions::time::DeltaTime,
    physics::PhysicsSimulation,
    players::{
        PlayerStats,
        PlayerId,
        PlayerConfig,
        state::PlayerBody
    },
    render::{
        NamedTexture,
        SpriteData,
        SpriteRenderer
    },
};
use nalgebra::Point2;

pub struct PlayerState {
    player_id: PlayerId,
    spawn: Point2<f64>,
    _stats: PlayerStats,
    body: PlayerBody,
}

impl PlayerState {
    pub fn new(player_id: PlayerId, config: &PlayerConfig, spawn: Point2<f64>, physics_sim: &mut PhysicsSimulation) -> PlayerState {
        let body = PlayerBody::new(config, spawn, physics_sim);
        let stats = PlayerStats::new(config);
        PlayerState {
            player_id,
            spawn,
            _stats: stats,
            body,
        }
    }

    pub fn pre_update(&mut self, _dt: DeltaTime) {
        /* self.crossbow.pre_update(dt); */
    }

    /*
    pub fn try_fire(&mut self, _audio: &AudioPlayer) {
        let curr_pos = self.get_body_position();
        let curr_dir = self.get_facing_dir();
        let offset = self.config.crossbow_body_offset;
        let start_position = match curr_dir {
            LrDirection::Left => Vec2::new(curr_pos.x - offset.0, curr_pos.y + offset.1),
            LrDirection::Right => Vec2::new(curr_pos.x + offset.0, curr_pos.y + offset.1),
        };

        self.crossbow.try_fire(audio, start_position, curr_dir);
    }
    */

    pub fn redeploy(&mut self, config: &PlayerConfig, physics_sim: &mut PhysicsSimulation) {
        self.body = PlayerBody::new(config, self.spawn.clone(), physics_sim);
    }

    pub fn respawn(&mut self, spawn: Point2<f64>) {
        self.spawn = spawn;
        self.body.teleport_to(self.spawn.clone());
    }

    pub fn get_player_id(&self) -> PlayerId {
        self.player_id
    }

    pub fn draw(&self, config: &PlayerConfig, sprite_renderer: &mut SpriteRenderer) {
        if let Some(position) = self.body.position() {
            let world_bottom_center_position = glm::vec3(position.x as f32, 0.0, -position.y as f32);
            let world_half_size = glm::vec2(config.physical_radius as f32, 2.0 * config.physical_radius as f32);

            sprite_renderer.queue(NamedTexture::SpriteSheet1, &[SpriteData {
                world_bottom_center_position,
                world_half_size,
                tex_bottom_left: glm::vec2(0.0001, 0.0001),
                tex_top_right: glm::vec2(0.9999, 0.9999),
            }]);
        }
    }
}