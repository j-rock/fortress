extern crate liquidfun;

use std::ffi::CStr;
use liquidfun::box2d::common::settings;

#[test]
fn print_info() {
    println!("MaxManifoldPoints: {:?}",  settings::MAX_MANIFOLD_POINTS);
    println!("MaxPolygonVertices: {:?}",  settings::MAX_POLYGON_VERTICES);
    println!("AabbExtension: {:?}",  settings::AABB_EXTENSION);
    println!("AabbMultiplier: {:?}",  settings::AABB_MULTIPLIER);
    println!("LinearSlop: {:?}",  settings::LINEAR_SLOP);
    println!("AngularSlop: {:?}",  settings::ANGULAR_SLOP);
    println!("PolygonRadius: {:?}",  settings::POLYGON_RADIUS);
    println!("MaxSubSteps: {:?}",  settings::MAX_SUB_STEPS);
    println!("MaxTOIContacts: {:?}",  settings::MAX_TOI_CONTACTS);
    println!("VelocityThreshold: {:?}",  settings::VELOCITY_THRESHOLD);
    println!("MaxLinearCorrection: {:?}",  settings::MAX_LINEAR_CORRECTION);
    println!("MaxAngularCorrection: {:?}",  settings::MAX_ANGULAR_CORRECTION);
    println!("MaxTranslation: {:?}",  settings::MAX_TRANSLATION);
    println!("MaxTranslationSquared: {:?}",  settings::MAX_TRANSLATION_SQUARED);
    println!("MaxRotation: {:?}",  settings::MAX_ROTATION);
    println!("MaxRotationSquared: {:?}",  settings::MAX_ROTATION_SQUARED);
    println!("Baumgarte: {:?}",  settings::BAUMGARTE);
    println!("ToiBaugarte: {:?}",  settings::TOI_BAUGARTE);
    println!("ParticleStride: {:?}",  settings::PARTICLE_STRIDE);
    println!("MinParticleWeight: {:?}",  settings::MIN_PARTICLE_WEIGHT);
    println!("MaxParticlePressure: {:?}",  settings::MAX_PARTICLE_PRESSURE);
    println!("MaxParticleForce: {:?}",  settings::MAX_PARTICLE_FORCE);
    println!("MaxTriadDistance: {:?}",  settings::MAX_TRIAD_DISTANCE);
    println!("MaxTriadDistanceSquared: {:?}",  settings::MAX_TRIAD_DISTANCE_SQUARED);
    println!("MinParticleSystemBufferCapacity: {:?}",  settings::MIN_PARTICLE_SYSTEM_BUFFER_CAPACITY);
    println!("BarrierCollisionTime: {:?}",  settings::BARRIER_COLLISION_TIME);
    println!("TimeToSleep: {:?}",  settings::TIME_TO_SLEEP);
    println!("LinearSleepTolerance: {:?}",  settings::LINEAR_SLEEP_TOLERANCE);
    println!("AngularSleepTolerance: {:?}",  settings::ANGULAR_SLEEP_TOLERANCE);
    println!("b2_version: {:?}", unsafe { settings::b2_version });
    println!("b2_liquidFunVersion: {:?}", unsafe { settings::b2_liquidFunVersion });
    println!("b2_liquidFunVersionString: {:?}", unsafe { CStr::from_ptr(settings::b2_liquidFunVersionString) });
}
