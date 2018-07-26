#include "c_b2Contact.h"

extern "C" {

	b2Manifold* b2Contact_GetManifold(b2Contact* self) {
		return self->GetManifold();
	}

	void b2Contact_GetWorldManifold(b2Contact* self, b2WorldManifold* worldManifold) {
		return self->GetWorldManifold(worldManifold);
	}

	bool b2Contact_IsTouching(b2Contact* self) {
		return self->IsTouching();
	}

	void b2Contact_SetEnabled(b2Contact* self, bool flag) {
		return self->SetEnabled(flag);
	}

	bool b2Contact_IsEnabled(b2Contact* self) {
		return self->IsEnabled();
	}

	b2Contact* b2Contact_GetNext(b2Contact* self) {
		return self->GetNext();
	}

	b2Fixture* b2Contact_GetFixtureA(b2Contact* self) {
		return self->GetFixtureA();
	}

	int32 b2Contact_GetChildIndexA(b2Contact* self) {
		return self->GetChildIndexA();
	}

	b2Fixture* b2Contact_GetFixtureB(b2Contact* self) {
		return self->GetFixtureB();
	}

	int32 b2Contact_GetChildIndexB(b2Contact* self) {
		return self->GetChildIndexB();
	}

	void b2Contact_SetFriction(b2Contact* self, float32 friction) {
		return self->SetFriction(friction);
	}

	float32 b2Contact_GetFriction(b2Contact* self) {
		return self->GetFriction();
	}

	void b2Contact_ResetFriction(b2Contact* self) {
		return self->ResetFriction();
	}

	void b2Contact_SetRestitution(b2Contact* self, float32 restitution) {
		return self->SetRestitution(restitution);
	}

	float32 b2Contact_GetRestitution(b2Contact* self) {
		return self->GetRestitution();
	}

	void b2Contact_ResetRestitution(b2Contact* self) {
		return self->ResetRestitution();
	}

	void b2Contact_SetTangentSpeed(b2Contact* self, float32 speed) {
		return self->SetTangentSpeed(speed);
	}

	float32 b2Contact_GetTangentSpeed(b2Contact* self) {
		return self->GetTangentSpeed();
	}


} // extern C
