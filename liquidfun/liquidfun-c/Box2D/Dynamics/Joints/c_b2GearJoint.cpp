#include "c_b2GearJoint.h"

extern "C" {

    b2GearJoint* b2GearJoint_Create(
		b2World* world,
		void* userData,
		b2Body* bodyA,
	    b2Body* bodyB,
	    bool collideConnected,
        b2Joint* joint1,
		b2Joint* joint2,
		float32 ratio
	) {
        b2GearJointDef def;
        def.userData = userData;
        def.bodyA = bodyA;
        def.bodyB = bodyB;
        def.collideConnected = collideConnected;
        def.joint1 = joint1;
		def.joint2 = joint2;
		def.ratio = ratio;

        return static_cast<b2GearJoint*>(world->CreateJoint(&def));
    }

    b2Joint* b2GearJoint_GetJoint1(b2GearJoint* self) {
        return self->GetJoint1();
    }

	b2Joint* b2GearJoint_GetJoint2(b2GearJoint* self) {
        return self->GetJoint2();
    }

	void b2GearJoint_SetRatio(b2GearJoint* self, float32 ratio) {
        self->SetRatio(ratio);
    }

	float32 b2GearJoint_GetRatio(b2GearJoint* self) {
        return self->GetRatio();
    }


} // extern C
