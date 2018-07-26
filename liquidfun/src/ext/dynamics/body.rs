use super::super::super::box2d::dynamics::body::*;
use super::super::super::box2d::dynamics::world::*;

pub struct BodyIterator {
	body: Option<Body>
}

impl World {
	/// Get a world body iterator.
	pub fn get_body_iterator(&mut self) -> BodyIterator {
		BodyIterator { body: self.get_body_list() }
	}
}

impl Iterator for BodyIterator {
	type Item = Body;
	fn next(&mut self) -> Option<Body> {
		let body = self.body.clone();

		self.body = match self.body {
			Some(ref x) => x.get_next(),
			None => None,
		};

		body
	}
}
