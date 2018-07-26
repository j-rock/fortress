#ifndef C_B2_PULLEY_JOINT
#define C_B2_PULLEY_JOINT

#ifdef __cplusplus
extern "C" {
#endif

    b2PulleyJoint* b2PulleyJoint_Create(
        b2World* world,
        void* userData,
        b2Body* bodyA,
        b2Body* bodyB,
        bool collideConnected,
    	b2Vec2 groundAnchorA,
    	b2Vec2 groundAnchorB,
    	b2Vec2 localAnchorA,
    	b2Vec2 localAnchorB,
    	float32 lengthA,
    	float32 lengthB,
    	float32 ratio
    );

	c_b2Vec2 b2PulleyJoint_GetGroundAnchorA(b2PulleyJoint* self);
	c_b2Vec2 b2PulleyJoint_GetGroundAnchorB(b2PulleyJoint* self);
	float32 b2PulleyJoint_GetLengthA(b2PulleyJoint* self);
	float32 b2PulleyJoint_GetLengthB(b2PulleyJoint* self);
	float32 b2PulleyJoint_GetRatio(b2PulleyJoint* self);
	float32 b2PulleyJoint_GetCurrentLengthA(b2PulleyJoint* self);
	float32 b2PulleyJoint_GetCurrentLengthB(b2PulleyJoint* self);

#ifdef __cplusplus
} // extern C
#endif
#endif
