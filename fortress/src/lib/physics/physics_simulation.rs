use crate::{
    app::StatusOr,
    dimensions::time::DeltaTime,
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
        Contact,
        ContactMatcher,
        Proximity,
        ProximityMatcher,
        ProximityType,
    },
    world::WorldView,
};
use nalgebra;
use ncollide2d::narrow_phase::ContactEvent;
use nphysics2d::{
    object::{
        DefaultBodyHandle,
        DefaultBodySet,
        DefaultColliderSet,
        DefaultColliderHandle,
    },
    force_generator::DefaultForceGeneratorSet,
    joint::DefaultJointConstraintSet,
    world::{
        DefaultMechanicalWorld,
        DefaultGeometricalWorld
    },
};
use std::{
    cell::{
        Ref,
        RefCell,
        RefMut,
    },
    collections::HashSet,
    rc::Rc,
};

#[derive(Deserialize)]
struct SimulationConfig {
    force_generator_initial_capacity: usize,
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

    mechanical_world: DefaultMechanicalWorld<f64>,
    geometrical_world: DefaultGeometricalWorld<f64>,
    bodies: DefaultBodySet<f64>,
    colliders: DefaultColliderSet<f64>,
    joint_constraints: DefaultJointConstraintSet<f64>,
    force_generators: DefaultForceGeneratorSet<f64>,

    registrar: EntityRegistrar,
    proximity_matchers: Vec<ProximityMatcher>,
    contact_matchers: Vec<ContactMatcher>
}

impl RawPhysicsSimulation {
    pub fn new(config_watcher: &mut ConfigWatcher) -> StatusOr<RawPhysicsSimulation> {
        let config = SimpleConfigManager::<SimulationConfig>::from_config_resource(config_watcher, "physics_simulation.conf")?;

        let force_generators = {
            let config = config.get();
            DefaultForceGeneratorSet::with_capacity(config.force_generator_initial_capacity)
        };

        Ok(RawPhysicsSimulation {
            config,
            mechanical_world: DefaultMechanicalWorld::new(nalgebra::zero()),
            geometrical_world: DefaultGeometricalWorld::new(),
            bodies: DefaultBodySet::new(),
            colliders: DefaultColliderSet::new(),
            joint_constraints: DefaultJointConstraintSet::new(),
            force_generators,
            registrar: EntityRegistrar::new(),
            proximity_matchers: vec!(),
            contact_matchers: vec!()
        })
    }

    pub fn step(&mut self, dt: DeltaTime) {
        // Currently ignore config updates, since they aren't very useful.
        self.config.update();

        self.mechanical_world.set_timestep(dt.as_f64_seconds());
        self.mechanical_world.step(
            &mut self.geometrical_world,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joint_constraints,
            &mut self.force_generators);
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

    pub fn register(&mut self, entity_id: EntityId, entity: Entity) {
        self.registrar.register(entity_id, entity);
    }

    pub fn drop_collider(&mut self, handle: DefaultColliderHandle) {
        self.registrar.unregister(EntityId::from_collider_handle(handle));
        self.colliders.remove(handle);
    }

    pub fn drop_body(&mut self, handle: DefaultBodyHandle) {
        self.registrar.unregister(EntityId::from_body_handle(handle));
        self.bodies.remove(handle);
    }

    fn try_resolve_collider(&self, handle: DefaultColliderHandle) -> Option<Entity> {
        let collider_entity = self.registrar.resolve(EntityId::from_collider_handle(handle));
        if collider_entity.is_some() {
            return collider_entity;
        }

        let body_handle = self.colliders.get(handle)?.body();
        self.registrar.resolve(EntityId::from_body_handle(body_handle))
    }

    pub fn process_contacts(&self, world: WorldView) {
        // Resolve all entities first to avoid ABA problem.
        let proximity_events: HashSet<_> =
            self.geometrical_world.proximity_events()
                .iter()
                .filter_map(|proximity| {
                    let entity1 = self.try_resolve_collider(proximity.collider1)?;
                    let entity2 = self.try_resolve_collider(proximity.collider2)?;
                    Some(Proximity {
                        entity1,
                        entity2,
                        prev_type: ProximityType::from(proximity.prev_status),
                        curr_type: ProximityType::from(proximity.new_status),
                    })
                })
                .collect();

        let contact_events: HashSet<_> =
            self.geometrical_world.contact_events()
            .iter()
            .filter_map(|contact| {
                match contact {
                    ContactEvent::Started(handle1, handle2) => {
                        let entity1 = self.try_resolve_collider(*handle1)?;
                        let entity2 = self.try_resolve_collider(*handle2)?;
                        Some(Contact::Started(entity1, entity2))
                    },
                    ContactEvent::Stopped(handle1, handle2) => {
                        let entity1 = self.try_resolve_collider(*handle1)?;
                        let entity2 = self.try_resolve_collider(*handle2)?;
                        Some(Contact::Stopped(entity1, entity2))
                    }
                }
            })
            .collect();

        let mut world = world;

        // Entities resolved (if possible), now apply updates.
        for proximity in proximity_events.into_iter() {
            for matcher in self.proximity_matchers.iter() {
                matcher.try_apply(proximity, &mut world);
            }
        }

        for contact in contact_events.into_iter() {
            for matcher in self.contact_matchers.iter() {
                matcher.try_apply(contact, &mut world);
            }
        }
    }
}

