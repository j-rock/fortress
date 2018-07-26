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
pub struct RevoluteJointDef {

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

	/// A flag to enable joint limits.
	pub enable_limit: bool,

	/// The lower angle for the joint limit (radians).
	pub lower_angle: Float32,

	/// The upper angle for the joint limit (radians).
	pub upper_angle: Float32,

	/// A flag to enable the joint motor.
	pub enable_motor: bool,

	/// The desired motor speed. Usually in radians per second.
	pub motor_speed: Float32,

	/// The maximum motor torque used to achieve the desired motor speed.
	/// Usually in N-m.
	pub max_motor_torque: Float32,
}

impl RevoluteJointDef {
    pub fn initialize(&mut self, body_a: Body, body_b: Body, anchor: &Vec2) {
		self.local_anchor_a = body_a.get_local_point(anchor);
		self.local_anchor_b = body_b.get_local_point(anchor);
		self.reference_angle = body_b.get_angle() - body_a.get_angle();
		self.body_a = Some(body_a);
		self.body_b = Some(body_b);
    }
}

impl Default for RevoluteJointDef {
	fn default() -> Self {
		RevoluteJointDef {
			user_data: ptr::null_mut(),
			body_a: None,
			body_b: None,
			collide_connected: false,
			local_anchor_a: Vec2::zero(),
			local_anchor_b: Vec2::zero(),
			reference_angle: 0.0,
			enable_limit: false,
			lower_angle: 0.0,
			upper_angle: 0.0,
			enable_motor: false,
			motor_speed: 0.0,
			max_motor_torque: 0.0,
	    }
	}
}

impl JointDef<RevoluteJoint> for RevoluteJointDef {
	fn joint_type() -> JointType { JointType::RevoluteJoint }

	fn create(&self, world: &mut World) -> RevoluteJoint {
		unsafe { RevoluteJoint { ptr: b2RevoluteJoint_Create(
			world.ptr,
			self.user_data,
			if let Some(ref p) = self.body_a { p.ptr } else { ptr::null_mut() },
		    if let Some(ref p) = self.body_b { p.ptr } else { ptr::null_mut() },
		    self.collide_connected,
		    self.local_anchor_a,
		    self.local_anchor_b,
		    self.reference_angle,
		    self.enable_limit,
		    self.lower_angle,
		    self.upper_angle,
		    self.enable_motor,
		    self.motor_speed,
		    self.max_motor_torque,
		) } }
	}
}

pub enum B2RevoluteJoint {}

extern {
	fn b2RevoluteJoint_Create(
		world: *mut B2World,
		userData: *mut c_void,
		bodyA: *mut B2Body,
	    bodyB: *mut B2Body,
	    collideConnected: bool,
	    localAnchorA: Vec2,
	    localAnchorB: Vec2,
	    referenceAngle: Float32,
	    enableLimit: bool,
	    lowerAngle: Float32,
	    upperAngle: Float32,
	    enableMotor: bool,
	    motorSpeed: Float32,
	    maxMotorTorque: Float32
	) -> *mut B2RevoluteJoint;

	fn b2RevoluteJoint_GetReferenceAngle(this: *const B2RevoluteJoint) -> Float32;
	fn b2RevoluteJoint_GetJointAngle(this: *const B2RevoluteJoint) -> Float32;
	fn b2RevoluteJoint_GetJointSpeed(this: *const B2RevoluteJoint) -> Float32;
	fn b2RevoluteJoint_IsLimitEnabled(this: *const B2RevoluteJoint) -> bool;
	fn b2RevoluteJoint_EnableLimit(this: *const B2RevoluteJoint, flag: bool);
	fn b2RevoluteJoint_GetLowerLimit(this: *const B2RevoluteJoint) -> Float32;
	fn b2RevoluteJoint_GetUpperLimit(this: *const B2RevoluteJoint) -> Float32;
	fn b2RevoluteJoint_SetLimits(this: *const B2RevoluteJoint, lower: Float32, upper: Float32);
	fn b2RevoluteJoint_IsMotorEnabled(this: *const B2RevoluteJoint) -> bool;
	fn b2RevoluteJoint_EnableMotor(this: *const B2RevoluteJoint, flag: bool);
	fn b2RevoluteJoint_SetMotorSpeed(this: *const B2RevoluteJoint, speed: Float32);
	fn b2RevoluteJoint_GetMotorSpeed(this: *const B2RevoluteJoint) -> Float32;
	fn b2RevoluteJoint_SetMaxMotorTorque(this: *const B2RevoluteJoint, torque: Float32);
	fn b2RevoluteJoint_GetMaxMotorTorque(this: *const B2RevoluteJoint) -> Float32;
	fn b2RevoluteJoint_GetMotorTorque(this: *const B2RevoluteJoint, inv_dt: Float32) -> Float32;
}


/// A revolute joint constrains two bodies to share a common point while they
/// are free to rotate about the point. The relative rotation about the shared
/// point is the joint angle. You can limit the relative rotation with
/// a joint limit that specifies a lower and upper angle. You can use a motor
/// to drive the relative rotation about the shared point. A maximum motor torque
/// is provided so that infinite forces are not generated.
#[derive(Clone, Debug)]
pub struct RevoluteJoint {
	pub ptr: *mut B2RevoluteJoint
}

impl RevoluteJoint {
	pub fn get_reference_angle(&self) -> Float32 {
		unsafe { b2RevoluteJoint_GetReferenceAngle(self.ptr) }
	}

	pub fn get_joint_angle(&self) -> Float32 {
		unsafe { b2RevoluteJoint_GetJointAngle(self.ptr) }
	}

	pub fn get_joint_speed(&self) -> Float32 {
		unsafe { b2RevoluteJoint_GetJointSpeed(self.ptr) }
	}

	pub fn is_limit_enabled(&self) -> bool {
		unsafe { b2RevoluteJoint_IsLimitEnabled(self.ptr) }
	}

	pub fn enable_limit(&self, flag: bool) {
		unsafe { b2RevoluteJoint_EnableLimit(self.ptr, flag) }
	}

	pub fn get_lower_limit(&self) -> Float32 {
		unsafe { b2RevoluteJoint_GetLowerLimit(self.ptr) }
	}

	pub fn get_upper_limit(&self) -> Float32 {
		unsafe { b2RevoluteJoint_GetUpperLimit(self.ptr) }
	}

	pub fn set_limits(&self, lower: Float32, upper: Float32) {
		unsafe { b2RevoluteJoint_SetLimits(self.ptr, lower, upper) }
	}

	pub fn is_motor_enabled(&self) -> bool {
		unsafe { b2RevoluteJoint_IsMotorEnabled(self.ptr) }
	}

	pub fn enable_motor(&self, flag: bool) {
		unsafe { b2RevoluteJoint_EnableMotor(self.ptr, flag) }
	}

	pub fn set_motor_speed(&self, speed: Float32) {
		unsafe { b2RevoluteJoint_SetMotorSpeed(self.ptr, speed) }
	}

	pub fn get_motor_speed(&self) -> Float32 {
		unsafe { b2RevoluteJoint_GetMotorSpeed(self.ptr) }
	}

	pub fn set_max_motor_torque(&self, torque: Float32) {
		unsafe { b2RevoluteJoint_SetMaxMotorTorque(self.ptr, torque) }
	}

	pub fn get_max_motor_torque(&self) -> Float32 {
		unsafe { b2RevoluteJoint_GetMaxMotorTorque(self.ptr) }
	}

	pub fn get_motor_torque(&self, inv_dt: Float32) -> Float32 {
		unsafe { b2RevoluteJoint_GetMotorTorque(self.ptr, inv_dt) }
	}

}

impl Joint for RevoluteJoint {
	fn get_handle(&self) -> *mut B2Joint {
		self.ptr as *mut B2Joint
	}

	fn get_next(&self) -> Self
	{
		unsafe {
			RevoluteJoint { ptr: b2Joint_GetNext(self.get_handle()) as *mut _ }
		}
	}
}
