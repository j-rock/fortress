use crate::{
    app::StatusOr,
    entities::{
        Entity,
        EntityId,
        EntityRegistrar
    },
    file::{
        ConfigWatcher,
        SimpleConfigManager,
    },
    physics::{
        ContactMatcher,
        ProximityMatcher,
    },
    world::WorldView,
};
use ncollide2d::events::ContactEvent;
use nphysics2d::{
    object::ColliderHandle,
    world::World
};
use std::{
    cell::{
        Ref,
        RefCell,
        RefMut,
    },
    rc::Rc,
};

#[derive(Deserialize)]
struct SimulationConfig {
    error_reduction_coefficient: f64,
    warm_start_coefficient: f64,
    restitution_velocity_threshold: f64,
    allowed_linear_error: f64,
    allowed_angular_error: f64,
    max_linear_correction: f64,
    max_angular_correction: f64,
    max_stabilization_multiplier: f64,
    max_velocity_iterations: usize,
    max_position_iterations: usize,
}

#[derive(Clone)]
pub struct PhysicsSimulation {
    raw: Rc<RefCell<RawPhysicsSimulation>>
}

impl PhysicsSimulation {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<PhysicsSimulation> {
        let raw_physics_sim = RawPhysicsSimulation::new(config_watcher)?;
        Ok(PhysicsSimulation {
            raw: Rc::new(RefCell::new(raw_physics_sim))
        })
    }

    pub fn borrow(&self) -> Ref<RawPhysicsSimulation> {
        self.raw.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<RawPhysicsSimulation> {
        self.raw.borrow_mut()
    }
}

pub struct RawPhysicsSimulation {
    config: SimpleConfigManager<SimulationConfig>,
    world: World<f64>,
    registrar: EntityRegistrar,
    proximity_matchers: Vec<ProximityMatcher>,
    contact_matchers: Vec<ContactMatcher>
}

impl RawPhysicsSimulation {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<RawPhysicsSimulation> {
        let config = SimpleConfigManager::<SimulationConfig>::from_config_resource(config_watcher, "physics_simulation.conf")?;

        let world = {
            let config = config.get();
            let mut world = World::new();
            Self::update_world_from_config(config, &mut world);
            world
        };

        Ok(RawPhysicsSimulation {
            config,
            world,
            registrar: EntityRegistrar::new(),
            proximity_matchers: vec!(),
            contact_matchers: vec!()
        })
    }

    pub fn step<'a>(&mut self, world: &mut WorldView<'a>) {
        self.config.update();
        let config = self.config.get();
        Self::update_world_from_config(config, &mut self.world);
        self.world.set_timestep(world.dt().as_f64_seconds());
        self.world.step();
        self.process_contacts(world);
    }

    pub fn add_contact_matchers(&mut self, matchers: Vec<ContactMatcher>) {
        let mut matchers = matchers;
        self.contact_matchers.append(&mut matchers);
    }

    pub fn add_proximity_matchers(&mut self, matchers: Vec<ProximityMatcher>) {
        let mut matchers = matchers;
        self.proximity_matchers.append(&mut matchers);
    }

    pub fn registrar_mut(&mut self) -> &mut EntityRegistrar {
        &mut self.registrar
    }

    pub fn world(&self) -> &World<f64> {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World<f64> {
        &mut self.world
    }

    fn update_world_from_config(config: &SimulationConfig, world: &mut World<f64>) {
        let integration_parameters = world.integration_parameters_mut();
        integration_parameters.erp = config.error_reduction_coefficient;
        integration_parameters.warmstart_coeff = config.warm_start_coefficient;
        integration_parameters.restitution_velocity_threshold = config.restitution_velocity_threshold;
        integration_parameters.allowed_linear_error = config.allowed_linear_error;
        integration_parameters.allowed_angular_error = config.allowed_angular_error;
        integration_parameters.max_linear_correction = config.max_linear_correction;
        integration_parameters.max_angular_correction = config.max_angular_correction;
        integration_parameters.max_stabilization_multiplier = config.max_stabilization_multiplier;
        integration_parameters.max_velocity_iterations = config.max_velocity_iterations;
        integration_parameters.max_position_iterations = config.max_position_iterations;
    }

    fn try_resolve_collider(&self, handle: ColliderHandle) -> Option<Entity> {
        let collider_entity = self.registrar.resolve(EntityId::from_collider_handle(handle));
        if collider_entity.is_some() {
            return collider_entity;
        }

        let body_handle = self.world.collider_body_handle(handle)?;
        return self.registrar.resolve(EntityId::from_body_handle(body_handle));
    }

    fn process_contacts<'a>(&mut self, world: &mut WorldView<'a>) {
        // Resolve all entities first to avoid ABA problem.
        let proximity_events: Vec<_> =
            self.world.proximity_events()
                .iter()
                .filter_map(|proximity| {
                    let entity1 = self.try_resolve_collider(proximity.collider1)?;
                    let entity2 = self. try_resolve_collider(proximity.collider2)?;
                    Some((proximity, entity1, entity2))
                })
                .collect();

        let contact_events: Vec<_> =
            self.world.contact_events()
            .iter()
            .filter_map(|contact| {
                match contact {
                    ContactEvent::Started(handle1, handle2) => {
                        let entity1 = self.try_resolve_collider(*handle1)?;
                        let entity2 = self.try_resolve_collider(*handle2)?;
                        return Some((contact, entity1, entity2));
                    },
                    ContactEvent::Stopped(handle1, handle2) => {
                        let entity1 = self.try_resolve_collider(*handle1)?;
                        let entity2 = self.try_resolve_collider(*handle2)?;
                        return Some((contact, entity1, entity2));
                    }
                }
            })
            .collect();

        // Entities resolved (if possible), now apply updates.
        for (proximity, entity1, entity2) in proximity_events.into_iter() {
            for matcher in self.proximity_matchers.iter() {
                matcher.try_apply(entity1, entity2, proximity, world);
            }
        }

        for (contact, entity1, entity2) in contact_events.into_iter() {
            for matcher in self.contact_matchers.iter() {
                matcher.try_apply(entity1, entity2, contact, world);
            }
        }
    }
}

