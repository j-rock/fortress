#include "c_b2EdgeShape.h"

extern "C" {

    b2EdgeShape* b2EdgeShape_New() {
        return new b2EdgeShape;
    }

    void b2EdgeShape_Delete(b2EdgeShape* self) {
        delete self;
    }

    b2Shape* b2EdgeShape_Upcast(b2EdgeShape* self) {
        return static_cast<b2Shape*>(reinterpret_cast<b2EdgeShape*>(self));
    }

    void b2EdgeShape_Set(b2EdgeShape* self, const b2Vec2* v1, const b2Vec2* v2) {
        self->Set(*v1, *v2);
    }

    void b2EdgeShape_Set0(b2EdgeShape* self, const b2Vec2* v) {
        self->m_hasVertex0 = v != NULL;

        if (self->m_hasVertex0) {
            self->m_vertex0 = *v;
        }
    }

    void b2EdgeShape_Set3(b2EdgeShape* self, const b2Vec2* v) {
        self->m_hasVertex3 = v != NULL;

        if (self->m_hasVertex3) {
            self->m_vertex3 = *v;
        }
    }

} // extern C
