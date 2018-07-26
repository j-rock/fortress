#ifndef C_B2_DISTANCE_JOINT
#define C_B2_DISTANCE_JOINT

#ifdef __cplusplus
extern "C" {
#endif

	b2DistanceJoint* b2DistanceJoint_Create(
		b2World* world,
		void* userData,
		b2Body* bodyA,
	    b2Body* bodyB,
	    bool collideConnected,
        b2Vec2 localAnchorA,
    	b2Vec2 localAnchorB,
    	float32 length,
    	float32 frequencyHz,
    	float32 dampingRatio
	);

	const b2Vec2* b2DistanceJoint_GetLocalAnchorA(b2DistanceJoint* self);
	const b2Vec2* b2DistanceJoint_GetLocalAnchorB(b2DistanceJoint* self);
	void b2DistanceJoint_SetLength(b2DistanceJoint* self, float32 length);
	float32 b2DistanceJoint_GetLength(b2DistanceJoint* self);
	void b2DistanceJoint_SetFrequency(b2DistanceJoint* self, float32 hz);
	float32 b2DistanceJoint_GetFrequency(b2DistanceJoint* self);
	void b2DistanceJoint_SetDampingRatio(b2DistanceJoint* self, float32 ratio);
	float32 b2DistanceJoint_GetDampingRatio(b2DistanceJoint* self);

#ifdef __cplusplus
} // extern C
#endif
#endif
