#ifndef C_B2_GEAR_JOINT
#define C_B2_GEAR_JOINT

#ifdef __cplusplus
extern "C" {
#endif

	b2GearJoint* b2GearJoint_Create(
		b2World* world,
		void* userData,
		b2Body* bodyA,
	    b2Body* bodyB,
	    bool collideConnected,
		b2Joint* joint1,
		b2Joint* joint2,
		float32 ratio
	);

	b2Joint* b2GearJoint_GetJoint1(b2GearJoint* self);
	b2Joint* b2GearJoint_GetJoint2(b2GearJoint* self);
	void b2GearJoint_SetRatio(b2GearJoint* self, float32 ratio);
	float32 b2GearJoint_GetRatio(b2GearJoint* self);

#ifdef __cplusplus
} // extern C
#endif
#endif
