#include "c_b2Joint.h"

extern "C" {

    b2PulleyJoint* b2PulleyJoint_Create(
        b2World* world,
        void* userData,
        b2Body* bodyA,
        b2Body* bodyB,
        bool collideConnected,
    	b2Vec2 groundAnchorA,
    	b2Vec2 groundAnchorB,
    	b2Vec2 localAnchorA,
    	b2Vec2 localAnchorB,
    	float32 lengthA,
    	float32 lengthB,
    	float32 ratio
	) {
        b2PulleyJointDef def;
        def.userData = userData;
        def.bodyA = bodyA;
        def.bodyB = bodyB;
        def.collideConnected = collideConnected;
        def.groundAnchorA = groundAnchorA;
    	def.groundAnchorB = groundAnchorB;
    	def.localAnchorA = localAnchorA;
    	def.localAnchorB = localAnchorB;
    	def.lengthA = lengthA;
    	def.lengthB = lengthB;
    	def.ratio = ratio;

        return static_cast<b2PulleyJoint*>(world->CreateJoint(&def));
    }

    c_b2Vec2 b2PulleyJoint_GetGroundAnchorA(b2PulleyJoint* self) {
        b2Vec2 tmp = self->GetGroundAnchorA();
		return *cast(&tmp);
    }

	c_b2Vec2 b2PulleyJoint_GetGroundAnchorB(b2PulleyJoint* self) {
        b2Vec2 tmp = self->GetGroundAnchorB();
		return *cast(&tmp);
    }

	float32 b2PulleyJoint_GetLengthA(b2PulleyJoint* self) {
        return self->GetLengthA();
    }

	float32 b2PulleyJoint_GetLengthB(b2PulleyJoint* self) {
        return self->GetLengthB();
    }

	float32 b2PulleyJoint_GetRatio(b2PulleyJoint* self) {
        return self->GetRatio();
    }

	float32 b2PulleyJoint_GetCurrentLengthA(b2PulleyJoint* self) {
        return self->GetCurrentLengthA();
    }

	float32 b2PulleyJoint_GetCurrentLengthB(b2PulleyJoint* self) {
        return self->GetCurrentLengthB();
    }


} // extern C
