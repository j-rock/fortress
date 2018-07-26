#include "c_b2Joint.h"

extern "C" {

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
    ) {
        b2WheelJointDef def;
        def.userData = userData;
        def.bodyA = bodyA;
        def.bodyB = bodyB;
        def.collideConnected = collideConnected;
        def.localAnchorA = localAnchorA;
    	def.localAnchorB = localAnchorB;
    	def.localAxisA = localAxisA;
    	def.enableMotor = enableMotor;
    	def.maxMotorTorque = maxMotorTorque;
    	def.motorSpeed = motorSpeed;
    	def.frequencyHz = frequencyHz;
    	def.dampingRatio = dampingRatio;

        return static_cast<b2WheelJoint*>(world->CreateJoint(&def));
    }

	const b2Vec2* b2WheelJoint_GetLocalAnchorA(b2WheelJoint* self) {
        return &self->GetLocalAnchorA();
    }

	const b2Vec2* b2WheelJoint_GetLocalAnchorB(b2WheelJoint* self) {
        return &self->GetLocalAnchorB();
    }

	const b2Vec2* b2WheelJoint_GetLocalAxisA(b2WheelJoint* self) {
        return &self->GetLocalAxisA();
    }

	float32 b2WheelJoint_GetJointTranslation(b2WheelJoint* self) {
        return self->GetJointTranslation();
    }

	float32 b2WheelJoint_GetJointSpeed(b2WheelJoint* self) {
        return self->GetJointSpeed();
    }

	bool b2WheelJoint_IsMotorEnabled(b2WheelJoint* self) {
        return self->IsMotorEnabled();
    }

	void b2WheelJoint_EnableMotor(b2WheelJoint* self, bool flag) {
        self->EnableMotor(flag);
    }

	void b2WheelJoint_SetMotorSpeed(b2WheelJoint* self, float32 speed) {
        self->SetMotorSpeed(speed);
    }

	float32 b2WheelJoint_GetMotorSpeed(b2WheelJoint* self) {
        return self->GetMotorSpeed();
    }

	void b2WheelJoint_SetMaxMotorTorque(b2WheelJoint* self, float32 torque) {
        self->SetMaxMotorTorque(torque);
    }

	float32 b2WheelJoint_GetMaxMotorTorque(b2WheelJoint* self) {
        return self->GetMaxMotorTorque();
    }

	float32 b2WheelJoint_GetMotorTorque(b2WheelJoint* self, float32 inv_dt) {
        return self->GetMotorTorque(inv_dt);
    }

	void b2WheelJoint_SetSpringFrequencyHz(b2WheelJoint* self, float32 hz) {
        self->SetSpringFrequencyHz(hz);
    }

	float32 b2WheelJoint_GetSpringFrequencyHz(b2WheelJoint* self) {
        return self->GetSpringFrequencyHz();
    }

	void b2WheelJoint_SetSpringDampingRatio(b2WheelJoint* self, float32 ratio) {
        self->SetSpringDampingRatio(ratio);
    }

	float32 b2WheelJoint_GetSpringDampingRatio(b2WheelJoint* self) {
        return self->GetSpringDampingRatio();
    }


} // extern C
