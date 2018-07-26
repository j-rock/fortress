#ifndef C_B2_REVOLUTE_JOINT
#define C_B2_REVOLUTE_JOINT

#ifdef __cplusplus
extern "C" {
#endif

	b2RevoluteJoint* b2RevoluteJoint_Create(
		b2World* world,
		void* userData,
		b2Body* bodyA,
	    b2Body* bodyB,
	    bool collideConnected,
	    b2Vec2 localAnchorA,
	    b2Vec2 localAnchorB,
	    float32 referenceAngle,
	    bool enableLimit,
	    float32 lowerAngle,
	    float32 upperAngle,
	    bool enableMotor,
	    float32 motorSpeed,
	    float32 maxMotorTorque
	);

	float32 b2RevoluteJoint_GetReferenceAngle(b2RevoluteJoint* self);
	float32 b2RevoluteJoint_GetJointAngle(b2RevoluteJoint* self);
	float32 b2RevoluteJoint_GetJointSpeed(b2RevoluteJoint* self);
	bool b2RevoluteJoint_IsLimitEnabled(b2RevoluteJoint* self);
	void b2RevoluteJoint_EnableLimit(b2RevoluteJoint* self, bool flag);
	float32 b2RevoluteJoint_GetLowerLimit(b2RevoluteJoint* self);
	float32 b2RevoluteJoint_GetUpperLimit(b2RevoluteJoint* self);
	void b2RevoluteJoint_SetLimits(b2RevoluteJoint* self, float32 lower, float32 upper);
	bool b2RevoluteJoint_IsMotorEnabled(b2RevoluteJoint* self);
	void b2RevoluteJoint_EnableMotor(b2RevoluteJoint* self, bool flag);
	void b2RevoluteJoint_SetMotorSpeed(b2RevoluteJoint* self, float32 speed);
	float32 b2RevoluteJoint_GetMotorSpeed(b2RevoluteJoint* self);
	void b2RevoluteJoint_SetMaxMotorTorque(b2RevoluteJoint* self, float32 torque);
	float32 b2RevoluteJoint_GetMaxMotorTorque(b2RevoluteJoint* self);
	float32 b2RevoluteJoint_GetMotorTorque(b2RevoluteJoint* self, float32 inv_dt);

#ifdef __cplusplus
} // extern C
#endif
#endif
