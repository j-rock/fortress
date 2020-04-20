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
            Timer,
        }
    },
    items::{
        ItemConfig,
        ItemPickup,
    },
    particles::{
        ParticleEvent,
        ParticleSystem,
    },
    physics::PhysicsSimulation,
    players::{
        Hero,
        PlayerBulletConfig,
        PlayerConfig,
        PlayerId,
        PlayerItemConfig,
        PlayerSystemConfig,
        state::{
            CollectedItemAnimation,
            PlayerBody,
            PlayerStats,
        }
    },
    render::{
        FullyIlluminatedSpriteRenderer,
        PointLights,
        ScreenShake,
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
    stats: PlayerStats,
    body: PlayerBody,
    hero: Hero,
    weapon: Weapon,

    frozen_from_firing_special_timer: Timer,
    hero_switch_timer: Timer,

    collected_item_animations: CollectedItemAnimation,
}

impl PlayerState {
    pub fn new(player_id: PlayerId, config: &PlayerSystemConfig, spawn: Point2<f64>, physics_sim: &mut PhysicsSimulation) -> PlayerState {
        let body = PlayerBody::new(&config.player, player_id, spawn, physics_sim);
        let stats = PlayerStats::new(&config.bullet);
        let weapon = Weapon::new(&config.bullet, physics_sim);
        PlayerState {
            player_id,
            hero: Hero::Rogue,
            stats,
            body,
            weapon,
            frozen_from_firing_special_timer: Timer::expired(),
            hero_switch_timer: Timer::expired(),
            collected_item_animations: CollectedItemAnimation::new(&config.item),
        }
    }

    pub fn pre_update(&mut self, dt: DeltaTime) {
        self.weapon.pre_update(dt);
        self.hero_switch_timer.tick(dt);
        self.frozen_from_firing_special_timer.tick(dt);
        self.collected_item_animations.pre_update(dt);
    }

    pub fn post_update(&mut self) {
        self.weapon.post_update();
    }

    pub fn redeploy(&mut self, config: &PlayerSystemConfig, physics_sim: &mut PhysicsSimulation) {
        self.body.redeploy(&config.player, self.player_id,physics_sim);
        self.stats = PlayerStats::new(&config.bullet);
        self.weapon = Weapon::new(&config.bullet, physics_sim);
    }

    pub fn respawn(&mut self, spawn: Point2<f64>) {
        self.body.respawn(spawn);
    }

    pub fn get_player_id(&self) -> PlayerId {
        self.player_id
    }

    pub fn populate_lights(&self, config: &PlayerSystemConfig, item_config: &ItemConfig, lights: &mut PointLights) {
        self.weapon.populate_lights(&config.bullet, lights);
        if let Some(position) = self.position() {
            self.collected_item_animations.populate_lights(&config.item, item_config, position, lights);
        }
    }

    pub fn queue_draw_weapon(&self, config: &PlayerBulletConfig, full_light: &mut FullyIlluminatedSpriteRenderer) {
        self.weapon.queue_draw(config, full_light);
    }

    pub fn queue_draw_collected_items(&self, config: &PlayerItemConfig, full_light: &mut FullyIlluminatedSpriteRenderer) {
        if let Some(position) = self.position() {
            self.collected_item_animations.queue_draw(config, position, full_light);
        }
    }

    pub fn try_set_velocity(&mut self, config: &PlayerSystemConfig, dir: Option<OctoDirection>) -> bool {
        if !self.frozen_from_firing_special_timer.is_expired() || dir.is_none() {
            self.body.stop_moving();
            return false;
        }

        if let Some(dir) = dir {
            self.body.update_direction(dir);
            if let Some(hero) = config.hero.get(&self.hero()) {
                let move_speed = self.stats.get_move_speed(hero);
                self.body.move_forward(move_speed);
            }
        }
        true
    }

    pub fn try_fire(&mut self, config: &PlayerSystemConfig, audio: &AudioPlayer, rng: &mut RandGen) {
        if let Some(position) = self.position() {
            let facing_dir = self.body.facing_dir();
            let start_position = Point2::from(position.coords + config.player.weapon_physical_offset * facing_dir.clone());
            if self.weapon.try_fire_normal(&config.bullet, &self.stats, self.player_id, start_position, facing_dir, rng) {
                audio.play_sound(Sound::ShootSingleFireball);
            }
        }
    }

    pub fn try_fire_special(&mut self, config: &PlayerSystemConfig, audio: &AudioPlayer, rng: &mut RandGen, shake: &mut ScreenShake) {
        if let Some(position) = self.position() {
            let facing_dir = self.body.facing_dir();
            let start_position = Point2::from(position.coords + config.player.weapon_physical_offset * facing_dir.clone());
            if !self.weapon.try_fire_special(&config.bullet, &self.stats, self.player_id, start_position, facing_dir, rng) {
                return;
            }

            self.frozen_from_firing_special_timer = Timer::new(config.player.fire_special_move_freeze_duration_micros);
            audio.play_sound(Sound::ShootSpecial);
            shake.intensify(config.bullet.special_screen_shake_intensity);

            if let Some(config) = config.hero.get(&self.hero) {
                self.body.shove_backward(config.fire_special_knockback_strength);
            }
        }
    }

    pub fn try_switch_hero(&mut self, config: &PlayerConfig, audio: &AudioPlayer, particles: &mut ParticleSystem, shake: &mut ScreenShake) {
        if !self.hero_switch_timer.is_expired() {
            return;
        }
        self.hero_switch_timer = Timer::new(config.switch_hero_duration_micros);

        self.hero = match self.hero {
            Hero::CapedWarrior => Hero::FireMage,
            Hero::FireMage => Hero::Barbarian,
            Hero::Barbarian => Hero::Rogue,
            Hero::Rogue => Hero::CapedWarrior,
        };
        self.weapon.switch_bullet_element();
        audio.play_sound(Sound::HeroSwitch);
        shake.intensify(config.switch_hero_screen_shake_intensity);
        if let Some(position) = self.position() {
            particles.queue_event(ParticleEvent::hero_switch(position));
        }
    }

    // Returns bullet direction.
    pub fn bullet_hit(&mut self, bullet_id: BulletId) -> Option<Vector2<f64>> {
        self.weapon.bullet_hit(bullet_id)
    }

    pub fn bullet_attack(&self, bullet_id: BulletId) -> Option<Attack> {
        self.weapon.bullet_attack(&self.stats, bullet_id)
    }

    pub fn position(&self) -> Option<Point2<f64>> {
        self.body.position()
    }

    pub fn lr_dir(&self) -> LrDirection {
        self.body.lr_direction()
    }

    pub fn collect_item(&mut self, config: &PlayerItemConfig, item_pickup: ItemPickup) {
        self.stats.collect_item(item_pickup);
        self.collected_item_animations.add_animation(config, item_pickup);
    }

    pub fn hero(&self) -> Hero {
        self.hero
    }

    pub fn skull_count(&self) -> usize {
        self.stats.skull_count()
    }
}