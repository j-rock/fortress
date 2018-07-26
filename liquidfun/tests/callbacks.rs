extern crate liquidfun;

use liquidfun::box2d::common::math::*;
use liquidfun::box2d::common::settings::*;
use liquidfun::box2d::collision::shapes::polygon_shape::*;
use liquidfun::box2d::dynamics::body::*;
use liquidfun::box2d::dynamics::fixture::*;
use liquidfun::box2d::dynamics::world::*;
use liquidfun::box2d::particle::particle_system::*;

#[test]
fn raycast_callback_simple() {
	let mut world = World::default();

	let mut shape = PolygonShape::new();
	shape.set_as_box(1.0, 1.0);

	let mut bd = BodyDef::default();

    bd.position.y = -2.0;
	let body_a = world.create_body(&bd);
	body_a.create_fixture_from_shape(&shape, 0.0);

    bd.position.y = -5.0;
	let body_b = world.create_body(&bd);
	body_b.create_fixture_from_shape(&shape, 0.0);

    world.ray_cast(&mut (|f: Fixture, p: &Vec2, n: &Vec2, frac: Float32| -> Float32 {
        println!("fixture hit {:?}", (f, p, n, frac));

        frac
    }, |ps: ParticleSystem, i: Int32, p: &Vec2, n: &Vec2, frac: Float32| -> Float32 {
        println!("particle_system hit {:?}", (ps, i, p, n, frac));

        frac
    }, |ps: ParticleSystem| -> bool {
        println!("being asked whether to query ps {:?}", ps);
        
        false
    }), &Vec2(0.5, 0.0), &Vec2(0.5, -10.0));
}
