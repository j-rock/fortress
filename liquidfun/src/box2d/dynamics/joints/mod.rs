use libc::c_void;
use std::ptr;
use super::world::World;
use super::super::dynamics::body::{B2Body, Body};
use super::super::common::math::Vec2;
use super::super::common::settings::Float32;

pub mod distance_joint;
pub mod friction_joint;
pub mod gear_joint;
pub mod motor_joint;
pub mod mouse_joint;
pub mod prismatic_joint;
pub mod pulley_joint;
pub mod revolute_joint;
pub mod rope_joint;
pub mod weld_joint;
pub mod wheel_joint;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum JointType {
	UnknownJoint = 0,
	RevoluteJoint,
	PrismaticJoint,
	DistanceJoint,
	PulleyJoint,
	MouseJoint,
	GearJoint,
	WheelJoint,
    WeldJoint,
	FrictionJoint,
	RopeJoint,
	MotorJoint,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum LimitState
{
	InactiveLimit = 0,
	AtLowerLimit,
	AtUpperLimit,
	EqualLimits
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Jacobian
{
	pub linear: Vec2,
	pub angular_a: Float32,
	pub angular_b: Float32,
}

/// A joint edge is used to connect bodies and joints together
/// in a joint graph where each body is a node and each joint
/// is an edge. A joint edge belongs to a doubly linked list
/// maintained in each attached body. Each joint has two joint
/// nodes, one for each attached body.
#[derive(Debug)]
#[repr(C)]
pub struct JointEdge
{
	/// provides quick access to the other body attached.
	other: *mut B2Body,
	/// the joint
	joint: *mut B2Joint,
	/// the previous joint edge in the body's joint list
	prev: *mut JointEdge,
	/// the next joint edge in the body's joint list
	next: *mut JointEdge,
}

impl JointEdge {
    pub fn other(&self) -> Body {
        Body { ptr: self.other }
    }

	// TODO: do something other than this...
    pub fn joint(&self) -> *mut B2Joint {
		self.joint
    }

    pub fn prev(&self) -> Option<&Self> {
        if self.prev.is_null() {
            None
        } else {
            Some(unsafe { &*self.prev })
        }
    }

    pub fn next(&self) -> Option<&Self> {
        if self.next.is_null() {
            None
        } else {
            Some(unsafe { &*self.next })
        }
    }
}

pub trait JointDef<T>: Default {
	fn joint_type() -> JointType;

	fn create(&self, &mut World) -> T;
}

pub enum B2Joint {}

extern {
	fn b2Joint_GetType(this: *const B2Joint) -> JointType;
    fn b2Joint_GetBodyA(this: *const B2Joint) -> *mut B2Body;
    fn b2Joint_GetBodyB(this: *const B2Joint) -> *mut B2Body;
    fn b2Joint_GetAnchorA(this: *const B2Joint) -> Vec2;
    fn b2Joint_GetAnchorB(this: *const B2Joint) -> Vec2;
    fn b2Joint_GetReactionForce(this: *const B2Joint, inv_dt: Float32) -> Vec2;
    fn b2Joint_GetReactionTorque(this: *const B2Joint, inv_dt: Float32) -> Float32;
    fn b2Joint_GetNext(this: *const B2Joint) -> *mut B2Joint;
    fn b2Joint_GetUserData(this: *const B2Joint) -> *mut c_void;
    fn b2Joint_SetUserData(this: *const B2Joint, data: *mut c_void);
    fn b2Joint_IsActive(this: *const B2Joint) -> bool;
    fn b2Joint_GetCollideConnected(this: *const B2Joint) -> bool;
    fn b2Joint_ShiftOrigin(this: *const B2Joint, newOrigin: &Vec2);
}

pub trait Joint {
	fn get_handle(&self) -> *mut B2Joint;

	/// Get the type of the concrete joint.
	fn get_type(&self) -> JointType {
		unsafe { b2Joint_GetType(self.get_handle()) }
	}

	/// Get the first body attached to this joint.
	fn get_body_a(&self) -> Body {
		unsafe { Body { ptr: b2Joint_GetBodyA(self.get_handle()) } }
	}

	/// Get the second body attached to this joint.
	fn get_body_b(&self) -> Body {
		unsafe { Body { ptr: b2Joint_GetBodyB(self.get_handle()) } }
	}

	/// Get the anchor point on bodyA in world coordinates.
	fn get_anchor_a(&self) -> Vec2 {
		unsafe { b2Joint_GetAnchorA(self.get_handle()) }
	}

	/// Get the anchor point on bodyB in world coordinates.
	fn get_anchor_b(&self) -> Vec2 {
		unsafe { b2Joint_GetAnchorB(self.get_handle()) }
	}

	/// Get the reaction force on bodyB at the joint anchor in Newtons.
	fn get_reaction_force(&self, inv_dt: Float32) -> Vec2 {
		unsafe { b2Joint_GetReactionForce(self.get_handle(), inv_dt) }
	}

	/// Get the reaction torque on bodyB in N*m.
	fn get_reaction_torque(&self, inv_dt: Float32) -> Float32 {
		unsafe { b2Joint_GetReactionTorque(self.get_handle(), inv_dt) }
	}

	/// Get the next joint the world joint list.
	fn get_next(&self) -> Self;

	fn get_user_data<T>(&self) -> Option<&mut T> {
        unsafe {
            let tmp = b2Joint_GetUserData(self.get_handle()) as *mut T;

			if tmp.is_null() {
				None
			}
			else {
				Some(&mut *tmp)
			}
        }
    }

	fn set_user_data<T>(&self, data: Option<&mut T>) {
		unsafe {
			b2Joint_SetUserData(self.get_handle(),
				if let Some(data) = data {
					data as *mut T as *mut c_void
				}
				else {
					ptr::null_mut()
				}
			)
		}
	}

	/// Short-cut function to determine if either body is inactive.
	fn is_active(&self) -> bool {
		unsafe { b2Joint_IsActive(self.get_handle()) }
	}

	/// Get collide connected.
	/// Note: modifying the collide connect flag won't work correctly because
	/// the flag is only checked when fixture AABBs begin to overlap.
	fn get_collide_connected(&self) -> bool {
		unsafe { b2Joint_GetCollideConnected(self.get_handle()) }
	}

	/// Shift the origin for any points stored in world coordinates.
	fn shift_origin(&self, new_origin: &Vec2) {
		unsafe { b2Joint_ShiftOrigin(self.get_handle(), new_origin) }
	}
}
