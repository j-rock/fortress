use super::body::*;
use super::world_callbacks::*;
use super::super::common::math::*;
use super::super::common::settings::*;
use super::super::particle::particle_system::*;
use super::joints;

pub enum B2World {}

extern {
    fn b2World_New(gravity: *const Vec2) -> *mut B2World;
    fn b2World_Delete(world: *mut B2World);
    fn b2World_SetContactListener(this: *mut B2World, listener: *mut B2ContactListener);
    fn b2World_CreateBody(world: *mut B2World, bd: *const BodyDef) -> *mut B2Body;
    fn b2World_DestroyBody(world: *mut B2World, body: *mut B2Body);
    fn b2World_CreateParticleSystem(world: *mut B2World, def: *const ParticleSystemDef) -> *mut B2ParticleSystem;
    fn b2World_GetBodyCount(world: *const B2World) -> Int32;
    fn b2World_GetJointCount(world: *const B2World) -> Int32;
    fn b2World_GetBodyList(world: *const B2World) -> *mut B2Body;
    fn b2World_GetParticleSystemList(world: *const B2World) -> *mut B2ParticleSystem;
    fn b2World_StepParticle(this: *mut B2World, timeStep: Float32, velocityIterations: Int32, positionIterations: Int32, particleIterations: Int32);
    fn b2World_Step(this: *mut B2World, timeStep: Float32, velocityIterations: Int32, positionIterations: Int32);
    fn b2World_CalculateReasonableParticleIterations(this: *const B2World, timeStep: Float32) -> Int32;
    fn b2World_RayCast(this: *const B2World, callback: *const B2RayCastCallback, point1: &Vec2, point2: &Vec2);
    fn b2World_SetGravity(world: *mut B2World, gravity: *const Vec2);
    fn b2World_GetGravity(world: *mut B2World) -> Vec2;
}

/// The world class manages all physics entities, dynamic simulation,
/// and asynchronous queries. The world also contains efficient memory
/// management facilities.
pub struct World {
	pub ptr: *mut B2World,
}

impl World {

    /// Construct a world object.
    /// @param gravity the world gravity vector.
    pub fn new(gravity: &Vec2) -> World {
        unsafe {
            World { ptr: b2World_New(gravity) }
        }
    }

    pub fn set_contact_listener<C: ContactListener>(&mut self, listener: &mut C, glue: &mut ContactListenerGlue) {
        glue.use_with(listener);
        unsafe { b2World_SetContactListener(self.ptr, glue.ptr) }
    }

    /// Create a rigid body given a definition. No reference to the definition
    /// is retained.
    /// @warning This function is locked during callbacks.
    pub fn create_body(&mut self, def: &BodyDef) -> Body {
        unsafe {
            Body { ptr: b2World_CreateBody(self.ptr, def) }
        }
    }

    pub fn destroy_body(&mut self, body: &mut Body) {
        unsafe {
            b2World_DestroyBody(self.ptr, body.ptr);
        }
    }

    /// Create a joint to constrain bodies together. No reference to the definition
	/// is retained. This may cause the connected bodies to cease colliding.
	/// @warning This function is locked during callbacks.
    pub fn create_joint<J, JD>(&mut self, jd: &JD) -> J
        where JD: joints::JointDef<J>
    {
        jd.create(self)
    }

    /// Create a particle system given a definition. No reference to the
    /// definition is retained.
    /// @warning This function is locked during callbacks.
    pub fn create_particle_system(&self, def: &ParticleSystemDef) -> ParticleSystem {
        unsafe {
            ParticleSystem { ptr: b2World_CreateParticleSystem(self.ptr, def) }
        }
    }

    /// Get the number of bodies.
    pub fn get_body_count(&self) -> i32 {
        unsafe {
            b2World_GetBodyCount(self.ptr)
        }
    }

    /// Get the number of joints.
    pub fn get_joint_count(&self) -> i32 {
        unsafe {
            b2World_GetJointCount(self.ptr)
        }
    }

    /// Get the world body list. With the returned body, use b2Body::GetNext to get
    /// the next body in the world list. A NULL body indicates the end of the list.
    /// @return the head of the world body list.
    pub fn get_body_list(&self) -> Option<Body> {
        let ptr;
        unsafe {
            ptr = b2World_GetBodyList(self.ptr);
        }

        if ptr.is_null() {
            None
        } else {
            Some(Body { ptr: ptr })
        }
    }

    /// Get the world particle-system list. With the returned body, use
    /// b2ParticleSystem::GetNext to get the next particle-system in the world
    /// list. A NULL particle-system indicates the end of the list.
    /// @return the head of the world particle-system list.
    pub fn get_particle_system_list(&self) -> Option<ParticleSystem> {
        let ptr;
        unsafe {
            ptr = b2World_GetParticleSystemList(self.ptr);
        }

        if ptr.is_null() {
            None
        } else {
            Some(ParticleSystem { ptr: ptr })
        }
    }

    /// Take a time step. This performs collision detection, integration,
	/// and constraint solution.
	/// For the numerical stability of particles, minimize the following
	/// dimensionless gravity acceleration:
	///     gravity / particleRadius * (timeStep / particleIterations)^2
	/// b2CalculateParticleIterations() or
	/// CalculateReasonableParticleIterations() help to determine the optimal
	/// particleIterations.
	/// @param timeStep the amount of time to simulate, this should not vary.
	/// @param velocityIterations for the velocity constraint solver.
	/// @param positionIterations for the position constraint solver.
	/// @param particleIterations for the particle simulation.
    pub fn step_particle(&mut self, time_step: Float32, velocity_iterations: Int32, position_iterations: Int32, particle_iterations: Int32) {
        unsafe { b2World_StepParticle(self.ptr, time_step, velocity_iterations, position_iterations, particle_iterations) }
    }

    /// Take a time step. This performs collision detection, integration,
    /// and constraint solution.
    /// @param timeStep the amount of time to simulate, this should not vary.
    /// @param velocityIterations for the velocity constraint solver.
    /// @param positionIterations for the position constraint solver.
    pub fn step(&mut self, time_step: f32, velocity_iterations: i32, position_iterations: i32) {
        unsafe {
            b2World_Step(self.ptr, time_step, velocity_iterations, position_iterations);
        }
    }

    /// Recommend a value to be used in `Step` for `particleIterations`.
	/// This calculation is necessarily a simplification and should only be
	/// used as a starting point. Please see "Particle Iterations" in the
	/// Programmer's Guide for details.
	/// @param timeStep is the value to be passed into `Step`.
    pub fn calculate_reasonable_particle_iterations(&self, time_step: Float32) -> Int32 {
        unsafe { b2World_CalculateReasonableParticleIterations(self.ptr, time_step) }
    }

    /// Ray-cast the world for all fixtures in the path of the ray. Your callback
    /// controls whether you get the closest point, any point, or n-points.
    /// The ray-cast ignores shapes that contain the starting point.
    /// @param callback a user implemented callback class.
    /// @param point1 the ray starting point
    /// @param point2 the ray ending point
    pub fn ray_cast<C: RayCastCallback>(&self, callback: &mut C, point1: &Vec2, point2: &Vec2)
    {
        let mut glue = RayCastCallbackGlue::new();
        glue.use_with(callback);

        unsafe { b2World_RayCast(self.ptr, glue.ptr, point1, point2) }
    }

    /// Change the global gravity vector.
    pub fn set_gravity(&mut self, gravity: &Vec2) {
    	unsafe { b2World_SetGravity(self.ptr, gravity) }
    }

    /// Get the global gravity vector.
    pub fn get_gravity(&mut self) -> Vec2 {
    	unsafe { b2World_GetGravity(self.ptr) }
    }

}

// Wrapper around World that cleans up on Drop.
pub struct WrappedWorld {
    pub world: World
}

impl WrappedWorld {
    pub fn new(gravity: &Vec2) -> WrappedWorld {
        unsafe {
            WrappedWorld {
                world: World { ptr: b2World_New(gravity) }
            }
        }
    }
}

impl Drop for WrappedWorld {
    fn drop(&mut self) {
        unsafe {
            b2World_Delete(self.world.ptr);
        }
    }
}

