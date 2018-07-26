use libc::c_void;
use std::ptr;

use super::super::super::dynamics::world::{B2World, World};
use super::super::super::common::math::*;
use super::super::super::common::settings::*;
use super::super::super::dynamics::body::{B2Body, Body};
use super::{JointType, JointDef, Joint, B2Joint, b2Joint_GetNext, LimitState};

/// Revolute joint definition. This requires defining an
/// anchor point where the bodies are joined. The definition
/// uses local anchor points so that the initial configuration
/// can violate the constraint slightly. You also need to
/// specify the initial relative angle for joint limits. This
/// helps when saving and loading a game.
/// The local anchor points are measured from the body's origin
/// rather than the center of mass because:
/// 1. you might not know where the center of mass will be.
/// 2. if you add/remove shapes from a body and recompute the mass,
///    the joints will be broken.
#[derive(Debug)]
pub struct RopeJointDef {

	/// Use this to attach application specific data to your joints.
	pub user_data: *mut c_void,

	/// The first attached body.
	pub body_a: Option<Body>,

	/// The second attached body.
	pub body_b: Option<Body>,

	/// Set this flag to true if the attached bodies should collide.
	pub collide_connected: bool,

	/// The local anchor point relative to bodyA's origin.
	pub local_anchor_a: Vec2,

	/// The local anchor point relative to bodyB's origin.
	pub local_anchor_b: Vec2,

	/// The maximum length of the rope.
	/// Warning: this must be larger than b2_linearSlop or
	/// the joint will have no effect.
	pub max_length: Float32,
}

impl Default for RopeJointDef {
	fn default() -> Self {
		RopeJointDef {
			user_data: ptr::null_mut(),
			body_a: None,
			body_b: None,
			collide_connected: false,
			local_anchor_a: Vec2::zero(),
			local_anchor_b: Vec2::zero(),
			max_length: 0.0,
	    }
	}
}

impl JointDef<RopeJoint> for RopeJointDef {
	fn joint_type() -> JointType { JointType::RopeJoint }

	fn create(&self, world: &mut World) -> RopeJoint {
		unsafe { RopeJoint { ptr: b2RopeJoint_Create(
			world.ptr,
			self.user_data,
			if let Some(ref p) = self.body_a { p.ptr } else { ptr::null_mut() },
		    if let Some(ref p) = self.body_b { p.ptr } else { ptr::null_mut() },
		    self.collide_connected,
			self.local_anchor_a,
			self.local_anchor_b,
			self.max_length,
		) } }
	}
}

pub enum B2RopeJoint {}

extern {
    fn b2RopeJoint_Create(
		world: *mut B2World,
		userData: *mut c_void,
		bodyA: *mut B2Body,
	    bodyB: *mut B2Body,
	    collideConnected: bool,
		localAnchorA: Vec2,
		localAnchorB: Vec2,
		maxLength: Float32
	) -> *mut B2RopeJoint;

	fn b2RopeJoint_GetLocalAnchorA(this: *const B2RopeJoint) -> &Vec2;
	fn b2RopeJoint_GetLocalAnchorB(this: *const B2RopeJoint) -> &Vec2;
	fn b2RopeJoint_SetMaxLength(this: *const B2RopeJoint, length: Float32);
	fn b2RopeJoint_GetMaxLength(this: *const B2RopeJoint) -> Float32;
	fn b2RopeJoint_GetLimitState(this: *const B2RopeJoint) -> LimitState;
}

#[derive(Clone, Debug)]
pub struct RopeJoint {
	pub ptr: *mut B2RopeJoint
}

impl RopeJoint {
	pub fn get_local_anchor_a(&self) -> &Vec2 {
		unsafe { b2RopeJoint_GetLocalAnchorA(self.ptr) }
	}

	pub fn get_local_anchor_b(&self) -> &Vec2 {
		unsafe { b2RopeJoint_GetLocalAnchorB(self.ptr) }
	}

	pub fn set_max_length(&self, length: Float32) {
		unsafe { b2RopeJoint_SetMaxLength(self.ptr, length) }
	}

	pub fn get_max_length(&self) -> Float32 {
		unsafe { b2RopeJoint_GetMaxLength(self.ptr) }
	}

	pub fn get_limit_state(&self) -> LimitState {
		unsafe { b2RopeJoint_GetLimitState(self.ptr) }
	}

}

impl Joint for RopeJoint {
	fn get_handle(&self) -> *mut B2Joint {
		self.ptr as *mut B2Joint
	}

	fn get_next(&self) -> Self
	{
		unsafe {
			RopeJoint { ptr: b2Joint_GetNext(self.get_handle()) as *mut _ }
		}
	}
}
