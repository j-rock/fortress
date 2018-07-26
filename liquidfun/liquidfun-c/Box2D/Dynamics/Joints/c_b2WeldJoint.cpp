#include "c_b2Joint.h"

extern "C" {

    b2WeldJoint* b2WeldJoint_Create(
        b2World* world,
        void* userData,
        b2Body* bodyA,
        b2Body* bodyB,
        bool collideConnected,
    	b2Vec2 localAnchorA,
    	b2Vec2 localAnchorB,
    	float32 referenceAngle,
    	float32 frequencyHz,
    	float32 dampingRatio
    ) {
        b2WeldJointDef def;
        def.userData = userData;
        def.bodyA = bodyA;
        def.bodyB = bodyB;
        def.collideConnected = collideConnected;
        def.localAnchorA = localAnchorA;
    	def.localAnchorB = localAnchorB;
    	def.referenceAngle = referenceAngle;
    	def.frequencyHz = frequencyHz;
    	def.dampingRatio = dampingRatio;

        return static_cast<b2WeldJoint*>(world->CreateJoint(&def));
    }

	const b2Vec2* b2WeldJoint_GetLocalAnchorA(b2WeldJoint* self) {
        return &self->GetLocalAnchorA();
    }

	const b2Vec2* b2WeldJoint_GetLocalAnchorB(b2WeldJoint* self) {
        return &self->GetLocalAnchorB();
    }

	float32 b2WeldJoint_GetReferenceAngle(b2WeldJoint* self) {
        return self->GetReferenceAngle();
    }

	void b2WeldJoint_SetFrequency(b2WeldJoint* self, float32 hz) {
        self->SetFrequency(hz);
    }

	float32 b2WeldJoint_GetFrequency(b2WeldJoint* self) {
        return self->GetFrequency();
    }

	void b2WeldJoint_SetDampingRatio(b2WeldJoint* self, float32 ratio) {
        self->SetDampingRatio(ratio);
    }

	float32 b2WeldJoint_GetDampingRatio(b2WeldJoint* self) {
        return self->GetDampingRatio();
    }


} // extern C
