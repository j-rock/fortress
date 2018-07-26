
#ifndef C_B2_COLLISION
#define C_B2_COLLISION

#ifdef __cplusplus
extern "C" {
#endif

    bool b2AABB_IsValid(b2AABB *self);
    b2Vec2 b2AABB_GetCenter(b2AABB *self);
    b2Vec2 b2AABB_GetExtents(b2AABB *self);
    float32 b2AABB_GetPerimeter(b2AABB *self);
    void b2AABB_Combine(b2AABB *self, const b2AABB& aabb);
    void b2AABB_Combine(b2AABB *self, const b2AABB& aabb1, const b2AABB& aabb2);
    bool b2AABB_Contains(b2AABB *self, const b2AABB& aabb);
    bool b2AABB_RayCast(b2AABB *self, b2RayCastOutput* output, const b2RayCastInput& input);

#ifdef __cplusplus
} // extern C
#endif
#endif
