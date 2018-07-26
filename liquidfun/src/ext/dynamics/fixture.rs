use super::super::super::box2d::dynamics::fixture::*;
use super::super::super::box2d::dynamics::body::*;

pub struct FixtureIterator {
	fixture: Option<Fixture>
}

impl Body {
	pub fn get_fixture_iterator(&mut self) -> FixtureIterator {
		FixtureIterator { fixture: self.get_fixture_list() }
	}
}

impl Iterator for FixtureIterator {
	type Item = Fixture;
	fn next(&mut self) -> Option<Fixture> {
		let fixture = self.fixture.clone();

		self.fixture = match self.fixture {
			Some(ref fixture) => fixture.get_next(),
			None => None,
		};

		fixture
	}
}
