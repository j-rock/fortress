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
pub struct WheelJointDef {

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

	/// The local translation axis in bodyA.
	pub local_axis_a: Vec2,

	/// Enable/disable the joint motor.
	pub enable_motor: bool,

	/// The maximum motor torque, usually in N-m.
	pub max_motor_torque: Float32,

	/// The desired motor speed in radians per second.
	pub motor_speed: Float32,

	/// Suspension frequency, zero indicates no suspension
	pub frequency_hz: Float32,

	/// Suspension damping ratio, one indicates critical damping
	pub damping_ratio: Float32,
}

impl Default for WheelJointDef {
	fn default() -> Self {
		WheelJointDef {
			user_data: ptr::null_mut(),
			body_a: None,
			body_b: None,
			collide_connected: false,
			local_anchor_a: Vec2::zero(),
			local_anchor_b: Vec2::zero(),
			local_axis_a: Vec2::zero(),
			enable_motor: false,
			max_motor_torque: 0.0,
			motor_speed: 0.0,
			frequency_hz: 0.0,
			damping_ratio: 0.0,
	    }
	}
}

impl JointDef<WheelJoint> for WheelJointDef {
	fn joint_type() -> JointType { JointType::WheelJoint }

	fn create(&self, world: &mut World) -> WheelJoint {
		unsafe { WheelJoint { ptr: b2WheelJoint_Create(
			world.ptr,
			self.user_data,
			if let Some(ref p) = self.body_a { p.ptr } else { ptr::null_mut() },
		    if let Some(ref p) = self.body_b { p.ptr } else { ptr::null_mut() },
		    self.collide_connected,
			self.local_anchor_a,
			self.local_anchor_b,
			self.local_axis_a,
			self.enable_motor,
			self.max_motor_torque,
			self.motor_speed,
			self.frequency_hz,
			self.damping_ratio,
		) } }
	}
}

pub enum B2WheelJoint {}

extern {
    fn b2WheelJoint_Create(
		world: *mut B2World,
		userData: *mut c_void,
		bodyA: *mut B2Body,
	    bodyB: *mut B2Body,
	    collideConnected: bool,
		localAnchorA: Vec2,
		localAnchorB: Vec2,
		localAxisA: Vec2,
		enableMotor: bool,
		maxMotorTorque: Float32,
		motorSpeed: Float32,
		frequencyHz: Float32,
		dampingRatio: Float32,
	) -> *mut B2WheelJoint;

	fn b2WheelJoint_GetLocalAnchorA(this: *const B2WheelJoint) -> &Vec2;
	fn b2WheelJoint_GetLocalAnchorB(this: *const B2WheelJoint) -> &Vec2;
	fn b2WheelJoint_GetLocalAxisA(this: *const B2WheelJoint) -> &Vec2;
	fn b2WheelJoint_GetJointTranslation(this: *const B2WheelJoint) -> Float32;
	fn b2WheelJoint_GetJointSpeed(this: *const B2WheelJoint) -> Float32;
	fn b2WheelJoint_IsMotorEnabled(this: *const B2WheelJoint) -> bool;
	fn b2WheelJoint_EnableMotor(this: *const B2WheelJoint, flag: bool);
	fn b2WheelJoint_SetMotorSpeed(this: *const B2WheelJoint, speed: Float32);
	fn b2WheelJoint_GetMotorSpeed(this: *const B2WheelJoint) -> Float32;
	fn b2WheelJoint_SetMaxMotorTorque(this: *const B2WheelJoint, torque: Float32);
	fn b2WheelJoint_GetMaxMotorTorque(this: *const B2WheelJoint) -> Float32;
	fn b2WheelJoint_GetMotorTorque(this: *const B2WheelJoint, inv_dt: Float32) -> Float32;
	fn b2WheelJoint_SetSpringFrequencyHz(this: *const B2WheelJoint, hz: Float32);
	fn b2WheelJoint_GetSpringFrequencyHz(this: *const B2WheelJoint) -> Float32;
	fn b2WheelJoint_SetSpringDampingRatio(this: *const B2WheelJoint, ratio: Float32);
	fn b2WheelJoint_GetSpringDampingRatio(this: *const B2WheelJoint) -> Float32;
}

#[derive(Clone, Debug)]
pub struct WheelJoint {
	pub ptr: *mut B2WheelJoint
}

impl WheelJoint {
	pub fn get_local_anchor_a(&self) -> &Vec2 {
		unsafe { b2WheelJoint_GetLocalAnchorA(self.ptr) }
	}

	pub fn get_local_anchor_b(&self) -> &Vec2 {
		unsafe { b2WheelJoint_GetLocalAnchorB(self.ptr) }
	}

	pub fn get_local_axis_a(&self) -> &Vec2 {
		unsafe { b2WheelJoint_GetLocalAxisA(self.ptr) }
	}

	pub fn get_joint_translation(&self) -> Float32 {
		unsafe { b2WheelJoint_GetJointTranslation(self.ptr) }
	}

	pub fn get_joint_speed(&self) -> Float32 {
		unsafe { b2WheelJoint_GetJointSpeed(self.ptr) }
	}

	pub fn is_motor_enabled(&self) -> bool {
		unsafe { b2WheelJoint_IsMotorEnabled(self.ptr) }
	}

	pub fn enable_motor(&self, flag: bool) {
		unsafe { b2WheelJoint_EnableMotor(self.ptr, flag) }
	}

	pub fn set_motor_speed(&self, speed: Float32) {
		unsafe { b2WheelJoint_SetMotorSpeed(self.ptr, speed) }
	}

	pub fn get_motor_speed(&self) -> Float32 {
		unsafe { b2WheelJoint_GetMotorSpeed(self.ptr) }
	}

	pub fn set_max_motor_torque(&self, torque: Float32) {
		unsafe { b2WheelJoint_SetMaxMotorTorque(self.ptr, torque) }
	}

	pub fn get_max_motor_torque(&self) -> Float32 {
		unsafe { b2WheelJoint_GetMaxMotorTorque(self.ptr) }
	}

	pub fn get_motor_torque(&self, inv_dt: Float32) -> Float32 {
		unsafe { b2WheelJoint_GetMotorTorque(self.ptr, inv_dt) }
	}

	pub fn set_spring_frequency_hz(&self, hz: Float32) {
		unsafe { b2WheelJoint_SetSpringFrequencyHz(self.ptr, hz) }
	}

	pub fn get_spring_frequency_hz(&self) -> Float32 {
		unsafe { b2WheelJoint_GetSpringFrequencyHz(self.ptr) }
	}

	pub fn set_spring_damping_ratio(&self, ratio: Float32) {
		unsafe { b2WheelJoint_SetSpringDampingRatio(self.ptr, ratio) }
	}

	pub fn get_spring_damping_ratio(&self) -> Float32 {
		unsafe { b2WheelJoint_GetSpringDampingRatio(self.ptr) }
	}


}

impl Joint for WheelJoint {
	fn get_handle(&self) -> *mut B2Joint {
		self.ptr as *mut B2Joint
	}

	fn get_next(&self) -> Self
	{
		unsafe {
			WheelJoint { ptr: b2Joint_GetNext(self.get_handle()) as *mut _ }
		}
	}
}
