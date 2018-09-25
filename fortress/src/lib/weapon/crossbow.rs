use audio::{
    AudioPlayer,
    Sound,
};
use dimensions::{
    Attack,
    Damage,
    LrDirection,
    time::{
        self,
        DeltaTime,
    }
};
use entity::{
    Entity,
    EntityRegistrar,
    EntityType,
    RegisteredBody,
};
use glm;
use liquidfun::box2d::{
    collision::shapes::polygon_shape::PolygonShape,
    common::math::Vec2,
    dynamics::{
        body::{
            Body,
            BodyDef,
            BodyType,
        },
        fixture::{
            FixtureDef
        },
        world::World,
    },
};
use physics::{
    collision_category,
    CollisionMatcher,
};
use player::PlayerConfig;
use render::{
    BoxData,
    BoxRenderer
};
use slab::Slab;
use wraith::Wraith;

type ArrowId = usize;

struct Arrow {
    body: RegisteredBody,
}

pub struct Crossbow {
    arrows: Slab<Arrow>,

    arrow_speed: Vec2,
    arrow_damage: Damage,
    arrow_knockback_strength: f32,
    firing_period: time::Microseconds,
    current_delay: Option<time::Microseconds>,
    arrow_box_size: Vec2,

    registrar: EntityRegistrar,
    world: World,
}

impl Crossbow {
    pub fn new(config: &PlayerConfig, registrar: &EntityRegistrar, world: &mut World) -> Crossbow {
        Crossbow {
            arrows: Slab::new(),
            arrow_speed: Vec2::new(config.arrow_speed.0, config.arrow_speed.1),
            arrow_damage: config.arrow_damage,
            arrow_knockback_strength: config.arrow_knockback_strength,
            firing_period: time::milliseconds(config.firing_period_ms),
            current_delay: None,
            arrow_box_size: Vec2::new(config.arrow_box_size.0, config.arrow_box_size.1),
            registrar: registrar.clone(),
            world: world.clone(),
        }
    }

    pub fn pre_update(&mut self, dt: DeltaTime) {
        if let Some(delay) = self.current_delay {
            let new_delay = delay - dt.as_microseconds();
            self.current_delay = if new_delay <= 0 {
                None
            } else {
                Some(new_delay)
            };
        }
    }

    pub fn try_fire(&mut self, audio: &AudioPlayer, start_position: Vec2, direction: LrDirection) {
        if let None = self.current_delay {
            // Dirty trick to get next arrow id. Create vacant entry, read its key, drop it.
            let next_arrow_id = {
                self.arrows.vacant_entry().key()
            };
            let etype = EntityType::CrossbowArrow(next_arrow_id);
            let crossbow: *const Crossbow = self as *const Crossbow;
            let entity = Entity::new(etype, crossbow);

            let arrow_body = self.create_arrow_body(start_position, direction);
            let arrow = Arrow {
                body: RegisteredBody::new(arrow_body, self.registrar.clone(), Some(entity))
            };

            self.arrows.vacant_entry().insert(arrow);
            self.current_delay = Some(self.firing_period);

            audio.play_sound(Sound::Blast);
        }
    }

    pub fn get_attack(&self, arrow_id: ArrowId) -> Attack {
        let arrow = self.arrows.get(arrow_id).expect("Crossbow had bad arrow.");
        let arrow_velocity = arrow.body.data_setter.get_linear_velocity();
        let arrow_dir = if arrow_velocity.x <= 0.0 {
            LrDirection::Left
        } else {
            LrDirection::Right
        };

        Attack {
            damage: self.arrow_damage,
            knockback_strength: self.arrow_knockback_strength,
            knockback_dir: arrow_dir
        }
    }

    pub fn remove_arrow(&mut self, arrow_id: ArrowId) {
        self.arrows.remove(arrow_id);
    }

    pub fn arrow_hit() -> CollisionMatcher {
        CollisionMatcher::fuzzy_match_two(Box::new(|etype| {
            if let EntityType::CrossbowArrow(_x) = etype {
                true
            } else {
                false
            }
        }), Box::new(|_etype| {
            true
        }), Box::new(|_audio, crossbow_entity, other_entity| {
            let arrow_id = if let EntityType::CrossbowArrow(x) = crossbow_entity.etype() {
                x
            } else {
                panic!("Crossbow collision matcher is broken.");
            };

            let crossbow: &mut Self = crossbow_entity.resolve();

            if other_entity.etype() == EntityType::Wraith {
                let wraith: &mut Wraith = other_entity.resolve();

                let arrow_attack = crossbow.get_attack(arrow_id);
                wraith.take_attack(arrow_attack);
            }

            crossbow.remove_arrow(arrow_id);
        }))
    }

    fn create_arrow_body(&mut self, start_position: Vec2, direction: LrDirection) -> Body {
        let mut body_def = BodyDef::default();
        body_def.body_type = BodyType::DynamicBody;
        body_def.position = start_position;
        body_def.linear_velocity = match direction {
            LrDirection::Left => Vec2::new(-self.arrow_speed.x, self.arrow_speed.y),
            LrDirection::Right => self.arrow_speed,
        };
        body_def.fixed_rotation = true;
        body_def.bullet = true;

        let body = self.world.create_body(&body_def);

        // Arrow body fixture
        let mut poly_shape = PolygonShape::new();
        {
            let (hx, hy) = (self.arrow_box_size.x / 2.0, self.arrow_box_size.y / 2.0);
            poly_shape.set_as_box(hx, hy);

            let mut fixture_def = FixtureDef::new(&poly_shape);
            fixture_def.is_sensor = true;
            fixture_def.filter.category_bits = collision_category::PLAYER_WEAPON;
            fixture_def.filter.mask_bits = collision_category::BARRIER | collision_category::WRAITH;
            body.create_fixture(&fixture_def);
        }

        body
    }

    pub fn draw(&self, box_renderer: &mut BoxRenderer) {
        let boxes: Vec<BoxData> = self.arrows.iter().map(|arrow| -> BoxData {
            let body_position = arrow.1.body.data_setter.get_position();
            let body_size = self.arrow_box_size;
            BoxData {
                position: glm::vec2(body_position.x, body_position.y),
                half_size: glm::vec2(body_size.x, body_size.y),
                rgba_tl: glm::vec4(0.3, 0.8, 0.3, 0.0),
                rgba_tr: glm::vec4(0.0, 0.8, 0.4, 0.0),
                rgba_bl: glm::vec4(0.2, 1.0, 0.2, 0.0),
                rgba_br: glm::vec4(0.0, 1.0, 0.1, 0.0),
            }
        }).collect();
        box_renderer.queue(boxes.as_slice());
    }
}
