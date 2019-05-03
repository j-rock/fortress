use crate::{
    audio::AudioPlayer,
    dimensions::{
        Attack,
        OctoDirection,
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
        FullyIlluminatedSpriteRenderer,
        PointLight,
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

    pub fn populate_lights(&self, config: &PlayerConfig, lights: &mut Vec<PointLight>) {
        self.weapon.populate_lights(config, lights);
    }

    pub fn queue_draw_weapon(&self, config: &PlayerConfig, full_light: &mut FullyIlluminatedSpriteRenderer) {
        self.weapon.queue_draw(config, full_light);
    }

    pub fn set_velocity(&mut self, dir: Option<OctoDirection>) {
        match dir {
            None => self.body.set_velocity(Vector2::new(0.0, 0.0)),
            Some(dir) => {
                self.facing_dir = dir.to_direction();
                self.body.set_velocity(self.stats.get_move_speed() * self.facing_dir);
            },
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

    pub fn bullet_attack(&self, bullet_id: BulletId) -> Option<Attack> {
        self.weapon.bullet_attack(bullet_id)
    }

    pub fn position(&self) -> Option<Point2<f64>> {
        self.body.position()
    }

    pub fn facing_dir(&self) -> Vector2<f64> {
        self.facing_dir
    }
}