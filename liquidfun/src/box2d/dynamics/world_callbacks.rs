use libc::c_void;
use std::mem;
use super::fixture::*;
use super::contact::*;
use super::super::collision::*;
use super::super::particle::particle_system::*;
use super::super::common::settings::*;
use super::super::common::math::*;

pub type B2RayCastCallbackReportFixture = unsafe extern "C" fn (cObject: *const c_void, fixture: *const B2Fixture, point: &Vec2, normal: &Vec2, fraction: Float32) -> Float32;
pub type B2RayCastCallbackReportParticle = unsafe extern "C" fn (cObject: *const c_void, particleSystem: *const B2ParticleSystem, index: Int32, point: &Vec2, normal: &Vec2, fraction: Float32) -> Float32;
pub type B2RayCastCallbackShouldQueryParticleSystem = unsafe extern "C" fn (cObject: *const c_void, particleSystem: *const B2ParticleSystem) -> bool;

pub enum B2RayCastCallback {}

extern {
    fn b2RayCastCallback_New() -> *mut B2RayCastCallback;
    fn b2RayCastCallback_Delete(this: *const B2RayCastCallback);
    fn b2RayCastCallback_Bind(this: *const B2RayCastCallback,
                            cObject: *const c_void,
                            reportFixture: B2RayCastCallbackReportFixture,
                            reportParticle: B2RayCastCallbackReportParticle,
                            shouldQueryParticleSystem: B2RayCastCallbackShouldQueryParticleSystem
    );
}

pub trait RayCastCallback {
    /// Called for each fixture found in the query. You control how the ray cast
	/// proceeds by returning a float:
	/// return -1: ignore this fixture and continue
	/// return 0: terminate the ray cast
	/// return fraction: clip the ray to this point
	/// return 1: don't clip the ray and continue
	/// @param fixture the fixture hit by the ray
	/// @param point the point of initial intersection
	/// @param normal the normal vector at the point of intersection
	/// @return -1 to filter, 0 to terminate, fraction to clip the ray for
	/// closest hit, 1 to continue
    fn report_fixture(&mut self, fixture: Fixture, point: &Vec2, normal: &Vec2, fraction: Float32) -> Float32;

    /// Called for each particle found in the query. You control how the ray
    /// cast proceeds by returning a float:
    /// return <=0: ignore the remaining particles in this particle system
    /// return fraction: ignore particles that are 'fraction' percent farther
    ///   along the line from 'point1' to 'point2'. Note that 'point1' and
    ///   'point2' are parameters to b2World::RayCast.
    /// @param particleSystem the particle system containing the particle
    /// @param index the index of the particle in particleSystem
    /// @param point the point of intersection bt the ray and the particle
    /// @param normal the normal vector at the point of intersection
    /// @param fraction percent (0.0~1.0) from 'point0' to 'point1' along the
    ///   ray. Note that 'point1' and 'point2' are parameters to
    ///   b2World::RayCast.
    /// @return <=0 to ignore rest of particle system, fraction to ignore
    /// particles that are farther away.
    fn report_particle(&mut self, particle_system: ParticleSystem, index: Int32, point: &Vec2, normal: &Vec2, fraction: Float32) -> Float32;

    /// Cull an entire particle system from b2World::RayCast. Ignored in
	/// b2ParticleSystem::RayCast.
	/// @return true if you want to include particleSystem in the RayCast, or
	/// false to cull particleSystem from the RayCast.
	fn should_query_particle_system(&mut self, particle_system: ParticleSystem) -> bool;
}

impl<F> RayCastCallback for F
    where F: FnMut(Fixture, &Vec2, &Vec2, Float32) -> Float32
{
    fn report_fixture(&mut self, fixture: Fixture, point: &Vec2, normal: &Vec2, fraction: Float32) -> Float32 {
        self(fixture, point, normal, fraction)
    }

    fn report_particle(&mut self, particle_system: ParticleSystem, index: Int32, point: &Vec2, normal: &Vec2, fraction: Float32) -> Float32 {
        default_report_particle(particle_system.ptr, index, point, normal, fraction)
    }

    fn should_query_particle_system(&mut self, particle_system: ParticleSystem) -> bool {
        default_should_query_particle_system(particle_system.ptr)
    }
}

impl<RFF, RPF, SQF> RayCastCallback for (RFF, RPF, SQF)
    where RFF: FnMut(Fixture, &Vec2, &Vec2, Float32) -> Float32,
          RPF: FnMut(ParticleSystem, Int32, &Vec2, &Vec2, Float32) -> Float32,
          SQF: FnMut(ParticleSystem) -> bool
{
    fn report_fixture(&mut self, fixture: Fixture, point: &Vec2, normal: &Vec2, fraction: Float32) -> Float32 {
        self.0(fixture, point, normal, fraction)
    }

    fn report_particle(&mut self, particle_system: ParticleSystem, index: Int32, point: &Vec2, normal: &Vec2, fraction: Float32) -> Float32 {
        self.1(particle_system, index, point, normal, fraction)
    }

    fn should_query_particle_system(&mut self, particle_system: ParticleSystem) -> bool {
        self.2(particle_system)
    }
}

pub struct RayCastCallbackGlue {
	pub ptr: *mut B2RayCastCallback,
}

impl RayCastCallbackGlue {
    pub fn new() -> Self {
        unsafe { RayCastCallbackGlue { ptr: b2RayCastCallback_New(), } }
    }

    pub fn use_with<C: RayCastCallback>(&mut self, callback: &mut C) {
        unsafe {
            b2RayCastCallback_Bind(self.ptr,
                callback as *mut _ as *mut _,
                ray_cast_callback_report_fixture::<C>,
                ray_cast_callback_report_particle::<C>,
                ray_cast_callback_should_query_particle_system::<C>
            );
        }
    }
}

impl Drop for RayCastCallbackGlue {
    fn drop(&mut self) {
        unsafe { b2RayCastCallback_Delete(self.ptr) }
    }
}

fn default_report_particle(_particle_system: *const B2ParticleSystem, _index: Int32, _point: &Vec2, _normal: &Vec2, _fraction: Float32) -> Float32
{
    return 0.0;
}

fn default_should_query_particle_system(_particle_system: *const B2ParticleSystem) -> bool {
    return false;
}

unsafe extern "C" fn ray_cast_callback_report_fixture<C: RayCastCallback>(c_object: *const c_void, fixture: *const B2Fixture, point: &Vec2, normal: &Vec2, fraction: Float32) -> Float32 {
    let callback = mem::transmute::<_, &mut C>(c_object);
    let fixture = Fixture { ptr: fixture as *mut _ };

    callback.report_fixture(fixture, point, normal, fraction)
}

#[allow(non_snake_case)]
unsafe extern "C" fn ray_cast_callback_report_particle<C: RayCastCallback>(c_object: *const c_void, particle_system: *const B2ParticleSystem, index: Int32, point: &Vec2, normal: &Vec2, fraction: Float32) -> Float32 {
    let callback = mem::transmute::<_, &mut C>(c_object);
    let particle_system = ParticleSystem { ptr: particle_system as *mut _ };

    callback.report_particle(particle_system, index, point, normal, fraction)
}

#[allow(non_snake_case)]
unsafe extern "C" fn ray_cast_callback_should_query_particle_system<C: RayCastCallback>(c_object: *const c_void, particle_system: *const B2ParticleSystem) -> bool {
    let callback = mem::transmute::<_, &mut C>(c_object);
    let particle_system = ParticleSystem { ptr: particle_system as *mut _ };

    callback.should_query_particle_system(particle_system)
}

/// Contact impulses for reporting. Impulses are used instead of forces because
/// sub-step forces may approach infinity for rigid body collisions. These
/// match up one-to-one with the contact points in b2Manifold.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ContactImpulse
{
	pub normal_impulses: [Float32; MAX_MANIFOLD_POINTS],
	pub tangent_impulses: [Float32; MAX_MANIFOLD_POINTS],
	pub count: Int32,
}

pub type B2ContactListenerBeginFixtureFixture = unsafe extern "C" fn (cObject: *const c_void, contact: *const B2Contact);
pub type B2ContactListenerEndFixtureFixture = unsafe extern "C" fn (cObject: *const c_void, contact: *const B2Contact);
pub type B2ContactListenerBeginParticleFixture = unsafe extern "C" fn (cObject: *const c_void, particleSystem: *const B2ParticleSystem, particleBodyContact: *const ParticleBodyContact);
pub type B2ContactListenerEndParticleFixture = unsafe extern "C" fn (cObject: *const c_void, fixture: *const B2Fixture, particleSystem: *const B2ParticleSystem, index: Int32);
pub type B2ContactListenerBeginParticleParticle = unsafe extern "C" fn (cObject: *const c_void, particleSystem: *const B2ParticleSystem, particleContact: *const B2ParticleContact);
pub type B2ContactListenerEndParticleParticle = unsafe extern "C" fn (cObject: *const c_void, particleSystem: *const B2ParticleSystem, indexA: Int32, indexB: Int32);
pub type B2ContactListenerPreSolve = unsafe extern "C" fn (cObject: *const c_void, contact: *const B2Contact, oldManifold: *const Manifold);
pub type B2ContactListenerPostSolve = unsafe extern "C" fn (cObject: *const c_void, contact: *const B2Contact, impulse: *const ContactImpulse);

pub enum B2ContactListener {}

extern {
    fn b2ContactListener_New() -> *mut B2ContactListener;
    fn b2ContactListener_Delete(this: *const B2ContactListener);
    fn b2ContactListener_Bind(this: *const B2ContactListener,
    							cObject: *const c_void,
    							beginFixtureFixture: B2ContactListenerBeginFixtureFixture,
    							endFixtureFixture: B2ContactListenerEndFixtureFixture,
    							beginParticleFixture: B2ContactListenerBeginParticleFixture,
    							endParticleFixture: B2ContactListenerEndParticleFixture,
    							beginParticleParticle: B2ContactListenerBeginParticleParticle,
    							endParticleParticle: B2ContactListenerEndParticleParticle,
    							preSolve: B2ContactListenerPreSolve,
    							postSolve: B2ContactListenerPostSolve
    );
}

pub trait ContactListener {
    /// Called when two fixtures begin to touch.
    fn begin_fixture_fixture(&mut self, contact: Contact);

    /// Called when two fixtures cease to touch.
    fn end_fixture_fixture(&mut self, contact: Contact);

    /// Called when a fixture and particle start touching if the
    /// b2_fixtureContactFilterParticle flag is set on the particle.
    fn begin_particle_fixture(&mut self, particle_system: ParticleSystem, particle_body_contact: &ParticleBodyContact);

    /// Called when a fixture and particle stop touching if the
	/// b2_fixtureContactFilterParticle flag is set on the particle.
    fn end_particle_fixture(&mut self, fixture: Fixture, particle_system: ParticleSystem, index: Int32);

    /// Called when two particles start touching if
	/// b2_particleContactFilterParticle flag is set on either particle.
    fn begin_particle_particle(&mut self, particle_system: ParticleSystem, particle_contact: ParticleContact);

    /// Called when two particles start touching if
	/// b2_particleContactFilterParticle flag is set on either particle.
    fn end_particle_particle(&mut self, particle_system: ParticleSystem, index_a: Int32, index_b: Int32);

    /// This is called after a contact is updated. This allows you to inspect a
	/// contact before it goes to the solver. If you are careful, you can modify the
	/// contact manifold (&mut self, e.g. disable contact).
	/// A copy of the old manifold is provided so that you can detect changes.
	/// Note: this is called only for awake bodies.
	/// Note: this is called even when the number of contact points is zero.
	/// Note: this is not called for sensors.
	/// Note: if you set the number of contact points to zero, you will not
	/// get an EndContact callback. However, you may get a BeginContact callback
	/// the next step.
    fn pre_solve(&mut self, contact: Contact, old_manifold: &Manifold);

    /// This lets you inspect a contact after the solver is finished. This is useful
	/// for inspecting impulses.
	/// Note: the contact manifold does not include time of impact impulses, which can be
	/// arbitrarily large if the sub-step is small. Hence the impulse is provided explicitly
	/// in a separate data structure.
	/// Note: this is only called for contacts that are touching, solid, and awake.
    fn post_solve(&mut self, contact: Contact, impulse: &ContactImpulse);
}

#[derive(Clone, Debug)]
pub struct ContactListenerGlue {
    pub ptr: *mut B2ContactListener,
}

impl ContactListenerGlue {
    pub fn new() -> Self {
        unsafe { ContactListenerGlue { ptr: b2ContactListener_New(), } }
    }

    pub fn use_with<C: ContactListener>(&mut self, callback: &mut C) {
        unsafe {
            b2ContactListener_Bind(self.ptr,
                callback as *mut _ as *mut _,
                contact_listener_begin_fixture_fixture::<C>,
                contact_listener_end_fixture_fixture::<C>,
                contact_listener_begin_particle_fixture::<C>,
                contact_listener_end_particle_fixture::<C>,
                contact_listener_begin_particle_particle::<C>,
                contact_listener_end_particle_particle::<C>,
                contact_listener_pre_solve::<C>,
                contact_listener_post_solve::<C>
            );
        }
    }
}

impl Drop for ContactListenerGlue {
    fn drop(&mut self) {
        unsafe { b2ContactListener_Delete(self.ptr) }
    }
}

unsafe extern "C" fn contact_listener_begin_fixture_fixture<C: ContactListener>(c_object: *const c_void, contact: *const B2Contact) {
    let callback = mem::transmute::<_, &mut C>(c_object);
    let contact = Contact { ptr: contact as *mut _ };

    callback.begin_fixture_fixture(contact);
}

unsafe extern "C" fn contact_listener_end_fixture_fixture<C: ContactListener>(c_object: *const c_void, contact: *const B2Contact) {
    let callback = mem::transmute::<_, &mut C>(c_object);
    let contact = Contact { ptr: contact as *mut _ };

    callback.end_fixture_fixture(contact);
}

unsafe extern "C" fn contact_listener_begin_particle_fixture<C: ContactListener>(c_object: *const c_void, particle_system: *const B2ParticleSystem, particle_body_contact: *const ParticleBodyContact) {
    let callback = mem::transmute::<_, &mut C>(c_object);
    let particle_system = ParticleSystem { ptr: particle_system as *mut _ };

    callback.begin_particle_fixture(particle_system, &*particle_body_contact);
}

unsafe extern "C" fn contact_listener_end_particle_fixture<C: ContactListener>(c_object: *const c_void, fixture: *const B2Fixture, particle_system: *const B2ParticleSystem, index: Int32) {
    let callback = mem::transmute::<_, &mut C>(c_object);
    let fixture = Fixture { ptr: fixture as *mut _ };
    let particle_system = ParticleSystem { ptr: particle_system as *mut _ };

    callback.end_particle_fixture(fixture, particle_system, index);
}

unsafe extern "C" fn contact_listener_begin_particle_particle<C: ContactListener>(c_object: *const c_void, particle_system: *const B2ParticleSystem, particle_contact: *const B2ParticleContact) {
    let callback = mem::transmute::<_, &mut C>(c_object);
    let particle_system = ParticleSystem { ptr: particle_system as *mut _ };
    let particle_contact = ParticleContact { ptr: particle_contact as *mut _ };

    callback.begin_particle_particle(particle_system, particle_contact);
}

unsafe extern "C" fn contact_listener_end_particle_particle<C: ContactListener>(c_object: *const c_void, particle_system: *const B2ParticleSystem, index_a: Int32, index_b: Int32) {
    let callback = mem::transmute::<_, &mut C>(c_object);
    let particle_system = ParticleSystem { ptr: particle_system as *mut _ };

    callback.end_particle_particle(particle_system, index_a, index_b);
}

unsafe extern "C" fn contact_listener_pre_solve<C: ContactListener>(c_object: *const c_void, contact: *const B2Contact, old_manifold: *const Manifold) {
    let callback = mem::transmute::<_, &mut C>(c_object);
    let contact = Contact { ptr: contact as *mut _ };

    callback.pre_solve(contact, &*old_manifold);
}

unsafe extern "C" fn contact_listener_post_solve<C: ContactListener>(c_object: *const c_void, contact: *const B2Contact, impulse: *const ContactImpulse) {
    let callback = mem::transmute::<_, &mut C>(c_object);
    let contact = Contact { ptr: contact as *mut _ };

    callback.post_solve(contact, &*impulse);
}
