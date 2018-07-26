#ifndef C_B2_MOUSE_JOINT
#define C_B2_MOUSE_JOINT

#ifdef __cplusplus
extern "C" {
#endif

	b2MouseJoint* b2MouseJoint_Create(
		b2World* world,
		void* userData,
		b2Body* bodyA,
	    b2Body* bodyB,
	    bool collideConnected,
		b2Vec2 target,
		float32 maxForce,
		float32 frequencyHz,
		float32 dampingRatio
	);

	void b2MouseJoint_SetTarget(b2MouseJoint* self, const b2Vec2& target);
	const b2Vec2* b2MouseJoint_GetTarget(b2MouseJoint* self);
	void b2MouseJoint_SetMaxForce(b2MouseJoint* self, float32 force);
	float32 b2MouseJoint_GetMaxForce(b2MouseJoint* self);
	void b2MouseJoint_SetFrequency(b2MouseJoint* self, float32 hz);
	float32 b2MouseJoint_GetFrequency(b2MouseJoint* self);
	void b2MouseJoint_SetDampingRatio(b2MouseJoint* self, float32 ratio);
	float32 b2MouseJoint_GetDampingRatio(b2MouseJoint* self);

#ifdef __cplusplus
} // extern C
#endif
#endif
