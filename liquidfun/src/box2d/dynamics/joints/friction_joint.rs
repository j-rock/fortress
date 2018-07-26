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
pub struct FrictionJointDef {

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

	/// The maximum friction force in N.
	pub max_force: Float32,

	/// The maximum friction torque in N-m.
	pub max_torque: Float32,
}

impl FrictionJointDef {
    // pub fn initialize(&mut self, body_a: Body, body_b: Body, anchor1: &Vec2, anchor2: &Vec2) {
	// 	unimplemented!()
    // }
}

impl Default for FrictionJointDef {
	fn default() -> Self {
		FrictionJointDef {
			user_data: ptr::null_mut(),
			body_a: None,
			body_b: None,
			collide_connected: false,
			local_anchor_a: Vec2::zero(),
			local_anchor_b: Vec2::zero(),
			max_force: 0.0,
			max_torque: 0.0,
	    }
	}
}

impl JointDef<FrictionJoint> for FrictionJointDef {
	fn joint_type() -> JointType { JointType::FrictionJoint }

	fn create(&self, world: &mut World) -> FrictionJoint {
		unsafe { FrictionJoint { ptr: b2FrictionJoint_Create(
			world.ptr,
			self.user_data,
			if let Some(ref p) = self.body_a { p.ptr } else { ptr::null_mut() },
		    if let Some(ref p) = self.body_b { p.ptr } else { ptr::null_mut() },
		    self.collide_connected,
			self.local_anchor_a,
			self.local_anchor_b,
			self.max_force,
			self.max_torque,
		) } }
	}
}

pub enum B2FrictionJoint {}

extern {
    fn b2FrictionJoint_Create(
		world: *mut B2World,
		userData: *mut c_void,
		bodyA: *mut B2Body,
	    bodyB: *mut B2Body,
	    collideConnected: bool,
		localAnchorA: Vec2,
		localAnchorB: Vec2,
		maxForce: Float32,
		maxTorque: Float32
	) -> *mut B2FrictionJoint;

    fn b2FrictionJoint_GetLocalAnchorA(this: *const B2FrictionJoint) -> &Vec2;
    fn b2FrictionJoint_GetLocalAnchorB(this: *const B2FrictionJoint) -> &Vec2;
    fn b2FrictionJoint_SetLength(this: *const B2FrictionJoint, length: Float32);
    fn b2FrictionJoint_GetLength(this: *const B2FrictionJoint) -> Float32;
    fn b2FrictionJoint_SetFrequency(this: *const B2FrictionJoint, hz: Float32);
    fn b2FrictionJoint_GetFrequency(this: *const B2FrictionJoint) -> Float32;
    fn b2FrictionJoint_SetDampingRatio(this: *const B2FrictionJoint, ratio: Float32);
    fn b2FrictionJoint_GetDampingRatio(this: *const B2FrictionJoint) -> Float32;
}

#[derive(Clone, Debug)]
pub struct FrictionJoint {
	pub ptr: *mut B2FrictionJoint
}

impl FrictionJoint {
    pub fn get_local_anchor_a(&self) -> &Vec2 {
        unsafe { b2FrictionJoint_GetLocalAnchorA(self.ptr) }
    }

    pub fn get_local_anchor_b(&self) -> &Vec2 {
        unsafe { b2FrictionJoint_GetLocalAnchorB(self.ptr) }
    }

    pub fn set_length(&self, length: Float32) {
        unsafe { b2FrictionJoint_SetLength(self.ptr, length) }
    }

    pub fn get_length(&self) -> Float32 {
        unsafe { b2FrictionJoint_GetLength(self.ptr) }
    }

    pub fn set_frequency(&self, hz: Float32) {
        unsafe { b2FrictionJoint_SetFrequency(self.ptr, hz) }
    }

    pub fn get_frequency(&self) -> Float32 {
        unsafe { b2FrictionJoint_GetFrequency(self.ptr) }
    }

    pub fn set_damping_ratio(&self, ratio: Float32) {
        unsafe { b2FrictionJoint_SetDampingRatio(self.ptr, ratio) }
    }

    pub fn get_damping_ratio(&self) -> Float32 {
        unsafe { b2FrictionJoint_GetDampingRatio(self.ptr) }
    }

}

impl Joint for FrictionJoint {
	fn get_handle(&self) -> *mut B2Joint {
		self.ptr as *mut B2Joint
	}

	fn get_next(&self) -> Self
	{
		unsafe {
			FrictionJoint { ptr: b2Joint_GetNext(self.get_handle()) as *mut _ }
		}
	}
}
