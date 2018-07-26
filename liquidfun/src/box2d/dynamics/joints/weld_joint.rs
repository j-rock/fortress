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
pub struct WeldJointDef {

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

	/// The bodyB angle minus bodyA angle in the reference state (radians).
	pub reference_angle: Float32,

	/// The mass-spring-damper frequency in Hertz. Rotation only.
	/// Disable softness with a value of 0.
	pub frequency_hz: Float32,

	/// The damping ratio. 0 = no damping, 1 = critical damping.
	pub damping_ratio: Float32,
}

impl Default for WeldJointDef {
	fn default() -> Self {
		WeldJointDef {
			user_data: ptr::null_mut(),
			body_a: None,
			body_b: None,
			collide_connected: false,
			local_anchor_a: Vec2::zero(),
			local_anchor_b: Vec2::zero(),
			reference_angle: 0.0,
			frequency_hz: 0.0,
			damping_ratio: 0.0,
	    }
	}
}

impl JointDef<WeldJoint> for WeldJointDef {
	fn joint_type() -> JointType { JointType::WeldJoint }

	fn create(&self, world: &mut World) -> WeldJoint {
		unsafe { WeldJoint { ptr: b2WeldJoint_Create(
			world.ptr,
			self.user_data,
			if let Some(ref p) = self.body_a { p.ptr } else { ptr::null_mut() },
		    if let Some(ref p) = self.body_b { p.ptr } else { ptr::null_mut() },
		    self.collide_connected,
			self.local_anchor_a,
			self.local_anchor_b,
			self.reference_angle,
			self.frequency_hz,
			self.damping_ratio,
		) } }
	}
}

pub enum B2WeldJoint {}

extern {
    fn b2WeldJoint_Create(
		world: *mut B2World,
		userData: *mut c_void,
		bodyA: *mut B2Body,
	    bodyB: *mut B2Body,
	    collideConnected: bool,
    	localAnchorA: Vec2,
    	localAnchorB: Vec2,
    	referenceAngle: Float32,
    	frequencyHz: Float32,
    	dampingRatio: Float32
	) -> *mut B2WeldJoint;

	fn b2WeldJoint_GetLocalAnchorA(this: &B2WeldJoint) -> &Vec2;
	fn b2WeldJoint_GetLocalAnchorB(this: &B2WeldJoint) -> &Vec2;
	fn b2WeldJoint_GetReferenceAngle(this: *const B2WeldJoint) -> Float32;
	fn b2WeldJoint_SetFrequency(this: *const B2WeldJoint, hz: Float32);
	fn b2WeldJoint_GetFrequency(this: *const B2WeldJoint) -> Float32;
	fn b2WeldJoint_SetDampingRatio(this: *const B2WeldJoint, ratio: Float32);
	fn b2WeldJoint_GetDampingRatio(this: *const B2WeldJoint) -> Float32;
}

#[derive(Clone, Debug)]
pub struct WeldJoint {
	pub ptr: *mut B2WeldJoint
}

impl WeldJoint {
	pub fn get_local_anchor_a(&self) -> &Vec2 {
		unsafe { b2WeldJoint_GetLocalAnchorA(&*self.ptr) }
	}

	pub fn get_local_anchor_b(&self) -> &Vec2 {
		unsafe { b2WeldJoint_GetLocalAnchorB(&*self.ptr) }
	}

	pub fn get_reference_angle(&self) -> Float32 {
		unsafe { b2WeldJoint_GetReferenceAngle(self.ptr) }
	}

	pub fn set_frequency(&self, hz: Float32) {
		unsafe { b2WeldJoint_SetFrequency(self.ptr, hz) }
	}

	pub fn get_frequency(&self) -> Float32 {
		unsafe { b2WeldJoint_GetFrequency(self.ptr) }
	}

	pub fn set_damping_ratio(&self, ratio: Float32) {
		unsafe { b2WeldJoint_SetDampingRatio(self.ptr, ratio) }
	}

	pub fn get_damping_ratio(&self) -> Float32 {
		unsafe { b2WeldJoint_GetDampingRatio(self.ptr) }
	}

}

impl Joint for WeldJoint {
	fn get_handle(&self) -> *mut B2Joint {
		self.ptr as *mut B2Joint
	}

	fn get_next(&self) -> Self
	{
		unsafe {
			WeldJoint { ptr: b2Joint_GetNext(self.get_handle()) as *mut _ }
		}
	}
}
