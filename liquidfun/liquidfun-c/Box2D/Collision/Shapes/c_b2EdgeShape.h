#ifndef C_B2_EDGESHAPE
#define C_B2_EDGESHAPE

#ifdef __cplusplus
extern "C" {
#endif

    b2EdgeShape* b2EdgeShape_New();
	void b2EdgeShape_Delete(b2EdgeShape* self);
	b2Shape* b2EdgeShape_Upcast(b2EdgeShape* self);
    void b2EdgeShape_Set(b2EdgeShape* self, const b2Vec2* v1, const b2Vec2* v2);
    void b2EdgeShape_Set0(b2EdgeShape* self, const b2Vec2* v); // a null pointer disables the vertex
    void b2EdgeShape_Set3(b2EdgeShape* self, const b2Vec2* v); // a null pointer disables the vertex

#ifdef __cplusplus
} // extern C
#endif
#endif
