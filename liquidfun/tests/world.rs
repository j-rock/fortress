extern crate liquidfun;

use liquidfun::box2d::common::math::*;
use liquidfun::box2d::dynamics::body::*;
use liquidfun::box2d::dynamics::world::*;
use liquidfun::box2d::particle::particle_system::*;

#[test]
fn get_gravity() {

	let gravity = Vec2::new(0.0, -10.0);
	let mut world = World::new(&gravity);
	assert_eq!(gravity, world.get_gravity());
}

#[test]
fn get_body_list() {

	let mut world = World::default();
	let body_def = BodyDef::default();

	assert!(world.get_body_list().is_none());

	world.create_body(&body_def);
	world.create_body(&body_def);

	let mut body = world.get_body_list();

	assert_eq!(body.is_some(), true);

	let mut body_count = 0;
	
	while let Some(x) = body {
    	body_count += 1;
    	body = x.get_next();
	}

	assert_eq!(body_count, 2);
}

#[test]
fn get_body_iterator() {

	let mut world = World::default();
	let body_def = BodyDef::default();

	let mut it = world.get_body_iterator();

	assert!(it.next().is_none());

	world.create_body(&body_def);

	it = world.get_body_iterator();
	assert!(it.next().is_some());
	assert!(it.next().is_none());

	world.create_body(&body_def);

	it = world.get_body_iterator();
	assert!(it.next().is_some());
	assert!(it.next().is_some());
	assert!(it.next().is_none());

	world.create_body(&body_def);

	let mut body_count = 0;
	for _ in world.get_body_iterator() {
		body_count += 1;
	}
	assert_eq!(body_count, 3);
	
}

#[test]
fn get_particle_system_list() {

	let world = World::default();
	let particle_system_def = ParticleSystemDef::default();
	world.create_particle_system(&particle_system_def);

	let mut ps = world.get_particle_system_list();

	assert_eq!(ps.is_some(), true);

	let mut count = 0;
	
	while let Some(x) = ps {
    	count += 1;
    	ps = x.get_next();
	}

	assert_eq!(count, 1);
}