#include <Box2D/Box2D.h>
#include "c_b2Body.h"

extern "C" {

	b2Fixture* b2Body_CreateFixture(b2Body* self, const b2FixtureDef* def) {
	    return self->CreateFixture(def);
	}

	b2Fixture* b2Body_CreateFixture_FromShape(b2Body* self, const b2Shape* shape, float32 density) {
	    return self->CreateFixture(shape, density);
	}
	
	float32 b2Body_GetAngle(const b2Body* self) {
	    return self->GetAngle();
	}	

	b2Fixture* b2Body_GetFixtureList(b2Body* self) {
		return self->GetFixtureList();
	}

	b2Body* b2Body_GetNext(b2Body* self) {
		return self->GetNext();
	}

	const b2Vec2& b2Body_GetPosition(const b2Body* self) {
	    return self->GetPosition();
	}

	void* b2Body_GetUserData(const b2Body* self) {
		return self->GetUserData();
	}

	b2World* b2Body_GetWorld(b2Body* self) {
		return self->GetWorld();
	}

	b2Vec2 b2Body_GetLocalPoint(const b2Body* self, const b2Vec2& worldPoint) {
		return self->GetLocalPoint(worldPoint);
	}

} // extern C

