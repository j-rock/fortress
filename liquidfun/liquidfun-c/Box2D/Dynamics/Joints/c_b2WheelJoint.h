#ifndef C_B2_WHEEL_JOINT
#define C_B2_WHEEL_JOINT

#ifdef __cplusplus
extern "C" {
#endif

    b2WheelJoint* b2WheelJoint_Create(
        b2World* world,
        void* userData,
        b2Body* bodyA,
        b2Body* bodyB,
        bool collideConnected,
    	b2Vec2 localAnchorA,
    	b2Vec2 localAnchorB,
    	b2Vec2 localAxisA,
    	bool enableMotor,
    	float32 maxMotorTorque,
    	float32 motorSpeed,
    	float32 frequencyHz,
    	float32 dampingRatio
    );

	const b2Vec2* b2WheelJoint_GetLocalAnchorA(b2WheelJoint* self);
	const b2Vec2* b2WheelJoint_GetLocalAnchorB(b2WheelJoint* self);
	const b2Vec2* b2WheelJoint_GetLocalAxisA(b2WheelJoint* self);
	float32 b2WheelJoint_GetJointTranslation(b2WheelJoint* self);
	float32 b2WheelJoint_GetJointSpeed(b2WheelJoint* self);
	bool b2WheelJoint_IsMotorEnabled(b2WheelJoint* self);
	void b2WheelJoint_EnableMotor(b2WheelJoint* self, bool flag);
	void b2WheelJoint_SetMotorSpeed(b2WheelJoint* self, float32 speed);
	float32 b2WheelJoint_GetMotorSpeed(b2WheelJoint* self);
	void b2WheelJoint_SetMaxMotorTorque(b2WheelJoint* self, float32 torque);
	float32 b2WheelJoint_GetMaxMotorTorque(b2WheelJoint* self);
	float32 b2WheelJoint_GetMotorTorque(b2WheelJoint* self, float32 inv_dt);
	void b2WheelJoint_SetSpringFrequencyHz(b2WheelJoint* self, float32 hz);
	float32 b2WheelJoint_GetSpringFrequencyHz(b2WheelJoint* self);
	void b2WheelJoint_SetSpringDampingRatio(b2WheelJoint* self, float32 ratio);
	float32 b2WheelJoint_GetSpringDampingRatio(b2WheelJoint* self);

#ifdef __cplusplus
} // extern C
#endif
#endif
