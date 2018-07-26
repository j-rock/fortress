use libc::c_void;
use std::ptr;

use super::super::super::dynamics::world::{B2World, World};
use super::super::super::common::math::*;
use super::super::super::common::settings::*;
use super::super::super::dynamics::body::{B2Body, Body};
use super::{JointType, JointDef, Joint, B2Joint, b2Joint_GetNext};

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
pub struct MouseJointDef {

	/// Use this to attach application specific data to your joints.
	pub user_data: *mut c_void,

	/// The first attached body.
	pub body_a: Option<Body>,

	/// The second attached body.
	pub body_b: Option<Body>,

	/// Set this flag to true if the attached bodies should collide.
	pub collide_connected: bool,

	/// The initial world target point. This is assumed
	/// to coincide with the body anchor initially.
	pub target: Vec2,

	/// The maximum constraint force that can be exerted
	/// to move the candidate body. Usually you will express
	/// as some multiple of the weight (multiplier * mass * gravity).
	pub max_force: Float32,

	/// The response speed.
	pub frequency_hz: Float32,

	/// The damping ratio. 0 = no damping, 1 = critical damping.
	pub damping_ratio: Float32,
}

impl Default for MouseJointDef {
	fn default() -> Self {
		MouseJointDef {
			user_data: ptr::null_mut(),
			body_a: None,
			body_b: None,
			collide_connected: false,
			target: Vec2::zero(),
			max_force: 0.0,
			frequency_hz: 0.0,
			damping_ratio: 0.0,
	    }
	}
}

impl JointDef<MouseJoint> for MouseJointDef {
	fn joint_type() -> JointType { JointType::MouseJoint }

	fn create(&self, world: &mut World) -> MouseJoint {
		unsafe { MouseJoint { ptr: b2MouseJoint_Create(
			world.ptr,
			self.user_data,
			if let Some(ref p) = self.body_a { p.ptr } else { ptr::null_mut() },
		    if let Some(ref p) = self.body_b { p.ptr } else { ptr::null_mut() },
		    self.collide_connected,
			self.target,
			self.max_force,
			self.frequency_hz,
			self.damping_ratio
		) } }
	}
}

pub enum B2MouseJoint {}

extern {
    fn b2MouseJoint_Create(
		world: *mut B2World,
		userData: *mut c_void,
		bodyA: *mut B2Body,
	    bodyB: *mut B2Body,
	    collideConnected: bool,
		target: Vec2,
		maxForce: Float32,
		frequencyHz: Float32,
		dampingRatio: Float32
	) -> *mut B2MouseJoint;

	fn b2MouseJoint_SetTarget(this: &B2MouseJoint, target: &Vec2);
	fn b2MouseJoint_GetTarget(this: &B2MouseJoint) -> &Vec2;
	fn b2MouseJoint_SetMaxForce(this: *const B2MouseJoint, force: Float32);
	fn b2MouseJoint_GetMaxForce(this: *const B2MouseJoint) -> Float32;
	fn b2MouseJoint_SetFrequency(this: *const B2MouseJoint, hz: Float32);
	fn b2MouseJoint_GetFrequency(this: *const B2MouseJoint) -> Float32;
	fn b2MouseJoint_SetDampingRatio(this: *const B2MouseJoint, ratio: Float32);
	fn b2MouseJoint_GetDampingRatio(this: *const B2MouseJoint) -> Float32;
}

#[derive(Clone, Debug)]
pub struct MouseJoint {
	pub ptr: *mut B2MouseJoint
}

impl MouseJoint {
	pub fn set_target(&self, target: &Vec2) {
		unsafe { b2MouseJoint_SetTarget(&*self.ptr, target) }
	}

	pub fn get_target(&self) -> &Vec2 {
		unsafe { b2MouseJoint_GetTarget(&*self.ptr) }
	}

	pub fn set_max_force(&self, force: Float32) {
		unsafe { b2MouseJoint_SetMaxForce(self.ptr, force) }
	}

	pub fn get_max_force(&self) -> Float32 {
		unsafe { b2MouseJoint_GetMaxForce(self.ptr) }
	}

	pub fn set_frequency(&self, hz: Float32) {
		unsafe { b2MouseJoint_SetFrequency(self.ptr, hz) }
	}

	pub fn get_frequency(&self) -> Float32 {
		unsafe { b2MouseJoint_GetFrequency(self.ptr) }
	}

	pub fn set_damping_ratio(&self, ratio: Float32) {
		unsafe { b2MouseJoint_SetDampingRatio(self.ptr, ratio) }
	}

	pub fn get_damping_ratio(&self) -> Float32 {
		unsafe { b2MouseJoint_GetDampingRatio(self.ptr) }
	}
}


impl Joint for MouseJoint {
	fn get_handle(&self) -> *mut B2Joint {
		self.ptr as *mut B2Joint
	}

	fn get_next(&self) -> Self
	{
		unsafe {
			MouseJoint { ptr: b2Joint_GetNext(self.get_handle()) as *mut _ }
		}
	}
}
