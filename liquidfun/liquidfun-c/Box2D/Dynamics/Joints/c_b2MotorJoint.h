#ifndef C_B2_MOTOR_JOINT
#define C_B2_MOTOR_JOINT

#ifdef __cplusplus
extern "C" {
#endif

	b2MotorJoint* b2MotorJoint_Create(
		b2World* world,
		void* userData,
		b2Body* bodyA,
	    b2Body* bodyB,
	    bool collideConnected,
		b2Vec2 linearOffset,
		float32 angularOffset,
		float32 maxForce,
		float32 maxTorque,
		float32 correctionFactor
	);

	void b2MotorJoint_SetLinearOffset(b2MotorJoint* self, const b2Vec2& linearOffset);
	const b2Vec2* b2MotorJoint_GetLinearOffset(b2MotorJoint* self);
	void b2MotorJoint_SetAngularOffset(b2MotorJoint* self, float32 angularOffset);
	float32 b2MotorJoint_GetAngularOffset(b2MotorJoint* self);
	void b2MotorJoint_SetMaxForce(b2MotorJoint* self, float32 force);
	float32 b2MotorJoint_GetMaxForce(b2MotorJoint* self);
	void b2MotorJoint_SetMaxTorque(b2MotorJoint* self, float32 torque);
	float32 b2MotorJoint_GetMaxTorque(b2MotorJoint* self);
	void b2MotorJoint_SetCorrectionFactor(b2MotorJoint* self, float32 factor);
	float32 b2MotorJoint_GetCorrectionFactor(b2MotorJoint* self);

#ifdef __cplusplus
} // extern C
#endif
#endif
