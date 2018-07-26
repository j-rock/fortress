#include "c_b2DistanceJoint.h"

extern "C" {

    b2DistanceJoint* b2DistanceJoint_Create(
		b2World* world,
		void* userData,
		b2Body* bodyA,
	    b2Body* bodyB,
	    bool collideConnected,
        b2Vec2 localAnchorA,
    	b2Vec2 localAnchorB,
    	float32 length,
    	float32 frequencyHz,
    	float32 dampingRatio
	) {
        b2DistanceJointDef def;
        def.userData = userData;
        def.bodyA = bodyA;
        def.bodyB = bodyB;
        def.collideConnected = collideConnected;
        def.localAnchorA = localAnchorA;
    	def.localAnchorB = localAnchorB;
    	def.length = length;
    	def.frequencyHz = frequencyHz;
    	def.dampingRatio = dampingRatio;

        return static_cast<b2DistanceJoint*>(world->CreateJoint(&def));
    }

	const b2Vec2* b2DistanceJoint_GetLocalAnchorA(b2DistanceJoint* self) {
        return &self->GetLocalAnchorA();
    }

	const b2Vec2* b2DistanceJoint_GetLocalAnchorB(b2DistanceJoint* self) {
        return &self->GetLocalAnchorB();
    }

	void b2DistanceJoint_SetLength(b2DistanceJoint* self, float32 length) {
        self->SetLength(length);
    }

	float32 b2DistanceJoint_GetLength(b2DistanceJoint* self) {
        return self->GetLength();
    }

	void b2DistanceJoint_SetFrequency(b2DistanceJoint* self, float32 hz) {
        self->SetFrequency(hz);
    }

	float32 b2DistanceJoint_GetFrequency(b2DistanceJoint* self) {
        return self->GetFrequency();
    }

	void b2DistanceJoint_SetDampingRatio(b2DistanceJoint* self, float32 ratio) {
        return self->SetDampingRatio(ratio);
    }

	float32 b2DistanceJoint_GetDampingRatio(b2DistanceJoint* self) {
        return self->GetDampingRatio();
    }


} // extern C
