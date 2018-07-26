use libc::c_void;
use std::mem;
use std::ptr;
use super::fixture::*;
use super::world::*;
use super::joints::*;
use super::contact::*;
use super::super::collision::shapes::shape::*;
use super::super::common::math::{Vec2, Transform};
use super::super::common::settings::*;

/// The body type.
/// static: zero mass, zero velocity, may be manually moved
/// kinematic: zero mass, non-zero velocity set by user, moved by solver
/// dynamic: positive mass, non-zero velocity determined by forces, moved by solver
#[repr(C)]
#[derive(Debug)]
pub enum BodyType {
	StaticBody = 0,
	KinematicBody,
	DynamicBody
}

/// A body definition holds all the data needed to construct a rigid body.
/// You can safely re-use body definitions. Shapes are added to a body after construction.
#[repr(C)]
#[derive(Debug)]
pub struct BodyDef {

    /// The body type: static, kinematic, or dynamic.
    /// Note: if a dynamic body would have zero mass, the mass is set to one.
    pub body_type: BodyType,

    /// The world position of the body. Avoid creating bodies at the origin
    /// since this can lead to many overlapping shapes.
    pub position: Vec2,

    /// The world angle of the body in radians.
    pub angle: Float32,

    /// The linear velocity of the body's origin in world co-ordinates.
    pub linear_velocity: Vec2,

    /// The angular velocity of the body.
    pub angular_velocity: Float32,

    /// Linear damping is use to reduce the linear velocity. The damping parameter
    /// can be larger than 1.0f but the damping effect becomes sensitive to the
    /// time step when the damping parameter is large.
    pub linear_damping: Float32,


    /// Angular damping is use to reduce the angular velocity. The damping parameter
    /// can be larger than 1.0f but the damping effect becomes sensitive to the
    /// time step when the damping parameter is large.
    pub angular_damping: Float32,

    /// Set this flag to false if this body should never fall asleep. Note that
    /// this increases CPU usage.
    pub allow_sleep: bool,

    /// Is this body initially awake or sleeping?
    pub awake: bool,

    /// Should this body be prevented from rotating? Useful for characters.
    pub fixed_rotation: bool,

    /// Is this a fast moving body that should be prevented from tunneling through
    /// other moving bodies? Note that all bodies are prevented from tunneling through
    /// kinematic and static bodies. This setting is only considered on dynamic bodies.
    /// @warning You should use this flag sparingly since it increases processing time.
    pub bullet: bool,

    /// Does this body start out active?
    pub active: bool,

    /// Use this to store application specific body data.
    pub user_data: *mut c_void,

    /// Scale the gravity applied to this body.
    pub gravity_scale : Float32,
}

impl Default for BodyDef {
	fn default() -> BodyDef {
    	BodyDef {
	        user_data: ptr::null_mut(),
	        position: Vec2::default(),
	        angle: 0.0,
	        linear_velocity: Vec2::default(),
	        angular_velocity: 0.0,
	        linear_damping: 0.0,
	        angular_damping: 0.0,
	        allow_sleep: true,
	        awake: true,
	        fixed_rotation: false,
	        bullet: false,
	        body_type: BodyType::StaticBody,
	        active: true,
	        gravity_scale: 1.0,
	    }
    }
}

pub enum B2Body {}

extern {
	fn b2Body_CreateFixture(this: *const B2Body, def: *mut FixtureDef) -> *mut B2Fixture;
	fn b2Body_CreateFixture_FromShape(this: *const B2Body, shape: *const B2Shape, density: Float32) -> *mut B2Fixture;
	fn b2Body_DestroyFixture(this: *const B2Body, fixture: *mut B2Fixture);
	fn b2Body_SetTransform(this: *const B2Body, position: &Vec2, angle: Float32);
	fn b2Body_GetTransform(this: *const B2Body) -> &Transform;
	fn b2Body_GetPosition(this: *const B2Body) -> &Vec2;
	fn b2Body_GetAngle(this: *const B2Body) -> Float32;
	fn b2Body_GetWorldCenter(this: *const B2Body) -> &Vec2;
	fn b2Body_GetLocalCenter(this: *const B2Body) -> &Vec2;
	fn b2Body_SetLinearVelocity(this: *const B2Body, v: &Vec2);
	fn b2Body_GetLinearVelocity(this: *const B2Body) -> &Vec2;
	fn b2Body_SetAngularVelocity(this: *const B2Body, omega: Float32);
	fn b2Body_GetAngularVelocity(this: *const B2Body) -> Float32;
	fn b2Body_ApplyForce(this: *const B2Body, force: &Vec2, point: &Vec2, wake: bool);
	fn b2Body_ApplyForceToCenter(this: *const B2Body, force: &Vec2, wake: bool);
	fn b2Body_ApplyTorque(this: *const B2Body, torque: Float32, wake: bool);
	fn b2Body_ApplyLinearImpulse(this: *const B2Body, impulse: &Vec2, point: &Vec2, wake: bool);
	fn b2Body_ApplyAngularImpulse(this: *const B2Body, impulse: Float32, wake: bool);
	fn b2Body_GetMass(this: *const B2Body) -> Float32;
	fn b2Body_GetInertia(this: *const B2Body) -> Float32;
	fn b2Body_GetMassData(this: *const B2Body, data: *mut MassData);
	fn b2Body_SetMassData(this: *const B2Body, data: *const MassData);
	fn b2Body_ResetMassData(this: *const B2Body);
	fn b2Body_GetWorldPoint(this: *const B2Body, local_point: &Vec2) -> Vec2;
	fn b2Body_GetWorldVector(this: *const B2Body, localVector: &Vec2) -> Vec2;
	fn b2Body_GetLocalPoint(this: *const B2Body, worldPoint: &Vec2) -> Vec2;
	fn b2Body_GetLocalVector(this: *const B2Body, worldVector: &Vec2) -> Vec2;
	fn b2Body_GetLinearVelocityFromWorldPoint(this: *const B2Body, worldPoint: &Vec2) -> Vec2;
	fn b2Body_GetLinearVelocityFromLocalPoint(this: *const B2Body, local_point: &Vec2) -> Vec2;
	fn b2Body_GetLinearDamping(this: *const B2Body) -> Float32;
	fn b2Body_SetLinearDamping(this: *const B2Body, linearDamping: Float32);
	fn b2Body_GetAngularDamping(this: *const B2Body) -> Float32;
	fn b2Body_SetAngularDamping(this: *const B2Body, angularDamping: Float32);
	fn b2Body_GetGravityScale(this: *const B2Body) -> Float32;
	fn b2Body_SetGravityScale(this: *const B2Body, scale: Float32);
	fn b2Body_SetType(this: *const B2Body, btype: BodyType);
	fn b2Body_GetType(this: *const B2Body) -> BodyType;
	fn b2Body_SetBullet(this: *const B2Body, flag: bool);
	fn b2Body_IsBullet(this: *const B2Body) -> bool;
	fn b2Body_SetSleepingAllowed(this: *const B2Body, flag: bool);
	fn b2Body_IsSleepingAllowed(this: *const B2Body) -> bool;
	fn b2Body_SetAwake(this: *const B2Body, flag: bool);
	fn b2Body_IsAwake(this: *const B2Body) -> bool;
	fn b2Body_SetActive(this: *const B2Body, flag: bool);
	fn b2Body_IsActive(this: *const B2Body) -> bool;
	fn b2Body_SetFixedRotation(this: *const B2Body, flag: bool);
	fn b2Body_IsFixedRotation(this: *const B2Body) -> bool;
	fn b2Body_GetFixtureList(this: *const B2Body) -> *mut B2Fixture;
	fn b2Body_GetJointList(this: *const B2Body) -> *mut JointEdge;
	fn b2Body_GetContactList(this: *const B2Body) -> *mut ContactEdge;
	fn b2Body_GetNext(this: *const B2Body) -> *mut B2Body;
	fn b2Body_GetUserData(this: *const B2Body) -> *mut c_void;
	fn b2Body_SetUserData(this: *const B2Body, data: *mut c_void);
	fn b2Body_GetWorld(this: *const B2Body) -> *mut B2World;
}

/// A rigid body. These are created via b2World::CreateBody.
#[derive(Clone, Debug)]
pub struct Body {
	pub ptr: *mut B2Body
}

impl Body {

    /// Creates a fixture and attach it to this body. Use this function if you need
    /// to set some fixture parameters, like friction. Otherwise you can create the
    /// fixture directly from a shape.
    /// If the density is non-zero, this function automatically updates the mass of the body.
    /// Contacts are not created until the next time step.
    /// @param def the fixture definition.
    /// @warning This function is locked during callbacks.
    pub fn create_fixture(&self, def: &FixtureDef) -> Fixture {
        unsafe {
            Fixture { ptr: b2Body_CreateFixture(self.ptr, mem::transmute(def)) }
        }
    }

    /// Creates a fixture from a shape and attach it to this body.
    /// This is a convenience function. Use FixtureDef if you need to set parameters
    /// like friction, restitution, user data, or filtering.
    /// If the density is non-zero, this function automatically updates the mass of the body.
    /// @param shape the shape to be cloned.
    /// @param density the shape density (set to zero for static bodies).
    /// @warning This function is locked during callbacks.
    pub fn create_fixture_from_shape(&self, shape: &Shape, density: f32) -> Fixture {
        unsafe {
            Fixture { ptr: b2Body_CreateFixture_FromShape(self.ptr, shape.handle(), density) }
        }
    }

	pub fn destroy_fixture(&self, fixture: &mut Fixture) {
		unsafe {
			b2Body_DestroyFixture(self.ptr, fixture.ptr)
		}
	}

	pub fn set_transform(&self, position: &Vec2, angle: Float32) {
		unsafe {
			b2Body_SetTransform(self.ptr, position, angle)
		}
	}

	pub fn get_transform(&self) -> &Transform {
		unsafe {
			b2Body_GetTransform(self.ptr)
		}
	}

    /// Get the world body origin position.
    /// @return the world position of the body's origin.
    pub fn get_position(&self) -> &Vec2 {
        unsafe {
            b2Body_GetPosition(self.ptr)
        }
    }

    /// Get the angle in radians.
    /// @return the current world rotation angle in radians.
    pub fn get_angle(&self) -> f32 {
        unsafe {
            b2Body_GetAngle(self.ptr)
        }
    }

	pub fn get_world_center(&self) -> &Vec2 {
		unsafe {
			b2Body_GetWorldCenter(self.ptr)
		}
	}

	pub fn get_local_center(&self) -> &Vec2 {
		unsafe {
			b2Body_GetLocalCenter(self.ptr)
		}
	}

	pub fn set_linear_velocity(&self, v: &Vec2) {
		unsafe {
			b2Body_SetLinearVelocity(self.ptr, v);
		}
	}

	pub fn get_linear_velocity(&self) -> &Vec2 {
		unsafe {
			b2Body_GetLinearVelocity(self.ptr)
		}
	}

	pub fn set_angular_velocity(&self, omega: f32) {
		unsafe {
			b2Body_SetAngularVelocity(self.ptr, omega);
		}
	}

	pub fn get_angular_velocity(&self) -> f32 {
		unsafe {
			b2Body_GetAngularVelocity(self.ptr)
		}
	}

	pub fn apply_force(&self, force: &Vec2, point: &Vec2, wake: bool) {
		unsafe {
			b2Body_ApplyForce(self.ptr, force, point, wake);
		}
	}

	pub fn apply_force_to_center(&self, force: &Vec2, wake: bool) {
		unsafe {
			b2Body_ApplyForceToCenter(self.ptr, force, wake);
		}
	}

	pub fn apply_torque(&self, torque: f32, wake: bool) {
		unsafe {
			b2Body_ApplyTorque(self.ptr, torque, wake);
		}
	}

	pub fn apply_linear_impulse(&self, impulse: &Vec2, point: &Vec2, wake: bool) {
		unsafe {
			b2Body_ApplyLinearImpulse(self.ptr, impulse, point, wake);
		}
	}

	pub fn apply_angular_impulse(&self, impulse: f32, wake: bool) {
		unsafe {
			b2Body_ApplyAngularImpulse(self.ptr, impulse, wake);
		}
	}

	pub fn get_mass(&self) -> f32 {
		unsafe {
			b2Body_GetMass(self.ptr)
		}
	}

	pub fn get_inertia(&self) -> f32 {
		unsafe {
			b2Body_GetInertia(self.ptr)
		}
	}

	pub fn get_mass_data(&self) -> MassData {
		unsafe {
			let mut mass_data = mem::uninitialized();

			b2Body_GetMassData(self.ptr, &mut mass_data);

			mass_data
		}
	}

	pub fn set_mass_data(&self, data: &MassData) {
		unsafe { b2Body_SetMassData(self.ptr, data) }
	}

	pub fn reset_mass_data(&self) {
		unsafe {
			b2Body_ResetMassData(self.ptr)
		}
	}

	pub fn get_world_point(&self, local_point: &Vec2) -> Vec2 {
		unsafe {
			b2Body_GetWorldPoint(self.ptr, local_point)
		}
	}

	pub fn get_world_vector(&self, local_vector: &Vec2) -> Vec2 {
		unsafe {
			b2Body_GetWorldVector(self.ptr, local_vector)
		}
	}

    pub fn get_local_point(&self, world_point: &Vec2) -> Vec2 {
        unsafe {
            b2Body_GetLocalPoint(self.ptr, world_point)
        }
    }

	pub fn get_local_vector(&self, world_vector: &Vec2) -> Vec2 {
		unsafe {
			b2Body_GetLocalVector(self.ptr, world_vector)
		}
	}

	pub fn get_linear_velocity_from_world_point(&self, world_point: &Vec2) -> Vec2 {
		unsafe {
			b2Body_GetLinearVelocityFromWorldPoint(self.ptr, world_point)
		}
	}

	pub fn get_linear_velocity_from_local_point(&self, local_point: &Vec2) -> Vec2 {
		unsafe {
			b2Body_GetLinearVelocityFromLocalPoint(self.ptr, local_point)
		}
	}

	pub fn get_linear_damping(&self) -> f32 {
		unsafe {
			b2Body_GetLinearDamping(self.ptr)
		}
	}

	pub fn set_linear_damping(&self, linear_damping: f32) {
		unsafe {
			b2Body_SetLinearDamping(self.ptr, linear_damping)
		}
	}
	pub fn get_angular_damping(&self) -> f32 {
		unsafe {
			b2Body_GetAngularDamping(self.ptr)
		}
	}

	pub fn set_angular_damping(&self, angular_damping: f32) {
		unsafe {
			b2Body_SetAngularDamping(self.ptr, angular_damping)
		}
	}

	pub fn get_gravity_scale(&self) -> f32 {
		unsafe {
			b2Body_GetGravityScale(self.ptr)
		}
	}

	pub fn set_gravity_scale(&self, scale: f32) {
		unsafe {
			b2Body_SetGravityScale(self.ptr, scale)
		}
	}

	pub fn set_type(&self, btype: BodyType) {
		unsafe {
			b2Body_SetType(self.ptr, btype)
		}
	}

	pub fn get_type(&self) -> BodyType {
		unsafe {
			b2Body_GetType(self.ptr)
		}
	}

	pub fn set_bullet(&self, flag: bool) {
		unsafe {
			b2Body_SetBullet(self.ptr, flag)
		}
	}

	pub fn is_bullet(&self) -> bool {
		unsafe {
			b2Body_IsBullet(self.ptr)
		}
	}

	pub fn set_sleeping_allowed(&self, flag: bool) {
		unsafe {
			b2Body_SetSleepingAllowed(self.ptr, flag)
		}
	}

	pub fn is_sleeping_allowed(&self) -> bool {
		unsafe {
			b2Body_IsSleepingAllowed(self.ptr)
		}
	}

	pub fn set_awake(&self, flag: bool) {
		unsafe {
			b2Body_SetAwake(self.ptr, flag)
		}
	}

	pub fn is_awake(&self) -> bool {
		unsafe {
			b2Body_IsAwake(self.ptr)
		}
	}

	pub fn set_active(&self, flag: bool) {
		unsafe {
			b2Body_SetActive(self.ptr, flag)
		}
	}

	pub fn is_active(&self) -> bool {
		unsafe {
			b2Body_IsActive(self.ptr)
		}
	}

	pub fn set_fixed_rotation(&self, flag: bool) {
		unsafe {
			b2Body_SetFixedRotation(self.ptr, flag)
		}
	}

	pub fn is_fixed_rotation(&self) -> bool {
		unsafe {
			b2Body_IsFixedRotation(self.ptr)
		}
	}

	/// Get the list of all fixtures attached to this body.
    pub fn get_fixture_list(&self) -> Option<Fixture> {
        let ptr;
        unsafe {
            ptr = b2Body_GetFixtureList(self.ptr);
        }

        if ptr.is_null() {
            None
        } else {
            Some(Fixture { ptr: ptr })
        }
    }

	pub fn get_joint_list(&self) -> &mut JointEdge {
		unsafe { &mut *b2Body_GetJointList(self.ptr) }
	}

	pub fn get_contact_list(&self) -> &mut ContactEdge {
		unsafe { &mut *b2Body_GetContactList(self.ptr) }
	}

    /// Get the next body in the world's body list.
    pub fn get_next(&self) -> Option<Body> {
        let ptr: *mut B2Body;

        unsafe {
            ptr = b2Body_GetNext(self.ptr);
        }

        if ptr.is_null() {
            None
        } else {
            Some(Body { ptr: ptr })
        }
    }

    /// Get the user data pointer that was provided in the body definition.
    pub fn get_user_data<T>(&self) -> Option<&mut T> {
        unsafe {
            let tmp = b2Body_GetUserData(self.ptr) as *mut T;

			if tmp.is_null() {
				None
			}
			else {
				Some(&mut *tmp)
			}
        }
    }

	pub fn set_user_data<T>(&self, data: Option<&mut T>) {
		unsafe {
			b2Body_SetUserData(self.ptr,
				if let Some(data) = data {
					data as *mut T as *mut c_void
				}
				else {
					ptr::null_mut()
				}
			)
		}
	}

    /// Get the parent world of this body.
    pub fn get_world(&self) -> World {
        unsafe {
            World { ptr: b2Body_GetWorld(self.ptr) }
        }
    }
}
