#ifndef C_B2_JOINT
#define C_B2_JOINT

#ifdef __cplusplus
extern "C" {
#endif

    b2JointType b2Joint_GetType(b2Joint* self);
    b2Body* b2Joint_GetBodyA(b2Joint* self);
    b2Body* b2Joint_GetBodyB(b2Joint* self);
    c_b2Vec2 b2Joint_GetAnchorA(b2Joint* self);
    c_b2Vec2 b2Joint_GetAnchorB(b2Joint* self);
    c_b2Vec2 b2Joint_GetReactionForce(b2Joint* self, float32 inv_dt);
    float32 b2Joint_GetReactionTorque(b2Joint* self, float32 inv_dt);
    b2Joint* b2Joint_GetNext(b2Joint* self);
    void* b2Joint_GetUserData(b2Joint* self);
    void b2Joint_SetUserData(b2Joint* self, void* data);
    bool b2Joint_IsActive(b2Joint* self);
    bool b2Joint_GetCollideConnected(b2Joint* self);
    void b2Joint_ShiftOrigin(b2Joint* self, b2Vec2& newOrigin);

#ifdef __cplusplus
} // extern C
#endif
#endif
