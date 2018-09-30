#ifndef C_B2_CIRCLESHAPE
#define C_B2_CIRCLESHAPE

#ifdef __cplusplus
extern "C" {
#endif

    b2CircleShape* b2CircleShape_New(const b2Vec2* position, float radius);
	void b2CircleShape_Delete(b2CircleShape* self);
	b2Shape* b2CircleShape_Upcast(b2CircleShape* self);

#ifdef __cplusplus
} // extern C
#endif
#endif
