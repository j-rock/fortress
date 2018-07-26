#include "c_b2Joint.h"

extern "C" {

    b2RopeJoint* b2RopeJoint_Create(
        b2World* world,
        void* userData,
        b2Body* bodyA,
        b2Body* bodyB,
        bool collideConnected,
    	b2Vec2 localAnchorA,
    	b2Vec2 localAnchorB,
    	float32 maxLength
	) {
        b2RopeJointDef def;
        def.userData = userData;
        def.bodyA = bodyA;
        def.bodyB = bodyB;
        def.collideConnected = collideConnected;
        def.localAnchorA = localAnchorA;
    	def.localAnchorB = localAnchorB;
    	def.maxLength = maxLength;

        return static_cast<b2RopeJoint*>(world->CreateJoint(&def));
    }

	const b2Vec2* b2RopeJoint_GetLocalAnchorA(b2RopeJoint* self) {
        return &self->GetLocalAnchorA();
    }

	const b2Vec2* b2RopeJoint_GetLocalAnchorB(b2RopeJoint* self) {
        return &self->GetLocalAnchorB();
    }

	void b2RopeJoint_SetMaxLength(b2RopeJoint* self, float32 length) {
        self->SetMaxLength(length);
    }

	float32 b2RopeJoint_GetMaxLength(b2RopeJoint* self) {
        return self->GetMaxLength();
    }

	b2LimitState b2RopeJoint_GetLimitState(b2RopeJoint* self) {
        return self->GetLimitState();
    }


} // extern C
