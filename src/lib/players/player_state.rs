use crate::{
    audio::AudioPlayer,
    control::{
        ControlEvent::PlayerMove,
        Controller,
        ControllerId,
    },
    dimensions::{
        UpDownLeftRight,
        time::DeltaTime
    },
    physics::PhysicsSimulation,
    players::{
        PlayerStats,
        PlayerId,
        PlayerConfig,
        state::PlayerBody
    },
    render::{
        NamedSpriteSheet,
        PointLight,
        SpriteData,
        SpriteRenderer,
        SpriteSheetTexelId,
    },
    weapons::{
        BulletId,
        Weapon
    },
};
use nalgebra::{
    Point2,
    Vector2,
};

pub struct PlayerState {
    player_id: PlayerId,
    spawn: Point2<f64>,
    stats: PlayerStats,
    body: PlayerBody,

    facing_dir: Vector2<f64>,
    weapon_physical_offset: f64,
    weapon: Weapon,
}

impl PlayerState {
    pub fn new(player_id: PlayerId, config: &PlayerConfig, spawn: Point2<f64>, physics_sim: &mut PhysicsSimulation) -> PlayerState {
        let body = PlayerBody::new(config, spawn, physics_sim);
        let stats = PlayerStats::new(config);
        let weapon = Weapon::new(config, physics_sim);
        PlayerState {
            player_id,
            spawn,
            stats,
            body,
            facing_dir: Vector2::new(1.0, 0.0),
            weapon_physical_offset: config.weapon_physical_offset,
            weapon,
        }
    }

    pub fn pre_update(&mut self, dt: DeltaTime) {
        self.weapon.pre_update(dt);
    }

    pub fn post_update(&mut self) {
        self.weapon.post_update();
    }

    pub fn redeploy(&mut self, config: &PlayerConfig, physics_sim: &mut PhysicsSimulation) {
        self.body = PlayerBody::new(config, self.spawn.clone(), physics_sim);
        self.stats = PlayerStats::new(config);
        self.weapon_physical_offset = config.weapon_physical_offset;
        self.weapon = Weapon::new(config, physics_sim);
    }

    pub fn respawn(&mut self, spawn: Point2<f64>) {
        self.spawn = spawn;
        self.body.teleport_to(self.spawn.clone());
    }

    pub fn get_player_id(&self) -> PlayerId {
        self.player_id
    }

    pub fn populate_lights(&self, lights: &mut Vec<PointLight>) {
        self.weapon.populate_lights(lights);
    }

    pub fn queue_draw(&self, config: &PlayerConfig, sprite_renderer: &mut SpriteRenderer) {
        if let Some(position) = self.body.position() {
            let world_bottom_center_position = glm::vec3(position.x as f32, 0.0, -position.y as f32);
            let world_half_size = glm::vec2(config.physical_radius as f32, 2.0 * config.physical_radius as f32);

            sprite_renderer.queue(vec![SpriteData {
                world_bottom_center_position,
                world_half_size,
                sprite_texel_id: SpriteSheetTexelId {
                    name: String::from("player.png"),
                    sprite_sheet: NamedSpriteSheet::SpriteSheet1,
                },
            }]);

            self.weapon.queue_draw(sprite_renderer);
        }
    }

    pub fn set_velocity(&mut self, dir: Option<Vector2<f64>>) {
        if dir.is_none() {
            self.body.set_velocity(Vector2::new(0.0, 0.0));
            return;
        }

        if let Some(dir) = dir {
            let dir_magnitude = dir.norm();
            if !dir_magnitude.is_normal() {
                self.body.set_velocity(Vector2::new(0.0, 0.0));
                return;
            }

            self.facing_dir = dir / dir_magnitude;
            self.body.set_velocity(self.stats.get_move_speed() * self.facing_dir);
        }
    }

    pub fn try_fire(&mut self, audio: &AudioPlayer) {
        for position in self.body.position().iter() {
            let start_position = Point2::from(position.coords + self.weapon_physical_offset * self.facing_dir);
            self.weapon.try_fire(audio, self.player_id, start_position, self.facing_dir);
        }
    }

    pub fn bullet_hit(&mut self, bullet_id: BulletId) {
        self.weapon.bullet_hit(bullet_id);
    }

    pub fn compute_move_direction(controller_id: ControllerId, controller: &Controller) -> Option<Vector2<f64>> {
        let move_vert = if controller.is_pressed(controller_id, PlayerMove(UpDownLeftRight::Up)) {
            Some(1.0)
        } else if controller.is_pressed(controller_id, PlayerMove(UpDownLeftRight::Down)) {
            Some(-1.0)
        } else {
            None
        };

        let move_horiz = if controller.is_pressed(controller_id, PlayerMove(UpDownLeftRight::Left)) {
            Some(-1.0)
        } else if controller.is_pressed(controller_id, PlayerMove(UpDownLeftRight::Right)) {
            Some(1.0)
        } else {
            None
        };

        if move_vert.is_none() && move_horiz.is_none() {
            return None;
        }

        Some(Vector2::new(move_horiz.unwrap_or(0.0), move_vert.unwrap_or(0.0)))
    }
}