use dimensions::{
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
    Registered,
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
use std::collections::HashMap;

type ArrowId = usize;

struct Arrow {
    body: Registered<Body>,
}

pub struct Crossbow {
    world: World,

    arrow_speed: Vec2,
    arrow_box_size: Vec2,
    arrows: HashMap<ArrowId, Arrow>,
    arrow_counter: usize,

    firing_period: time::Microseconds,
    current_delay: Option<time::Microseconds>,

    registrar: EntityRegistrar,
}

impl Crossbow {
    pub fn new(config: &PlayerConfig, registrar: &EntityRegistrar, world: &mut World) -> Crossbow {
        Crossbow {
            world: world.clone(),

            arrow_speed: Vec2::new(config.arrow_speed.0, config.arrow_speed.1),
            arrow_box_size: Vec2::new(config.arrow_box_size.0, config.arrow_box_size.1),
            arrows: HashMap::new(),
            arrow_counter: 0,

            firing_period: time::milliseconds(config.firing_period_ms),
            current_delay: None,

            registrar: registrar.clone(),
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

    pub fn try_fire(&mut self, start_position: Vec2, direction: LrDirection) {
        if let None = self.current_delay {
            self.arrow_counter += 1;

            let next_arrow_id = self.arrow_counter;
            let etype = EntityType::CrossbowArrow(next_arrow_id);
            let crossbow: *const Crossbow = self as *const Crossbow;
            let entity = Entity::new(etype, crossbow);

            let arrow_body = self.create_arrow_body(start_position, direction);
            let arrow = Arrow {
                body: Registered::new(arrow_body, self.registrar.clone(), Some(entity))
            };

            self.arrows.insert(next_arrow_id, arrow);

            self.current_delay = Some(self.firing_period);
        }
    }

    pub fn remove_arrow(&mut self, arrow_id: ArrowId) {
        if let Some(mut arrow) = self.arrows.remove(&arrow_id) {
            self.world.destroy_body(&mut arrow.body.data_setter);
        }
    }

    pub fn arrow_hit() -> CollisionMatcher {
        CollisionMatcher::fuzzy_match_one(Box::new(|etype| {
            if let EntityType::CrossbowArrow(_x) = etype {
                true
            } else {
                false
            }
        }), Box::new(|entity| {
            let crossbow: &mut Self = entity.resolve();
            println!("Hit!");
            if let EntityType::CrossbowArrow(x) = entity.etype() {
                crossbow.remove_arrow(x);
            }
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
            fixture_def.filter.mask_bits = collision_category::MASK_ALLOW_ALL & !collision_category::PLAYER_BODY;
            body.create_fixture(&fixture_def);
        }

        body
    }

    pub fn draw(&self, box_renderer: &mut BoxRenderer) {
        let boxes: Vec<BoxData> = self.arrows.values().map(|arrow| -> BoxData {
            let body_position = arrow.body.data_setter.get_position();
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
