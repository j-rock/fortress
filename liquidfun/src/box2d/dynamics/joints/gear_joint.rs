use libc::c_void;
use std::ptr;

use super::super::super::dynamics::world::{B2World, World};
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
pub struct GearJointDef<J1, J2>
    where J1: Joint, J2: Joint
{

	/// Use this to attach application specific data to your joints.
	pub user_data: *mut c_void,

	/// The first attached body.
	pub body_a: Option<Body>,

	/// The second attached body.
	pub body_b: Option<Body>,

	/// Set this flag to true if the attached bodies should collide.
	pub collide_connected: bool,

    /// The first revolute/prismatic joint attached to the gear joint.
	pub joint1: Option<J1>,

	/// The second revolute/prismatic joint attached to the gear joint.
	pub joint2: Option<J2>,

	/// The gear ratio.
	/// @see b2GearJoint for explanation.
	pub ratio: Float32,
}

impl<J1, J2> Default for GearJointDef<J1, J2>
    where J1: Joint, J2: Joint
{
	fn default() -> Self {
		GearJointDef {
			user_data: ptr::null_mut(),
			body_a: None,
			body_b: None,
			collide_connected: false,
        	joint1: None,
        	joint2: None,
        	ratio: 0.0,
	    }
	}
}

impl<J1, J2> JointDef<GearJoint> for GearJointDef<J1, J2>
    where J1: Joint, J2: Joint
{
	fn joint_type() -> JointType { JointType::GearJoint }

	fn create(&self, world: &mut World) -> GearJoint {
		unsafe { GearJoint { ptr: b2GearJoint_Create(
			world.ptr,
			self.user_data,
			if let Some(ref p) = self.body_a { p.ptr } else { ptr::null_mut() },
		    if let Some(ref p) = self.body_b { p.ptr } else { ptr::null_mut() },
		    self.collide_connected,
            if let Some(ref p) = self.joint1 { p.get_handle() } else { ptr::null_mut() },
        	if let Some(ref p) = self.joint2 { p.get_handle() } else { ptr::null_mut() },
        	self.ratio,
		) } }
	}
}

pub enum B2GearJoint {}

extern {
    fn b2GearJoint_Create(
		world: *mut B2World,
		userData: *mut c_void,
		bodyA: *mut B2Body,
	    bodyB: *mut B2Body,
	    collideConnected: bool,
        joint1: *mut B2Joint,
		joint2: *mut B2Joint,
		ratio: Float32
	) -> *mut B2GearJoint;

    fn b2GearJoint_GetJoint1(this: *const B2GearJoint) -> *mut B2Joint;
	fn b2GearJoint_GetJoint2(this: *const B2GearJoint) -> *mut B2Joint;
	fn b2GearJoint_SetRatio(this: *const B2GearJoint, ratio: Float32);
	fn b2GearJoint_GetRatio(this: *const B2GearJoint) -> Float32;
}

#[derive(Clone, Debug)]
pub struct GearJoint {
	pub ptr: *mut B2GearJoint
}

impl GearJoint {
    pub fn get_joint1(&self) -> *mut B2Joint {
        unsafe { b2GearJoint_GetJoint1(self.ptr) }
    }

	pub fn get_joint2(&self) -> *mut B2Joint {
        unsafe { b2GearJoint_GetJoint2(self.ptr) }
    }

	pub fn set_ratio(&self, ratio: Float32) {
        unsafe { b2GearJoint_SetRatio(self.ptr, ratio) }
    }

	pub fn get_ratio(&self) -> Float32 {
        unsafe { b2GearJoint_GetRatio(self.ptr) }
    }
}

impl Joint for GearJoint {
	fn get_handle(&self) -> *mut B2Joint {
		self.ptr as *mut B2Joint
	}

	fn get_next(&self) -> Self
	{
		unsafe {
			GearJoint { ptr: b2Joint_GetNext(self.get_handle()) as *mut _ }
		}
	}
}
