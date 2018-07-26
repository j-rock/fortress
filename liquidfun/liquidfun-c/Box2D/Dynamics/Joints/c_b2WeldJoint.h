#ifndef C_B2_WELD_JOINT
#define C_B2_WELD_JOINT

#ifdef __cplusplus
extern "C" {
#endif

    b2WeldJoint* b2WeldJoint_Create(
        b2World* world,
        void* userData,
        b2Body* bodyA,
        b2Body* bodyB,
        bool collideConnected,
    	b2Vec2 localAnchorA,
    	b2Vec2 localAnchorB,
    	float32 referenceAngle,
    	float32 frequencyHz,
    	float32 dampingRatio
    );

	const b2Vec2* b2WeldJoint_GetLocalAnchorA(b2WeldJoint* self);
	const b2Vec2* b2WeldJoint_GetLocalAnchorB(b2WeldJoint* self);
	float32 b2WeldJoint_GetReferenceAngle(b2WeldJoint* self);
	void b2WeldJoint_SetFrequency(b2WeldJoint* self, float32 hz);
	float32 b2WeldJoint_GetFrequency(b2WeldJoint* self);
	void b2WeldJoint_SetDampingRatio(b2WeldJoint* self, float32 ratio);
	float32 b2WeldJoint_GetDampingRatio(b2WeldJoint* self);

#ifdef __cplusplus
} // extern C
#endif
#endif
