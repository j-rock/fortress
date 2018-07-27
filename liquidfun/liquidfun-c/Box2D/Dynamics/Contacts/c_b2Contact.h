#ifndef C_B2_CONTACT
#define C_B2_CONTACT

#ifdef __cplusplus
extern "C" {
#endif

	b2Manifold* b2Contact_GetManifold(b2Contact* self);
	void b2Contact_GetWorldManifold(b2Contact* self, b2WorldManifold* worldManifold);
	bool b2Contact_IsTouching(b2Contact* self);
	void b2Contact_SetEnabled(b2Contact* self, bool flag);
	bool b2Contact_IsEnabled(b2Contact* self);
	b2Contact* b2Contact_GetNext(b2Contact* self);
	b2Fixture* b2Contact_GetFixtureA(b2Contact* self);
	int32 b2Contact_GetChildIndexA(b2Contact* self);
	b2Fixture* b2Contact_GetFixtureB(b2Contact* self);
	int32 b2Contact_GetChildIndexB(b2Contact* self);
	void b2Contact_SetFriction(b2Contact* self, float32 friction);
	float32 b2Contact_GetFriction(b2Contact* self);
	void b2Contact_ResetFriction(b2Contact* self);
	void b2Contact_SetRestitution(b2Contact* self, float32 restitution);
	float32 b2Contact_GetRestitution(b2Contact* self);
	void b2Contact_ResetRestitution(b2Contact* self);
	void b2Contact_SetTangentSpeed(b2Contact* self, float32 speed);
	float32 b2Contact_GetTangentSpeed(b2Contact* self);

#ifdef __cplusplus
} // extern C
#endif
#endif
