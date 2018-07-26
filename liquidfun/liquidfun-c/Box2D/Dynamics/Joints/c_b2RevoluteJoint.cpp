#include "c_b2RevoluteJoint.h"

extern "C" {

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
	) {
		b2RevoluteJointDef def;
		def.userData = userData;
		def.bodyA = bodyA;
	    def.bodyB = bodyB;
	    def.collideConnected = collideConnected;
	    def.localAnchorA = localAnchorA;
	    def.localAnchorB = localAnchorB;
	    def.referenceAngle = referenceAngle;
	    def.enableLimit = enableLimit;
	    def.lowerAngle = lowerAngle;
	    def.upperAngle = upperAngle;
	    def.enableMotor = enableMotor;
	    def.motorSpeed = motorSpeed;
	    def.maxMotorTorque = maxMotorTorque;

		return static_cast<b2RevoluteJoint*>(world->CreateJoint(&def));
	}

	float32 b2RevoluteJoint_GetReferenceAngle(b2RevoluteJoint* self) {
		return self->GetReferenceAngle();
	}

	float32 b2RevoluteJoint_GetJointAngle(b2RevoluteJoint* self) {
		return self->GetJointAngle();
	}

	float32 b2RevoluteJoint_GetJointSpeed(b2RevoluteJoint* self) {
		return self->GetJointSpeed();
	}

	bool b2RevoluteJoint_IsLimitEnabled(b2RevoluteJoint* self) {
		return self->IsLimitEnabled();
	}

	void b2RevoluteJoint_EnableLimit(b2RevoluteJoint* self, bool flag) {
		self->EnableLimit(flag);
	}

	float32 b2RevoluteJoint_GetLowerLimit(b2RevoluteJoint* self) {
		return self->GetLowerLimit();
	}

	float32 b2RevoluteJoint_GetUpperLimit(b2RevoluteJoint* self) {
		return self->GetUpperLimit();
	}

	void b2RevoluteJoint_SetLimits(b2RevoluteJoint* self, float32 lower, float32 upper) {
		self->SetLimits(lower, upper);
	}

	bool b2RevoluteJoint_IsMotorEnabled(b2RevoluteJoint* self) {
		return self->IsMotorEnabled();
	}

	void b2RevoluteJoint_EnableMotor(b2RevoluteJoint* self, bool flag) {
		self->EnableMotor(flag);
	}

	void b2RevoluteJoint_SetMotorSpeed(b2RevoluteJoint* self, float32 speed) {
		self->SetMotorSpeed(speed);
	}

	float32 b2RevoluteJoint_GetMotorSpeed(b2RevoluteJoint* self) {
		return self->GetMotorSpeed();
	}

	void b2RevoluteJoint_SetMaxMotorTorque(b2RevoluteJoint* self, float32 torque) {
		self->SetMaxMotorTorque(torque);
	}

	float32 b2RevoluteJoint_GetMaxMotorTorque(b2RevoluteJoint* self) {
		return self->GetMaxMotorTorque();
	}

	float32 b2RevoluteJoint_GetMotorTorque(b2RevoluteJoint* self, float32 inv_dt) {
		return self->GetMotorTorque(inv_dt);
	}

} // extern C
