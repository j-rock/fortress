use crate::{
    app::StatusOr,
    file::{
        ConfigWatcher,
        SimpleConfigManager,
    },
    dimensions::{
        Damage,
        time::{
            DeltaTime,
            Timer
        },
    },
    text::{
        TextContent,
        TextRenderer,
        WorldTextRequest,
    },
    world::DamageTextWriterConfig,
};
use glm;
use nalgebra::{
    Point2,
    Vector2,
};

pub struct DamageTextWriter {
    config: SimpleConfigManager<DamageTextWriterConfig>,
    damage: Vec<Damage>,
    position: Vec<glm::Vec3>,
    velocity: Vec<glm::Vec3>,
    timer: Vec<Timer>,
}

impl DamageTextWriter {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<Self> {
        let config = SimpleConfigManager::<DamageTextWriterConfig>::from_config_resource(config_watcher, "damage_text.conf")?;
        let capacity = config.get().initial_capacity;

        Ok(DamageTextWriter {
            config,
            damage: Vec::with_capacity(capacity),
            position: Vec::with_capacity(capacity),
            velocity: Vec::with_capacity(capacity),
            timer: Vec::with_capacity(capacity),
        })
    }

    pub fn pre_update(&mut self, dt: DeltaTime) {
        self.config.update();
        let float_dt = dt.as_f32_seconds();
        let vertical_acceleration = self.config.get().vertical_acceleration * float_dt;

        (0..self.damage.len())
            .rev()
            .for_each(|idx| {
                let ref mut timer = self.timer[idx];
                timer.tick(dt);
                if timer.is_expired() {
                    self.swap_delete(idx);
                    return;
                }

                let ref mut velocity = self.velocity[idx];
                velocity.y += vertical_acceleration;
                let ref mut position = self.position[idx];
                *position = (*position) + (*velocity * float_dt);
            });
    }

    pub fn add_damage(&mut self, damage: Damage, position: Point2<f64>, direction: Option<Vector2<f64>>) {
        let config = self.config.get();

        let velocity = glm::vec2(config.start_velocity.0, config.start_velocity.2) * if let Some(world_direction) = direction {
            glm::vec2(world_direction.x as f32, -world_direction.y as f32)
        } else {
            glm::vec2(0.0, 0.0)
        };

        self.damage.push(damage);
        self.position.push(glm::vec3(position.x as f32, config.start_height, -position.y as f32));
        self.velocity.push(glm::vec3(velocity.x, config.start_velocity.1, velocity.y));
        self.timer.push(Timer::new(config.text_expiry_duration_micros));
    }

    pub fn queue_draw(&self, text: &mut TextRenderer) {
        let config = self.config.get();

        (0..self.damage.len())
            .for_each(|idx| {
                let damage_value = self.damage[idx].value();
                let content = [TextContent::Number(damage_value)];
                let world_position = self.position[idx];
                text.queue_world_text(content.iter().copied(), WorldTextRequest {
                    world_position: world_position + glm::vec3(config.shadow_offset.0, config.shadow_offset.1, config.shadow_offset.2),
                    raster_size: config.raster_size,
                    color: glm::vec3(config.shadow_color.0, config.shadow_color.1, config.shadow_color.2),
                    alpha: 1.0,
                });
                text.queue_world_text(content.iter().copied(), WorldTextRequest {
                    world_position,
                    raster_size: config.raster_size,
                    color: glm::vec3(config.color.0, config.color.1, config.color.2),
                    alpha: 1.0,
                });
            });
    }

    fn swap_delete(&mut self, index: usize) {
        self.damage.swap_remove(index);
        self.position.swap_remove(index);
        self.velocity.swap_remove(index);
        self.timer.swap_remove(index);
    }
}