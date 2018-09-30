#include "c_b2CircleShape.h"

extern "C" {

    b2CircleShape* b2CircleShape_New(const b2Vec2* position, float radius) {
        b2CircleShape* shape = new b2CircleShape;
        shape->m_p = *position;
        shape->m_radius = radius;
        return shape;
    }

    void b2CircleShape_Delete(b2CircleShape* self) {
        delete self;
    }

    b2Shape* b2CircleShape_Upcast(b2CircleShape* self) {
        return static_cast<b2Shape*>(reinterpret_cast<b2CircleShape*>(self));
    }

} // extern C
