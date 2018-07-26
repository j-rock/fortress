#include "c_b2MouseJoint.h"

extern "C" {

    b2MouseJoint* b2MouseJoint_Create(
		b2World* world,
		void* userData,
		b2Body* bodyA,
	    b2Body* bodyB,
	    bool collideConnected,
        b2Vec2 target,
		float32 maxForce,
		float32 frequencyHz,
		float32 dampingRatio
	) {
        b2MouseJointDef def;
        def.target = target;
		def.maxForce = maxForce;
		def.frequencyHz = frequencyHz;
		def.dampingRatio = dampingRatio;

        return static_cast<b2MouseJoint*>(world->CreateJoint(&def));
    }

    void b2MouseJoint_SetTarget(b2MouseJoint* self, const b2Vec2& target) {
        self->SetTarget(target);
    }

	const b2Vec2* b2MouseJoint_GetTarget(b2MouseJoint* self) {
        return &self->GetTarget();
    }

	void b2MouseJoint_SetMaxForce(b2MouseJoint* self, float32 force) {
        self->SetMaxForce(force);
    }

	float32 b2MouseJoint_GetMaxForce(b2MouseJoint* self) {
        return self->GetMaxForce();
    }

	void b2MouseJoint_SetFrequency(b2MouseJoint* self, float32 hz) {
        self->SetFrequency(hz);
    }

	float32 b2MouseJoint_GetFrequency(b2MouseJoint* self) {
        return self->GetFrequency();
    }

	void b2MouseJoint_SetDampingRatio(b2MouseJoint* self, float32 ratio) {
        self->SetDampingRatio(ratio);
    }

	float32 b2MouseJoint_GetDampingRatio(b2MouseJoint* self) {
        return self->GetDampingRatio();
    }


} // extern C
