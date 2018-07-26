#ifndef C_B2_PRISMATIC_JOINT
#define C_B2_PRISMATIC_JOINT

#ifdef __cplusplus
extern "C" {
#endif

    b2PrismaticJoint* b2PrismaticJoint_Create(
        b2World* world,
        void* userData,
        b2Body* bodyA,
        b2Body* bodyB,
        bool collideConnected,
    	b2Vec2 localAnchorA,
    	b2Vec2 localAnchorB,
    	b2Vec2 localAxisA,
    	float32 referenceAngle,
    	bool enableLimit,
    	float32 lowerTranslation,
    	float32 upperTranslation,
    	bool enableMotor,
    	float32 maxMotorForce,
    	float32 motorSpeed
    );

	const b2Vec2* b2PrismaticJoint_GetLocalAxisA(b2PrismaticJoint* self);
	float32 b2PrismaticJoint_GetReferenceAngle(b2PrismaticJoint* self);
	float32 b2PrismaticJoint_GetJointTranslation(b2PrismaticJoint* self);
	float32 b2PrismaticJoint_GetJointSpeed(b2PrismaticJoint* self);
	bool b2PrismaticJoint_IsLimitEnabled(b2PrismaticJoint* self);
	void b2PrismaticJoint_EnableLimit(b2PrismaticJoint* self, bool flag);
	float32 b2PrismaticJoint_GetLowerLimit(b2PrismaticJoint* self);
	float32 b2PrismaticJoint_GetUpperLimit(b2PrismaticJoint* self);
	void b2PrismaticJoint_SetLimits(b2PrismaticJoint* self, float32 lower, float32 upper);
	bool b2PrismaticJoint_IsMotorEnabled(b2PrismaticJoint* self);
	void b2PrismaticJoint_EnableMotor(b2PrismaticJoint* self, bool flag);
	void b2PrismaticJoint_SetMotorSpeed(b2PrismaticJoint* self, float32 speed);
	float32 b2PrismaticJoint_GetMotorSpeed(b2PrismaticJoint* self);
	void b2PrismaticJoint_SetMaxMotorForce(b2PrismaticJoint* self, float32 force);
	float32 b2PrismaticJoint_GetMaxMotorForce(b2PrismaticJoint* self);
	float32 b2PrismaticJoint_GetMotorForce(b2PrismaticJoint* self, float32 inv_dt);

#ifdef __cplusplus
} // extern C
#endif
#endif
