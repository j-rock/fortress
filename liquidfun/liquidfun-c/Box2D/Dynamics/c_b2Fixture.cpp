#include <Box2D/Box2D.h>
#include "c_b2Fixture.h"

extern "C" {

	b2Fixture* b2Fixture_GetNext(b2Fixture* self) {
		return self->GetNext();
	}

	b2Shape* b2Fixture_GetShape(b2Fixture* self) {
		return self->GetShape();
	}

	b2Body* b2Fixture_GetBody(b2Fixture* self) {
	    return self->GetBody();
	}

	b2Shape::Type b2Fixture_GetType(b2Fixture* self) {
		return self->GetType();
	}

} // extern C

