extern crate liquidfun;

use liquidfun::box2d::common::math::*;
use liquidfun::box2d::dynamics::body::*;
use liquidfun::box2d::dynamics::world::*;

#[test]
fn body_user_data() {

	let mut world = World::default();
	let mut body_def = BodyDef::default();

	let mut user_data = Vec2::new(6.0, 66.0);

	body_def.user_data = std::mem::transmute(&mut user_data as *mut _);

	let body = world.create_body(&body_def);

	let body_user_data: &mut Vec2 = std::mem::transmute(body.get_user_data());
	unsafe {
		println!("{:?} == {:?}", user_data, body_user_data);
	}

	assert_eq!(&user_data, body_user_data);
}
