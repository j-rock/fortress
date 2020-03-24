use crate::{
    app::RandGen,
    audio::{
        AudioPlayer,
        Sound,
    },
    dimensions::{
        Attack,
        LrDirection,
        OctoDirection,
        time::{
            DeltaTime,
            Microseconds,
        }
    },
    items::ItemPickup,
    particles::{
        ParticleEvent,
        ParticleSystem,
    },
    physics::PhysicsSimulation,
    players::{
        Hero,
        PlayerId,
        PlayerSystemConfig,
        state::{
            PlayerBody,
            PlayerStats,
        }
    },
    render::{
        FullyIlluminatedSpriteRenderer,
        PointLights,
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
    hero: Hero,

    facing_dir: Vector2<f64>,
    lr_dir: LrDirection,
    weapon_physical_offset: f64,
    weapon: Weapon,

    hero_switch_time_left: Option<Microseconds>,
}

impl PlayerState {
    pub fn new(player_id: PlayerId, config: &PlayerSystemConfig, spawn: Point2<f64>, physics_sim: &mut PhysicsSimulation) -> PlayerState {
        let body = PlayerBody::new(&config.player, player_id, spawn, physics_sim);
        let stats = PlayerStats::new(config);
        let weapon = Weapon::new(&config.bullet, physics_sim);
        PlayerState {
            player_id,
            spawn,
            hero: Hero::CapedWarrior,
            stats,
            body,
            facing_dir: Vector2::new(1.0, 0.0),
            lr_dir: LrDirection::Right,
            weapon_physical_offset: config.player.weapon_physical_offset,
            weapon,
            hero_switch_time_left: None,
        }
    }

    pub fn pre_update(&mut self, config: &PlayerSystemConfig, dt: DeltaTime) {
        self.weapon.pre_update(&config.bullet, &self.stats, dt);
        self.stats.pre_update(&config.item, dt);

        if let Some(time_left) = self.hero_switch_time_left {
            let new_time_left = time_left - dt.as_microseconds();
            if new_time_left < 0 {
                self.hero_switch_time_left = None;
            } else {
                self.hero_switch_time_left = Some(new_time_left);
            }
        }
    }

    pub fn post_update(&mut self) {
        self.weapon.post_update();
    }

    pub fn redeploy(&mut self, config: &PlayerSystemConfig, physics_sim: &mut PhysicsSimulation) {
        self.body = PlayerBody::new(&config.player, self.player_id, self.spawn.clone(), physics_sim);
        self.stats = PlayerStats::new(config);
        self.weapon_physical_offset = config.player.weapon_physical_offset;
        self.weapon = Weapon::new(&config.bullet, physics_sim);
    }

    pub fn respawn(&mut self, spawn: Point2<f64>) {
        self.spawn = spawn;
        self.body.teleport_to(self.spawn.clone());
    }

    pub fn get_player_id(&self) -> PlayerId {
        self.player_id
    }

    pub fn populate_lights(&self, config: &PlayerSystemConfig, lights: &mut PointLights) {
        self.weapon.populate_lights(&config.bullet, lights);
        if let Some(position) = self.position() {
            self.stats.populate_lights(&config.item, position, lights);
        }
    }

    pub fn queue_draw_weapon(&self, config: &PlayerSystemConfig, full_light: &mut FullyIlluminatedSpriteRenderer) {
        self.weapon.queue_draw(&config.bullet, full_light);
    }

    pub fn queue_draw_stats(&self, config: &PlayerSystemConfig, full_light: &mut FullyIlluminatedSpriteRenderer) {
        if let Some(position) = self.position() {
            self.stats.queue_draw(&config.item, position, full_light);
        }
    }

    pub fn set_velocity(&mut self, config: &PlayerSystemConfig, dir: Option<OctoDirection>) {
        match dir {
            None => self.body.set_velocity(Vector2::new(0.0, 0.0)),
            Some(dir) => {
                self.facing_dir = dir.to_direction();
                if let Some(lr_dir) = dir.to_lr_direction() {
                    self.lr_dir = lr_dir;
                }

                if let Some(hero) = config.hero.get(&self.hero()) {
                    self.body.set_velocity(self.facing_dir.clone() * hero.base_move_speed);
                }
            },
        }
    }

    pub fn try_fire(&mut self, audio: &AudioPlayer, rng: &mut RandGen) {
        if let Some(position) = self.position() {
            let start_position = Point2::from(position.coords + self.weapon_physical_offset * self.facing_dir);
            self.weapon.try_fire_normal(audio, &self.stats, self.player_id, start_position, self.facing_dir, rng);
        }
    }

    pub fn try_fire_special(&mut self, config: &PlayerSystemConfig, audio: &AudioPlayer, rng: &mut RandGen) {
        if let Some(position) = self.position() {
            let start_position = Point2::from(position.coords + self.weapon_physical_offset * self.facing_dir);
            self.weapon.try_fire_special(&config.bullet, audio, &self.stats, self.player_id, start_position, self.facing_dir, rng);
        }
    }

    pub fn try_switch_hero(&mut self, config: &PlayerSystemConfig, audio: &AudioPlayer, particles: &mut ParticleSystem) {
        if let None = self.hero_switch_time_left {
            self.hero_switch_time_left = Some(config.player.switch_hero_duration_micros);

            self.hero = match self.hero {
                Hero::CapedWarrior => Hero::FireMage,
                Hero::FireMage => Hero::Barbarian,
                Hero::Barbarian => Hero::Rogue,
                Hero::Rogue => Hero::CapedWarrior,
            };
            self.weapon.switch_bullet_element();
            audio.play_sound(Sound::HeroSwitch);
            if let Some(position) = self.position() {
                particles.queue_event(ParticleEvent::hero_switch(position));
            }
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

    pub fn hero(&self) -> Hero {
        self.hero
    }
}