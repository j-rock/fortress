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
pub struct PulleyJointDef {

	/// Use this to attach application specific data to your joints.
	pub user_data: *mut c_void,

	/// The first attached body.
	pub body_a: Option<Body>,

	/// The second attached body.
	pub body_b: Option<Body>,

	/// Set this flag to true if the attached bodies should collide.
	pub collide_connected: bool,

	/// The first ground anchor in world coordinates. This point never moves.
	pub ground_anchor_a: Vec2,

	/// The second ground anchor in world coordinates. This point never moves.
	pub ground_anchor_b: Vec2,

	/// The local anchor point relative to bodyA's origin.
	pub local_anchor_a: Vec2,

	/// The local anchor point relative to bodyB's origin.
	pub local_anchor_b: Vec2,

	/// The a reference length for the segment attached to bodyA.
	pub length_a: Float32,

	/// The a reference length for the segment attached to bodyB.
	pub length_b: Float32,

	/// The pulley ratio, used to simulate a block-and-tackle.
	pub ratio: Float32,
}

impl Default for PulleyJointDef {
	fn default() -> Self {
		PulleyJointDef {
			user_data: ptr::null_mut(),
			body_a: None,
			body_b: None,
			collide_connected: false,
			ground_anchor_a: Vec2::zero(),
			ground_anchor_b: Vec2::zero(),
			local_anchor_a: Vec2::zero(),
			local_anchor_b: Vec2::zero(),
			length_a: 0.0,
			length_b: 0.0,
			ratio: 0.0,
	    }
	}
}

impl JointDef<PulleyJoint> for PulleyJointDef {
	fn joint_type() -> JointType { JointType::PulleyJoint }

	fn create(&self, world: &mut World) -> PulleyJoint {
		unsafe { PulleyJoint { ptr: b2PulleyJoint_Create(
			world.ptr,
			self.user_data,
			if let Some(ref p) = self.body_a { p.ptr } else { ptr::null_mut() },
		    if let Some(ref p) = self.body_b { p.ptr } else { ptr::null_mut() },
		    self.collide_connected,
			self.ground_anchor_a,
			self.ground_anchor_b,
			self.local_anchor_a,
			self.local_anchor_b,
			self.length_a,
			self.length_b,
			self.ratio,
		) } }
	}
}

pub enum B2PulleyJoint {}

extern {
    fn b2PulleyJoint_Create(
		world: *mut B2World,
		userData: *mut c_void,
		bodyA: *mut B2Body,
	    bodyB: *mut B2Body,
	    collideConnected: bool,
		groundAnchorA: Vec2,
		groundAnchorB: Vec2,
		localAnchorA: Vec2,
		localAnchorB: Vec2,
		lengthA: Float32,
		lengthB: Float32,
		ratio: Float32
	) -> *mut B2PulleyJoint;

	fn b2PulleyJoint_GetGroundAnchorA(this: *const B2PulleyJoint) -> Vec2;
	fn b2PulleyJoint_GetGroundAnchorB(this: *const B2PulleyJoint) -> Vec2;
	fn b2PulleyJoint_GetLengthA(this: *const B2PulleyJoint) -> Float32;
	fn b2PulleyJoint_GetLengthB(this: *const B2PulleyJoint) -> Float32;
	fn b2PulleyJoint_GetRatio(this: *const B2PulleyJoint) -> Float32;
	fn b2PulleyJoint_GetCurrentLengthA(this: *const B2PulleyJoint) -> Float32;
	fn b2PulleyJoint_GetCurrentLengthB(this: *const B2PulleyJoint) -> Float32;
}

#[derive(Clone, Debug)]
pub struct PulleyJoint {
	pub ptr: *mut B2PulleyJoint
}

impl PulleyJoint {
	pub fn get_ground_anchor_a(&self) -> Vec2 {
		unsafe { b2PulleyJoint_GetGroundAnchorA(self.ptr) }
	}

	pub fn get_ground_anchor_b(&self) -> Vec2 {
		unsafe { b2PulleyJoint_GetGroundAnchorB(self.ptr) }
	}

	pub fn get_length_a(&self) -> Float32 {
		unsafe { b2PulleyJoint_GetLengthA(self.ptr) }
	}

	pub fn get_length_b(&self) -> Float32 {
		unsafe { b2PulleyJoint_GetLengthB(self.ptr) }
	}

	pub fn get_ratio(&self) -> Float32 {
		unsafe { b2PulleyJoint_GetRatio(self.ptr) }
	}

	pub fn get_current_length_a(&self) -> Float32 {
		unsafe { b2PulleyJoint_GetCurrentLengthA(self.ptr) }
	}

	pub fn get_current_length_b(&self) -> Float32 {
		unsafe { b2PulleyJoint_GetCurrentLengthB(self.ptr) }
	}

}

impl Joint for PulleyJoint {
	fn get_handle(&self) -> *mut B2Joint {
		self.ptr as *mut B2Joint
	}

	fn get_next(&self) -> Self
	{
		unsafe {
			PulleyJoint { ptr: b2Joint_GetNext(self.get_handle()) as *mut _ }
		}
	}
}
