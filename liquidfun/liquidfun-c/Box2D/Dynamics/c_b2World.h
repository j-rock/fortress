#ifndef C_B2_WORLD
#define C_B2_WORLD

#ifdef __cplusplus
extern "C" {
#endif

	b2World* b2World_New(const b2Vec2* gravity);
	void b2World_Delete(b2World* self);
	void b2World_SetContactListener(b2World* self, b2ContactListener* listener);
	b2Body* b2World_CreateBody(b2World* self, const b2BodyDef* bd);
	const b2Body* b2World_GetBodyList(b2World* self);
	b2ParticleSystem* b2World_GetParticleSystemList(b2World* self);
	void b2World_DestroyBody(b2World* self, b2Body* body);
	b2ParticleSystem* b2World_CreateParticleSystem(b2World* self, const b2ParticleSystemDef* def);
	void b2World_StepParticle(b2World* self, float32 timeStep, int32 velocityIterations, int32 positionIterations, int32 particleIterations);
	void b2World_Step(b2World* self, float32 timeStep, int32 velocityIterations, int32 positionIterations);
	int32 b2World_CalculateReasonableParticleIterations(b2World* self, float32 timeStep);
	void b2World_RayCast(b2World* self, b2RayCastCallback* callback, const b2Vec2* point1, const b2Vec2* point2);
	int32 b2World_GetBodyCount(b2World* self);
	int32 b2World_GetJointCount(b2World* self);
	void b2World_SetGravity(b2World* self, const b2Vec2* gravity);
	c_b2Vec2 b2World_GetGravity(b2World* self);

#ifdef __cplusplus
} // extern C
#endif
#endif
