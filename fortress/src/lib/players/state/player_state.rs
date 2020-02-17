use crate::{
    app::RandGen,
    audio::AudioPlayer,
    dimensions::{
        Attack,
        LrDirection,
        OctoDirection,
        time::DeltaTime
    },
    items::ItemPickup,
    physics::PhysicsSimulation,
    players::{
        PlayerId,
        PlayerConfig,
        state::{
            PlayerBody,
            PlayerStats,
        }
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
    lr_dir: LrDirection,
    weapon_physical_offset: f64,
    weapon: Weapon,
}

impl PlayerState {
    pub fn new(player_id: PlayerId, config: &PlayerConfig, spawn: Point2<f64>, physics_sim: &mut PhysicsSimulation) -> PlayerState {
        let body = PlayerBody::new(config, player_id, spawn, physics_sim);
        let stats = PlayerStats::new(config);
        let weapon = Weapon::new(config, physics_sim);
        PlayerState {
            player_id,
            spawn,
            stats,
            body,
            facing_dir: Vector2::new(1.0, 0.0),
            lr_dir: LrDirection::Right,
            weapon_physical_offset: config.weapon_physical_offset,
            weapon,
        }
    }

    pub fn pre_update(&mut self, config: &PlayerConfig, dt: DeltaTime) {
        self.weapon.pre_update(config, &self.stats, dt);
        self.stats.pre_update(config, dt);
    }

    pub fn post_update(&mut self) {
        self.weapon.post_update();
    }

    pub fn redeploy(&mut self, config: &PlayerConfig, physics_sim: &mut PhysicsSimulation) {
        self.body = PlayerBody::new(config, self.player_id, self.spawn.clone(), physics_sim);
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
        if let Some(position) = self.position() {
            self.stats.populate_lights(config, position, lights);
        }
    }

    pub fn queue_draw_weapon(&self, config: &PlayerConfig, full_light: &mut FullyIlluminatedSpriteRenderer) {
        self.weapon.queue_draw(config, full_light);
    }

    pub fn queue_draw_stats(&self, config: &PlayerConfig, full_light: &mut FullyIlluminatedSpriteRenderer) {
        if let Some(position) = self.position() {
            self.stats.queue_draw(config, position, full_light);
        }
    }

    pub fn set_velocity(&mut self, dir: Option<OctoDirection>) {
        match dir {
            None => self.body.set_velocity(Vector2::new(0.0, 0.0)),
            Some(dir) => {
                self.facing_dir = dir.to_direction();
                if let Some(lr_dir) = dir.to_lr_direction() {
                    self.lr_dir = lr_dir;
                }
                self.body.set_velocity(self.stats.get_move_speed() * self.facing_dir);
            },
        }
    }

    pub fn try_fire(&mut self, audio: &AudioPlayer, rng: &mut RandGen) {
        if let Some(position) = self.position() {
            let start_position = Point2::from(position.coords + self.weapon_physical_offset * self.facing_dir);
            self.weapon.try_fire_normal(audio, &self.stats, self.player_id, start_position, self.facing_dir, rng);
        }
    }

    pub fn try_fire_special(&mut self, config: &PlayerConfig, audio: &AudioPlayer, rng: &mut RandGen) {
        if let Some(position) = self.position() {
            let start_position = Point2::from(position.coords + self.weapon_physical_offset * self.facing_dir);
            self.weapon.try_fire_special(config, audio, &self.stats, self.player_id, start_position, self.facing_dir, rng);
        }
    }

    pub fn bullet_hit(&mut self, bullet_id: BulletId) {
        self.weapon.bullet_hit(bullet_id);
    }

    pub fn bullet_attack(&self, bullet_id: BulletId) -> Option<Attack> {
        self.weapon.bullet_attack(&self.stats, bullet_id)
    }

    pub fn position(&self) -> Option<Point2<f64>> {
        self.body.position()
    }

    pub fn lr_dir(&self) -> LrDirection {
        self.lr_dir
    }

    pub fn collect_item(&mut self, item_pickup: ItemPickup) {
        self.stats.collect_item(item_pickup);
    }
}