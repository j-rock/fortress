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
pub struct MotorJointDef {

	/// Use this to attach application specific data to your joints.
	pub user_data: *mut c_void,

	/// The first attached body.
	pub body_a: Option<Body>,

	/// The second attached body.
	pub body_b: Option<Body>,

	/// Set this flag to true if the attached bodies should collide.
	pub collide_connected: bool,

	/// Position of bodyB minus the position of bodyA, in bodyA's frame, in meters.
	pub linear_offset: Vec2,

	/// The bodyB angle minus bodyA angle in radians.
	pub angular_offset: Float32,

	/// The maximum motor force in N.
	pub max_force: Float32,

	/// The maximum motor torque in N-m.
	pub max_torque: Float32,

	/// Position correction factor in the range [0,1].
	pub correction_factor: Float32,
}

impl MotorJointDef {
	/// Initialize the bodies and offsets using the current transforms.
    pub fn initialize(&mut self, body_a: Body, body_b: Body) {
		self.linear_offset = body_a.get_local_point(body_b.get_position());

		self.angular_offset = body_b.get_angle() - body_a.get_angle();
		self.body_a = Some(body_a);
		self.body_b = Some(body_b);
    }
}

impl Default for MotorJointDef {
	fn default() -> Self {
		MotorJointDef {
			user_data: ptr::null_mut(),
			body_a: None,
			body_b: None,
			collide_connected: false,
			linear_offset: Vec2::zero(),
			angular_offset: 0.0,
			max_force: 0.0,
			max_torque: 0.0,
			correction_factor: 0.0,
	    }
	}
}

impl JointDef<MotorJoint> for MotorJointDef {
	fn joint_type() -> JointType { JointType::MotorJoint }

	fn create(&self, world: &mut World) -> MotorJoint {
		unsafe { MotorJoint { ptr: b2MotorJoint_Create(
			world.ptr,
			self.user_data,
			if let Some(ref p) = self.body_a { p.ptr } else { ptr::null_mut() },
		    if let Some(ref p) = self.body_b { p.ptr } else { ptr::null_mut() },
		    self.collide_connected,
			self.linear_offset,
			self.angular_offset,
			self.max_force,
			self.max_torque,
			self.correction_factor,
		) } }
	}
}

pub enum B2MotorJoint {}

extern {
    fn b2MotorJoint_Create(
		world: *mut B2World,
		userData: *mut c_void,
		bodyA: *mut B2Body,
	    bodyB: *mut B2Body,
	    collideConnected: bool,
		linearOffset: Vec2,
		angularOffset: Float32,
		maxForce: Float32,
		maxTorque: Float32,
		correctionFactor: Float32
	) -> *mut B2MotorJoint;

    fn b2MotorJoint_GetLocalAnchorA(this: *const B2MotorJoint) -> &Vec2;
    fn b2MotorJoint_GetLocalAnchorB(this: *const B2MotorJoint) -> &Vec2;
    fn b2MotorJoint_SetLength(this: *const B2MotorJoint, length: Float32);
    fn b2MotorJoint_GetLength(this: *const B2MotorJoint) -> Float32;
    fn b2MotorJoint_SetFrequency(this: *const B2MotorJoint, hz: Float32);
    fn b2MotorJoint_GetFrequency(this: *const B2MotorJoint) -> Float32;
    fn b2MotorJoint_SetDampingRatio(this: *const B2MotorJoint, ratio: Float32);
    fn b2MotorJoint_GetDampingRatio(this: *const B2MotorJoint) -> Float32;
}

#[derive(Clone, Debug)]
pub struct MotorJoint {
	pub ptr: *mut B2MotorJoint
}

impl MotorJoint {
    pub fn get_local_anchor_a(&self) -> &Vec2 {
        unsafe { b2MotorJoint_GetLocalAnchorA(self.ptr) }
    }

    pub fn get_local_anchor_b(&self) -> &Vec2 {
        unsafe { b2MotorJoint_GetLocalAnchorB(self.ptr) }
    }

    pub fn set_length(&self, length: Float32) {
        unsafe { b2MotorJoint_SetLength(self.ptr, length) }
    }

    pub fn get_length(&self) -> Float32 {
        unsafe { b2MotorJoint_GetLength(self.ptr) }
    }

    pub fn set_frequency(&self, hz: Float32) {
        unsafe { b2MotorJoint_SetFrequency(self.ptr, hz) }
    }

    pub fn get_frequency(&self) -> Float32 {
        unsafe { b2MotorJoint_GetFrequency(self.ptr) }
    }

    pub fn set_damping_ratio(&self, ratio: Float32) {
        unsafe { b2MotorJoint_SetDampingRatio(self.ptr, ratio) }
    }

    pub fn get_damping_ratio(&self) -> Float32 {
        unsafe { b2MotorJoint_GetDampingRatio(self.ptr) }
    }

}

impl Joint for MotorJoint {
	fn get_handle(&self) -> *mut B2Joint {
		self.ptr as *mut B2Joint
	}

	fn get_next(&self) -> Self
	{
		unsafe {
			MotorJoint { ptr: b2Joint_GetNext(self.get_handle()) as *mut _ }
		}
	}
}
