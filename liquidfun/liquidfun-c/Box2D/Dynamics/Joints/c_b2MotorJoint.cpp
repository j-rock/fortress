#include "c_b2MotorJoint.h"

extern "C" {

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
	) {
        b2MotorJointDef def;
        def.linearOffset = linearOffset;
		def.angularOffset = angularOffset;
		def.maxForce = maxForce;
		def.maxTorque = maxTorque;
		def.correctionFactor = correctionFactor;

        return static_cast<b2MotorJoint*>(world->CreateJoint(&def));
    }

    void b2MotorJoint_SetLinearOffset(b2MotorJoint* self, const b2Vec2& linearOffset) {
        self->SetLinearOffset(linearOffset);
    }
    const b2Vec2* b2MotorJoint_GetLinearOffset(b2MotorJoint* self) {
        return &self->GetLinearOffset();
    }
    void b2MotorJoint_SetAngularOffset(b2MotorJoint* self, float32 angularOffset) {
        self->SetAngularOffset(angularOffset);
    }
    float32 b2MotorJoint_GetAngularOffset(b2MotorJoint* self) {
        return self->GetAngularOffset();
    }
    void b2MotorJoint_SetMaxForce(b2MotorJoint* self, float32 force) {
        self->SetMaxForce(force);
    }
    float32 b2MotorJoint_GetMaxForce(b2MotorJoint* self) {
        return self->GetMaxForce();
    }
    void b2MotorJoint_SetMaxTorque(b2MotorJoint* self, float32 torque) {
        self->SetMaxTorque(torque);
    }
    float32 b2MotorJoint_GetMaxTorque(b2MotorJoint* self) {
        return self->GetMaxTorque();
    }
    void b2MotorJoint_SetCorrectionFactor(b2MotorJoint* self, float32 factor) {
        self->SetCorrectionFactor(factor);
    }
    float32 b2MotorJoint_GetCorrectionFactor(b2MotorJoint* self) {
        return self->GetCorrectionFactor();
    }

} // extern C
