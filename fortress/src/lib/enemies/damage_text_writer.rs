use crate::{
    enemies::DamageTextConfig,
    dimensions::{
        Damage,
        time::{
            DeltaTime,
            Timer
        },
    },
    math::EasingFn,
    text::{
        TextContent,
        TextRenderer,
        WorldTextRequest,
    },
};
use glm;
use nalgebra::{
    Point2,
    Vector2,
};

pub struct DamageTextWriter {
    damage: Vec<Damage>,
    position: Vec<glm::Vec3>,
    velocity: Vec<glm::Vec3>,
    timer: Vec<Timer>,
}

impl DamageTextWriter {
    pub fn new(config: &DamageTextConfig) -> Self {
        DamageTextWriter {
            damage: Vec::with_capacity(config.initial_capacity),
            position: Vec::with_capacity(config.initial_capacity),
            velocity: Vec::with_capacity(config.initial_capacity),
            timer: Vec::with_capacity(config.initial_capacity),
        }
    }

    pub fn pre_update(&mut self, config: &DamageTextConfig, dt: DeltaTime) {
        let float_dt = dt.as_f32_seconds();
        let vertical_acceleration = config.vertical_acceleration * float_dt;

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

    pub fn add_damage(&mut self, config: &DamageTextConfig, damage: Damage, position: Point2<f64>, direction: Option<Vector2<f64>>) {
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

    pub fn queue_draw(&self, config: &DamageTextConfig, text: &mut TextRenderer) {
        (0..self.damage.len())
            .for_each(|idx| {
                let damage = self.damage[idx];
                if let Some(color) = config.color.get(&damage.criticality()) {
                    let content = [TextContent::Number(damage.value())];
                    let world_position = self.position[idx];

                    let alpha = {
                        let t = self.timer[idx].as_completion_fraction_of(config.text_expiry_duration_micros);
                        EasingFn::ease_out_quintic(1.0 - t)
                    };

                    text.queue_world_text(content.iter().copied(), WorldTextRequest {
                        world_position: world_position + glm::vec3(config.shadow_offset.0, config.shadow_offset.1, config.shadow_offset.2),
                        raster_size: config.raster_size,
                        color: glm::vec3(config.shadow_color.0, config.shadow_color.1, config.shadow_color.2),
                        alpha,
                    });
                    text.queue_world_text(content.iter().copied(), WorldTextRequest {
                        world_position,
                        raster_size: config.raster_size,
                        color: glm::vec3(color.0, color.1, color.2),
                        alpha,
                    });
                }
            });
    }

    fn swap_delete(&mut self, index: usize) {
        self.damage.swap_remove(index);
        self.position.swap_remove(index);
        self.velocity.swap_remove(index);
        self.timer.swap_remove(index);
    }
}