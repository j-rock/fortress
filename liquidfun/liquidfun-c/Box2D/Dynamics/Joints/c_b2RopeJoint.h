#ifndef C_B2_ROPE_JOINT
#define C_B2_ROPE_JOINT

#ifdef __cplusplus
extern "C" {
#endif

    b2RopeJoint* b2RopeJoint_Create(
        b2World* world,
        void* userData,
        b2Body* bodyA,
        b2Body* bodyB,
        bool collideConnected,
    	b2Vec2 localAnchorA,
    	b2Vec2 localAnchorB,
    	float32 maxLength
    );

	const b2Vec2* b2RopeJoint_GetLocalAnchorA(b2RopeJoint* self);
	const b2Vec2* b2RopeJoint_GetLocalAnchorB(b2RopeJoint* self);
	void b2RopeJoint_SetMaxLength(b2RopeJoint* self, float32 length);
	float32 b2RopeJoint_GetMaxLength(b2RopeJoint* self);
	b2LimitState b2RopeJoint_GetLimitState(b2RopeJoint* self);

#ifdef __cplusplus
} // extern C
#endif
#endif
