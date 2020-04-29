use crate::{
    audio::{
        AudioPlayer,
        Sound,
    },
    dimensions::Reverse,
    items::{
        barrels::{
            BarrelBody,
            BarrelConfig,
            BarrelId,
        },
        ItemPickup,
    },
    math::RandGen,
    particles::{
        ParticleEvent,
        ParticleSystem,
    },
    physics::PhysicsSimulation,
    render::{
        FullyIlluminatedSpriteData,
        FullyIlluminatedSpriteRenderer,
        NamedSpriteSheet,
        SpriteSheetFrameId,
    },
};
use nalgebra::{
    Point2,
    Vector2,
};

pub struct Barrel {
    body: BarrelBody,
    strike: StrikeInfo,
}

impl Barrel {
    pub fn new(config: &BarrelConfig, id: BarrelId, location: Point2<f64>, physics_sim: &mut PhysicsSimulation) -> Self {
        let body = BarrelBody::new(config, id, location, physics_sim);
        let strike = StrikeInfo::new(config.num_strikes_health);

        Barrel {
            body,
            strike,
        }
    }

    pub fn pre_update(&mut self) {
        self.strike.pre_update();
    }

    pub fn queue_draw(&self, config: &BarrelConfig, renderer: &mut FullyIlluminatedSpriteRenderer) {
        if let Some(position) = self.body.position() {
            let world_half_size = glm::vec2(
                config.physical_radius as f32 * config.render_scale.0,
                config.physical_radius as f32 * config.render_scale.1);
            let world_center_position = glm::vec3(position.x as f32, world_half_size.y, -position.y as f32);

            renderer.queue(Some(FullyIlluminatedSpriteData {
                world_center_position,
                world_half_size,
                sprite_frame_id: SpriteSheetFrameId::new(String::from("jar.png"), NamedSpriteSheet::SpriteSheet1),
                frame: 0,
                unit_world_rotation: Vector2::new(0.0, 0.0),
                reverse: Reverse::none(),
                bloom_intensity: config.bloom_intensity,
            }));
        }
    }

    pub fn strike(&mut self, config: &BarrelConfig, audio: &AudioPlayer, particles: &mut ParticleSystem) {
        if !self.strike.strike() {
            return;
        }
        if self.is_expired() {
            audio.play_sound(Sound::BarrelDestroy);
        }
        audio.play_sound(Sound::BarrelHit);

        if let Some(position) = self.body.position() {
            let color = glm::vec3(config.blood_color.0, config.blood_color.1, config.blood_color.2);
            particles.queue_event(ParticleEvent::blood(position, color, config.num_blood_particles_per_hit));
        }
    }

    pub fn produce_item_on_death(&self, rng: &mut RandGen) -> Option<(ItemPickup, Point2<f64>)> {
        if !self.is_expired() {
            return None;
        }

        let position = self.body.position()?;
        let item_pickup = ItemPickup::random(rng);
        Some((item_pickup, position))
    }

    pub fn is_expired(&self) -> bool {
    self.strike.is_dead()
    }

    pub fn position(&self) -> Option<Point2<f64>> {
        self.body.position()
    }
}

struct StrikeInfo {
    struck_this_frame: bool,
    strikes_left: i64,
}

impl StrikeInfo {
    pub fn new(strike_amount: i64) -> Self {
        StrikeInfo {
            struck_this_frame: false,
            strikes_left: strike_amount,
        }
    }

    pub fn pre_update(&mut self) {
        self.struck_this_frame = false;
    }

    pub fn strike(&mut self) -> bool {
        if self.struck_this_frame {
            return false;
        }

        self.struck_this_frame = true;
        self.strikes_left -= 1;
        true
    }

    pub fn is_dead(&self) -> bool {
        self.strikes_left <= 0
    }
}
