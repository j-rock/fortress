#include "c_b2Joint.h"

extern "C" {

    b2JointType b2Joint_GetType(b2Joint* self) {
        return self->GetType();
    }

    b2Body* b2Joint_GetBodyA(b2Joint* self) {
        return self->GetBodyA();
    }

    b2Body* b2Joint_GetBodyB(b2Joint* self) {
        return self->GetBodyB();
    }

    c_b2Vec2 b2Joint_GetAnchorA(b2Joint* self) {
        b2Vec2 tmp = self->GetAnchorA();
		return *cast(&tmp);
    }

    c_b2Vec2 b2Joint_GetAnchorB(b2Joint* self) {
        b2Vec2 tmp = self->GetAnchorB();
		return *cast(&tmp);
    }

    c_b2Vec2 b2Joint_GetReactionForce(b2Joint* self, float32 inv_dt) {
        b2Vec2 tmp = self->GetReactionForce(inv_dt);
		return *cast(&tmp);
    }

    float32 b2Joint_GetReactionTorque(b2Joint* self, float32 inv_dt) {
        return self->GetReactionTorque(inv_dt);
    }

    b2Joint* b2Joint_GetNext(b2Joint* self) {
        return self->GetNext();
    }

    void* b2Joint_GetUserData(b2Joint* self) {
        return self->GetUserData();
    }

    void b2Joint_SetUserData(b2Joint* self, void* data) {
        self->SetUserData(data);
    }

    bool b2Joint_IsActive(b2Joint* self) {
        return self->IsActive();
    }

    bool b2Joint_GetCollideConnected(b2Joint* self) {
        return self->GetCollideConnected();
    }

    void b2Joint_ShiftOrigin(b2Joint* self, b2Vec2& newOrigin) {
        self->ShiftOrigin(newOrigin);
    }

} // extern C
