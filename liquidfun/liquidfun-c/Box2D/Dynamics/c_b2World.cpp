#include "c_b2World.h"

extern "C" {

    b2World* b2World_New(const b2Vec2* gravity) {
        return new b2World(*gravity);
    }

    void b2World_Delete(b2World* self) {
        delete self;
    }

	void b2World_SetContactListener(b2World* self, b2ContactListener* listener) {
        self->SetContactListener(listener);
    }

    const b2Body* b2World_GetBodyList(b2World* self) {
        return self->GetBodyList();
    }

    b2ParticleSystem* b2World_GetParticleSystemList(b2World* self) {
        return self->GetParticleSystemList();
    }

    b2Body* b2World_CreateBody(b2World* self, const b2BodyDef* bd) {
    	return self->CreateBody(bd);
    }

    void b2World_DestroyBody(b2World* self, b2Body* body) {
        self->DestroyBody(body);
    }

    b2ParticleSystem* b2World_CreateParticleSystem(b2World* self, const b2ParticleSystemDef* def) {
        return self->CreateParticleSystem(def);
    }

    void b2World_Step(b2World* self, float32 timeStep, int32 velocityIterations, int32 positionIterations) {
        self->Step(timeStep, velocityIterations, positionIterations);
    }

    void b2World_RayCast(b2World* self, b2RayCastCallback* callback, const b2Vec2* point1, const b2Vec2* point2) {
        self->RayCast(callback, *point1, *point2);
    }

    void b2World_StepParticle(b2World* self, float32 timeStep, int32 velocityIterations, int32 positionIterations, int32 particleIterations) {
        self->Step(timeStep, velocityIterations, positionIterations, particleIterations);
    }

    int32 b2World_CalculateReasonableParticleIterations(b2World* self, float32 timeStep) {
        return self->CalculateReasonableParticleIterations(timeStep);
    }

    int32 b2World_GetBodyCount(b2World* self) {
        return self->GetBodyCount();
    }

    int32 b2World_GetJointCount(b2World* self) {
        return self->GetJointCount();
    }

    void b2World_SetGravity(b2World* self, const b2Vec2* gravity) {
        self->SetGravity(*gravity);
    }

    c_b2Vec2 b2World_GetGravity(b2World* self) {
    	b2Vec2 tmp = self->GetGravity();
        return *cast(&tmp);
    }


} // extern C
