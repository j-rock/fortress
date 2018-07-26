#include "c_b2FrictionJoint.h"

extern "C" {

    b2FrictionJoint* b2FrictionJoint_Create(
		b2World* world,
		void* userData,
		b2Body* bodyA,
	    b2Body* bodyB,
	    bool collideConnected,
        b2Vec2 localAnchorA,
		b2Vec2 localAnchorB,
		float32 maxForce,
		float32 maxTorque
	) {
        b2FrictionJointDef def;
        def.userData = userData;
        def.bodyA = bodyA;
        def.bodyB = bodyB;
        def.collideConnected = collideConnected;
        def.localAnchorA = localAnchorA;
		def.localAnchorB = localAnchorB;
		def.maxForce = maxForce;
		def.maxTorque = maxTorque;

        return static_cast<b2FrictionJoint*>(world->CreateJoint(&def));
    }

    const b2Vec2* b2FrictionJoint_GetLocalAnchorA(b2FrictionJoint* self) {
        return &self->GetLocalAnchorA();
    }

    const b2Vec2* b2FrictionJoint_GetLocalAnchorB(b2FrictionJoint* self) {
        return &self->GetLocalAnchorB();
    }

    void b2FrictionJoint_SetMaxForce(b2FrictionJoint* self, float32 force) {
        self->SetMaxForce(force);
    }

    float32 b2FrictionJoint_GetMaxForce(b2FrictionJoint* self) {
        return self->GetMaxForce();
    }

    void b2FrictionJoint_SetMaxTorque(b2FrictionJoint* self, float32 torque) {
        self->SetMaxTorque(torque);
    }

    float32 b2FrictionJoint_GetMaxTorque(b2FrictionJoint* self) {
        return self->GetMaxTorque();
    }


} // extern C
