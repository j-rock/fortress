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
pub struct PrismaticJointDef {

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

	/// The local translation unit axis in bodyA.
	pub local_axis_a: Vec2,

	/// The constrained angle between the bodies: bodyB_angle - bodyA_angle.
	pub reference_angle: Float32,

	/// Enable/disable the joint limit.
	pub enable_limit: bool,

	/// The lower translation limit, usually in meters.
	pub lower_translation: Float32,

	/// The upper translation limit, usually in meters.
	pub upper_translation: Float32,

	/// Enable/disable the joint motor.
	pub enable_motor: bool,

	/// The maximum motor torque, usually in N-m.
	pub max_motor_force: Float32,

	/// The desired motor speed in radians per second.
	pub motor_speed: Float32,
}

impl Default for PrismaticJointDef {
	fn default() -> Self {
		PrismaticJointDef {
			user_data: ptr::null_mut(),
			body_a: None,
			body_b: None,
			collide_connected: false,
			local_anchor_a: Vec2::zero(),
			local_anchor_b: Vec2::zero(),
			local_axis_a: Vec2::zero(),
			reference_angle: 0.0,
			enable_limit: false,
			lower_translation: 0.0,
			upper_translation: 0.0,
			enable_motor: false,
			max_motor_force: 0.0,
			motor_speed: 0.0,
	    }
	}
}

impl JointDef<PrismaticJoint> for PrismaticJointDef {
	fn joint_type() -> JointType { JointType::PrismaticJoint }

	fn create(&self, world: &mut World) -> PrismaticJoint {
		unsafe { PrismaticJoint { ptr: b2PrismaticJoint_Create(
			world.ptr,
			self.user_data,
			if let Some(ref p) = self.body_a { p.ptr } else { ptr::null_mut() },
		    if let Some(ref p) = self.body_b { p.ptr } else { ptr::null_mut() },
		    self.collide_connected,
			self.local_anchor_a,
			self.local_anchor_b,
			self.local_axis_a,
			self.reference_angle,
			self.enable_limit,
			self.lower_translation,
			self.upper_translation,
			self.enable_motor,
			self.max_motor_force,
			self.motor_speed,
		) } }
	}
}

pub enum B2PrismaticJoint {}

extern {
    fn b2PrismaticJoint_Create(
		world: *mut B2World,
		userData: *mut c_void,
		bodyA: *mut B2Body,
	    bodyB: *mut B2Body,
	    collideConnected: bool,
		localAnchorA: Vec2,
		localAnchorB: Vec2,
		localAxisA: Vec2,
		referenceAngle: Float32,
		enableLimit: bool,
		lowerTranslation: Float32,
		upperTranslation: Float32,
		enableMotor: bool,
		maxMotorForce: Float32,
		motorSpeed: Float32,
	) -> *mut B2PrismaticJoint;

	fn b2PrismaticJoint_GetLocalAxisA(this: &B2PrismaticJoint) -> &Vec2;
	fn b2PrismaticJoint_GetReferenceAngle(this: *const B2PrismaticJoint) -> Float32;
	fn b2PrismaticJoint_GetJointTranslation(this: *const B2PrismaticJoint) -> Float32;
	fn b2PrismaticJoint_GetJointSpeed(this: *const B2PrismaticJoint) -> Float32;
	fn b2PrismaticJoint_IsLimitEnabled(this: *const B2PrismaticJoint) -> bool;
	fn b2PrismaticJoint_EnableLimit(this: *const B2PrismaticJoint, flag: bool);
	fn b2PrismaticJoint_GetLowerLimit(this: *const B2PrismaticJoint) -> Float32;
	fn b2PrismaticJoint_GetUpperLimit(this: *const B2PrismaticJoint) -> Float32;
	fn b2PrismaticJoint_SetLimits(this: *const B2PrismaticJoint, lower: Float32, upper: Float32);
	fn b2PrismaticJoint_IsMotorEnabled(this: *const B2PrismaticJoint) -> bool;
	fn b2PrismaticJoint_EnableMotor(this: *const B2PrismaticJoint, flag: bool);
	fn b2PrismaticJoint_SetMotorSpeed(this: *const B2PrismaticJoint, speed: Float32);
	fn b2PrismaticJoint_GetMotorSpeed(this: *const B2PrismaticJoint) -> Float32;
	fn b2PrismaticJoint_SetMaxMotorForce(this: *const B2PrismaticJoint, force: Float32);
	fn b2PrismaticJoint_GetMaxMotorForce(this: *const B2PrismaticJoint) -> Float32;
	fn b2PrismaticJoint_GetMotorForce(this: *const B2PrismaticJoint, inv_dt: Float32) -> Float32;
}

#[derive(Clone, Debug)]
pub struct PrismaticJoint {
	pub ptr: *mut B2PrismaticJoint
}

impl PrismaticJoint {
	pub fn get_local_axis_a(&self) -> &Vec2 {
		unsafe { b2PrismaticJoint_GetLocalAxisA(&*self.ptr) }
	}

	pub fn get_reference_angle(&self) -> Float32 {
		unsafe { b2PrismaticJoint_GetReferenceAngle(self.ptr) }
	}

	pub fn get_joint_translation(&self) -> Float32 {
		unsafe { b2PrismaticJoint_GetJointTranslation(self.ptr) }
	}

	pub fn get_joint_speed(&self) -> Float32 {
		unsafe { b2PrismaticJoint_GetJointSpeed(self.ptr) }
	}

	pub fn is_limit_enabled(&self) -> bool {
		unsafe { b2PrismaticJoint_IsLimitEnabled(self.ptr) }
	}

	pub fn enable_limit(&self, flag: bool) {
		unsafe { b2PrismaticJoint_EnableLimit(self.ptr, flag) }
	}

	pub fn get_lower_limit(&self) -> Float32 {
		unsafe { b2PrismaticJoint_GetLowerLimit(self.ptr) }
	}

	pub fn get_upper_limit(&self) -> Float32 {
		unsafe { b2PrismaticJoint_GetUpperLimit(self.ptr) }
	}

	pub fn set_limits(&self, lower: Float32, upper: Float32) {
		unsafe { b2PrismaticJoint_SetLimits(self.ptr, lower, upper) }
	}

	pub fn is_motor_enabled(&self) -> bool {
		unsafe { b2PrismaticJoint_IsMotorEnabled(self.ptr) }
	}

	pub fn enable_motor(&self, flag: bool) {
		unsafe { b2PrismaticJoint_EnableMotor(self.ptr, flag) }
	}

	pub fn set_motor_speed(&self, speed: Float32) {
		unsafe { b2PrismaticJoint_SetMotorSpeed(self.ptr, speed) }
	}

	pub fn get_motor_speed(&self) -> Float32 {
		unsafe { b2PrismaticJoint_GetMotorSpeed(self.ptr) }
	}

	pub fn set_max_motor_force(&self, force: Float32) {
		unsafe { b2PrismaticJoint_SetMaxMotorForce(self.ptr, force) }
	}

	pub fn get_max_motor_force(&self) -> Float32 {
		unsafe { b2PrismaticJoint_GetMaxMotorForce(self.ptr) }
	}

	pub fn get_motor_force(&self, inv_dt: Float32) -> Float32 {
		unsafe { b2PrismaticJoint_GetMotorForce(self.ptr, inv_dt) }
	}

}


impl Joint for PrismaticJoint {
	fn get_handle(&self) -> *mut B2Joint {
		self.ptr as *mut B2Joint
	}

	fn get_next(&self) -> Self
	{
		unsafe {
			PrismaticJoint { ptr: b2Joint_GetNext(self.get_handle()) as *mut _ }
		}
	}
}
