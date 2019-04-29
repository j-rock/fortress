use crate::{
    entities::{
        Entity,
        RegisteredBody
    },
    physics::{
        collision_category,
        PhysicsSimulation,
    },
    render::{
        FullyIlluminatedSpriteRenderer,
        LightDependentSpriteRenderer,
        PointLight,
    },
    treasures::{
        Treasure,
        TreasureChestId,
        TreasureConfig,
    },
    world::RandGen,
};
use nalgebra::{
    Point2,
    Vector2,
};
use ncollide2d::{
    shape::{
        Cuboid,
        ShapeHandle,
    },
    world::CollisionGroups
};
use nphysics2d::object::{
    BodyStatus,
    ColliderDesc,
    RigidBodyDesc,
};

pub struct TreasureChest {
    _body: RegisteredBody,
    _treasure: Option<Treasure>,
}

impl TreasureChest {
    pub fn new(config: &TreasureConfig, chest_id: TreasureChestId, location: Point2<f64>, rng: &mut RandGen, physics_sim: &mut PhysicsSimulation) -> TreasureChest {
        let box_shape = Cuboid::new(Vector2::new(config.physical_widths.0 / 2.0, config.physical_widths.1 / 2.0));
        let collider_desc = ColliderDesc::new(ShapeHandle::new(box_shape))
            .collision_groups(CollisionGroups::new()
                .with_membership(&[collision_category::INTERACT])
                .with_whitelist(&[collision_category::BARRIER, collision_category::PLAYER_WEAPON]));

        let mut rigid_body_desc = RigidBodyDesc::new()
            .status(BodyStatus::Static)
            .translation(location.coords)
            .collider(&collider_desc)
            .kinematic_rotation(true);

        let body_handle  = rigid_body_desc
            .build(physics_sim.borrow_mut().world_mut())
            .handle();
        let entity = Entity::TreasureChest(chest_id);
        let body = RegisteredBody::new(body_handle, entity, physics_sim);

        TreasureChest {
            _body: body,
            _treasure: Some(Treasure::new(rng)),
        }
    }

    pub fn post_update(&mut self, _config: &TreasureConfig, _physics_sim: &mut PhysicsSimulation) {
    }

    pub fn populate_lights(&self, _lights: &mut Vec<PointLight>) {
    }

    pub fn queue_draw(&self, _config: &TreasureConfig, _full_light_sprite: &mut FullyIlluminatedSpriteRenderer, _light_dependent_sprite: &mut LightDependentSpriteRenderer) {
    }
}