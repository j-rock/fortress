extern crate liquidfun;

use std::ffi::CStr;
use liquidfun::box2d::common::settings;

#[test]
fn print_info() {
    println!("MaxManifoldPoints: {:?}", unsafe { settings::MaxManifoldPoints });
    println!("MaxPolygonVertices: {:?}", unsafe { settings::MaxPolygonVertices });
    println!("AabbExtension: {:?}", unsafe { settings::AabbExtension });
    println!("AabbMultiplier: {:?}", unsafe { settings::AabbMultiplier });
    println!("LinearSlop: {:?}", unsafe { settings::LinearSlop });
    println!("AngularSlop: {:?}", unsafe { settings::AngularSlop });
    println!("PolygonRadius: {:?}", unsafe { settings::PolygonRadius });
    println!("MaxSubSteps: {:?}", unsafe { settings::MaxSubSteps });
    println!("MaxTOIContacts: {:?}", unsafe { settings::MaxTOIContacts });
    println!("VelocityThreshold: {:?}", unsafe { settings::VelocityThreshold });
    println!("MaxLinearCorrection: {:?}", unsafe { settings::MaxLinearCorrection });
    println!("MaxAngularCorrection: {:?}", unsafe { settings::MaxAngularCorrection });
    println!("MaxTranslation: {:?}", unsafe { settings::MaxTranslation });
    println!("MaxTranslationSquared: {:?}", unsafe { settings::MaxTranslationSquared });
    println!("MaxRotation: {:?}", unsafe { settings::MaxRotation });
    println!("MaxRotationSquared: {:?}", unsafe { settings::MaxRotationSquared });
    println!("Baumgarte: {:?}", unsafe { settings::Baumgarte });
    println!("ToiBaugarte: {:?}", unsafe { settings::ToiBaugarte });
    println!("ParticleStride: {:?}", unsafe { settings::ParticleStride });
    println!("MinParticleWeight: {:?}", unsafe { settings::MinParticleWeight });
    println!("MaxParticlePressure: {:?}", unsafe { settings::MaxParticlePressure });
    println!("MaxParticleForce: {:?}", unsafe { settings::MaxParticleForce });
    println!("MaxTriadDistance: {:?}", unsafe { settings::MaxTriadDistance });
    println!("MaxTriadDistanceSquared: {:?}", unsafe { settings::MaxTriadDistanceSquared });
    println!("MinParticleSystemBufferCapacity: {:?}", unsafe { settings::MinParticleSystemBufferCapacity });
    println!("BarrierCollisionTime: {:?}", unsafe { settings::BarrierCollisionTime });
    println!("TimeToSleep: {:?}", unsafe { settings::TimeToSleep });
    println!("LinearSleepTolerance: {:?}", unsafe { settings::LinearSleepTolerance });
    println!("AngularSleepTolerance: {:?}", unsafe { settings::AngularSleepTolerance });
    println!("b2_version: {:?}", unsafe { settings::b2_version });
    println!("b2_liquidFunVersion: {:?}", unsafe { settings::b2_liquidFunVersion });
    println!("b2_liquidFunVersionString: {:?}", unsafe { CStr::from_ptr(settings::b2_liquidFunVersionString) });
}
