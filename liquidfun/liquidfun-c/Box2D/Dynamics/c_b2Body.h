#ifndef C_B2_BODY
#define C_B2_BODY

#ifdef __cplusplus
extern "C" {
#endif

	b2Fixture* b2Body_CreateFixture(b2Body* self, const b2FixtureDef* def);
	b2Fixture* b2Body_CreateFixture_FromShape(b2Body* self, const b2Shape* shape, float32 density);
	void b2Body_DestroyFixture(b2Body* self, b2Fixture* fixture);
	void b2Body_SetTransform(b2Body* self, const b2Vec2& position, float32 angle);
	const b2Transform* b2Body_GetTransform(b2Body* self);
	const b2Vec2* b2Body_GetPosition(const b2Body* self);
	float32 b2Body_GetAngle(const b2Body* self);
	const b2Vec2* b2Body_GetWorldCenter(b2Body* self);
	const b2Vec2* b2Body_GetLocalCenter(b2Body* self);
	void b2Body_SetLinearVelocity(b2Body* self, const b2Vec2& v);
	const b2Vec2* b2Body_GetLinearVelocity(b2Body* self);
	void b2Body_SetAngularVelocity(b2Body* self, float32 omega);
	float32 b2Body_GetAngularVelocity(b2Body* self);
	void b2Body_ApplyForce(b2Body* self, const b2Vec2& force, const b2Vec2& point, bool wake);
	void b2Body_ApplyForceToCenter(b2Body* self, const b2Vec2& force, bool wake);
	void b2Body_ApplyTorque(b2Body* self, float32 torque, bool wake);
	void b2Body_ApplyLinearImpulse(b2Body* self, const b2Vec2& impulse, const b2Vec2& point, bool wake);
	void b2Body_ApplyAngularImpulse(b2Body* self, float32 impulse, bool wake);
	float32 b2Body_GetMass(b2Body* self);
	float32 b2Body_GetInertia(b2Body* self);
	void b2Body_GetMassData(b2Body* self, b2MassData* data);
	void b2Body_SetMassData(b2Body* self, const b2MassData* data);
	void b2Body_ResetMassData(b2Body* self);
	c_b2Vec2 b2Body_GetWorldPoint(b2Body* self, const b2Vec2& localPoint);
	c_b2Vec2 b2Body_GetWorldVector(b2Body* self, const b2Vec2& localVector);
	c_b2Vec2 b2Body_GetLocalPoint(const b2Body* self, const b2Vec2& worldPoint);
	c_b2Vec2 b2Body_GetLocalVector(b2Body* self, const b2Vec2& worldVector);
	c_b2Vec2 b2Body_GetLinearVelocityFromWorldPoint(b2Body* self, const b2Vec2& worldPoint);
	c_b2Vec2 b2Body_GetLinearVelocityFromLocalPoint(b2Body* self, const b2Vec2& localPoint);
	float32 b2Body_GetLinearDamping(b2Body* self);
	void b2Body_SetLinearDamping(b2Body* self, float32 linearDamping);
	float32 b2Body_GetAngularDamping(b2Body* self);
	void b2Body_SetAngularDamping(b2Body* self, float32 angularDamping);
	float32 b2Body_GetGravityScale(b2Body* self);
	void b2Body_SetGravityScale(b2Body* self, float32 scale);
	void b2Body_SetType(b2Body* self, b2BodyType type);
	b2BodyType b2Body_GetType(b2Body* self);
	void b2Body_SetBullet(b2Body* self, bool flag);
	bool b2Body_IsBullet(b2Body* self);
	void b2Body_SetSleepingAllowed(b2Body* self, bool flag);
	bool b2Body_IsSleepingAllowed(b2Body* self);
	void b2Body_SetAwake(b2Body* self, bool flag);
	bool b2Body_IsAwake(b2Body* self);
	void b2Body_SetActive(b2Body* self, bool flag);
	bool b2Body_IsActive(b2Body* self);
	void b2Body_SetFixedRotation(b2Body* self, bool flag);
	bool b2Body_IsFixedRotation(b2Body* self);
	b2Fixture* b2Body_GetFixtureList(b2Body* self);
	b2JointEdge* b2Body_GetJointList(b2Body* self);
	b2ContactEdge* b2Body_GetContactList(b2Body* self);
	b2Body* b2Body_GetNext(b2Body* self);
	void* b2Body_GetUserData(const b2Body* self);
	void b2Body_SetUserData(b2Body* self, void* data);
	b2World* b2Body_GetWorld(b2Body* self);

#ifdef __cplusplus
} // extern C
#endif
#endif
