#ifndef C_B2_FRICTION_JOINT
#define C_B2_FRICTION_JOINT

#ifdef __cplusplus
extern "C" {
#endif

	b2FrictionJoint* b2FrictionJoint_Create(
		b2World* world,
		void* userData,
		b2Body* bodyA,
	    b2Body* bodyB,
	    bool collideConnected,
		b2Vec2 localAnchorA,
		b2Vec2 localAnchorB,
		float32 maxForce,
		float32 maxTorque
	);

	const b2Vec2* b2FrictionJoint_GetLocalAnchorA(b2FrictionJoint* self);
	const b2Vec2* b2FrictionJoint_GetLocalAnchorB(b2FrictionJoint* self);
	void b2FrictionJoint_SetMaxForce(b2FrictionJoint* self, float32 force);
	float32 b2FrictionJoint_GetMaxForce(b2FrictionJoint* self);
	void b2FrictionJoint_SetMaxTorque(b2FrictionJoint* self, float32 torque);
	float32 b2FrictionJoint_GetMaxTorque(b2FrictionJoint* self);

#ifdef __cplusplus
} // extern C
#endif
#endif
