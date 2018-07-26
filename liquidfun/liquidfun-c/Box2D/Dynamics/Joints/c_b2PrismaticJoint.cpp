#include "c_b2Joint.h"

extern "C" {

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
    ) {
        b2PrismaticJointDef def;
        def.localAnchorA = localAnchorA;
    	def.localAnchorB = localAnchorB;
    	def.localAxisA = localAxisA;
    	def.referenceAngle = referenceAngle;
    	def.enableLimit = enableLimit;
    	def.lowerTranslation = lowerTranslation;
    	def.upperTranslation = upperTranslation;
    	def.enableMotor = enableMotor;
    	def.maxMotorForce = maxMotorForce;
    	def.motorSpeed = motorSpeed;

        return static_cast<b2PrismaticJoint*>(world->CreateJoint(&def));
    }

	const b2Vec2* b2PrismaticJoint_GetLocalAxisA(b2PrismaticJoint* self) {
        return &self->GetLocalAxisA();
    }

	float32 b2PrismaticJoint_GetReferenceAngle(b2PrismaticJoint* self) {
        return self->GetReferenceAngle();
    }

	float32 b2PrismaticJoint_GetJointTranslation(b2PrismaticJoint* self) {
        return self->GetJointTranslation();
    }

	float32 b2PrismaticJoint_GetJointSpeed(b2PrismaticJoint* self) {
        return self->GetJointSpeed();
    }

	bool b2PrismaticJoint_IsLimitEnabled(b2PrismaticJoint* self) {
        return self->IsLimitEnabled();
    }

	void b2PrismaticJoint_EnableLimit(b2PrismaticJoint* self, bool flag) {
        self->EnableLimit(flag);
    }

	float32 b2PrismaticJoint_GetLowerLimit(b2PrismaticJoint* self) {
        return self->GetLowerLimit();
    }

	float32 b2PrismaticJoint_GetUpperLimit(b2PrismaticJoint* self) {
        return self->GetUpperLimit();
    }

	void b2PrismaticJoint_SetLimits(b2PrismaticJoint* self, float32 lower, float32 upper) {
        self->SetLimits(lower, upper);
    }

	bool b2PrismaticJoint_IsMotorEnabled(b2PrismaticJoint* self) {
        return self->IsMotorEnabled();
    }

	void b2PrismaticJoint_EnableMotor(b2PrismaticJoint* self, bool flag) {
        self->EnableMotor(flag);
    }

	void b2PrismaticJoint_SetMotorSpeed(b2PrismaticJoint* self, float32 speed) {
        self->SetMotorSpeed(speed);
    }

	float32 b2PrismaticJoint_GetMotorSpeed(b2PrismaticJoint* self) {
        return self->GetMotorSpeed();
    }

	void b2PrismaticJoint_SetMaxMotorForce(b2PrismaticJoint* self, float32 force) {
        self->SetMaxMotorForce(force);
    }

	float32 b2PrismaticJoint_GetMaxMotorForce(b2PrismaticJoint* self) {
        return self->GetMaxMotorForce();
    }

	float32 b2PrismaticJoint_GetMotorForce(b2PrismaticJoint* self, float32 inv_dt) {
        return self->GetMotorForce(inv_dt);
    }


} // extern C
