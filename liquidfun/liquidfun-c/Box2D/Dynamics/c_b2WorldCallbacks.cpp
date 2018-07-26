#include "c_b2WorldCallbacks.h"

struct c_b2RayCastCallback: public b2RayCastCallback {
    c_b2RayCastCallback() {}
    ~c_b2RayCastCallback() {}

	float32 ReportFixture(b2Fixture* fixture, const b2Vec2& point, const b2Vec2& normal, float32 fraction)
	{
        return this->reportFixture(this->cObject, fixture, point, normal, fraction);
	}
	float32 ReportParticle(const b2ParticleSystem* particleSystem, int32 index, const b2Vec2& point, const b2Vec2& normal, float32 fraction)
	{
        return this->reportParticle(this->cObject, particleSystem, index, point, normal, fraction);
	}
	bool ShouldQueryParticleSystem(const b2ParticleSystem* particleSystem)
	{
        return this->shouldQueryParticleSystem(this->cObject, particleSystem);
	}

    void* cObject;
    c_b2RayCastCallbackReportFixture reportFixture;
    c_b2RayCastCallbackReportParticle reportParticle;
	c_b2RayCastCallbackShouldQueryParticleSystem shouldQueryParticleSystem;
};

struct c_b2ContactListener: public b2ContactListener {
    c_b2ContactListener() {}
    ~c_b2ContactListener() {}

	void BeginContact(b2Contact* contact)
    {
        this->beginFixtureFixture(this->cObject, contact);
    }
	void EndContact(b2Contact* contact)
    {
        this->endFixtureFixture(this->cObject, contact);
    }
	void BeginContact(b2ParticleSystem* particleSystem, b2ParticleBodyContact* particleBodyContact)
	{
        this->beginParticleFixture(this->cObject, particleSystem, particleBodyContact);
	}
	void EndContact(b2Fixture* fixture, b2ParticleSystem* particleSystem, int32 index)
	{
        this->endParticleFixture(this->cObject, fixture, particleSystem, index);
	}
	void BeginContact(b2ParticleSystem* particleSystem, b2ParticleContact* particleContact)
	{
        this->beginParticleParticle(this->cObject, particleSystem, particleContact);
	}
	void EndContact(b2ParticleSystem* particleSystem, int32 indexA, int32 indexB)
	{
        this->endParticleParticle(this->cObject, particleSystem, indexA, indexB);
	}
	void PreSolve(b2Contact* contact, const b2Manifold* oldManifold)
	{
        this->preSolve(this->cObject, contact, oldManifold);
	}
	void PostSolve(b2Contact* contact, const b2ContactImpulse* impulse)
	{
        this->postSolve(this->cObject, contact, impulse);
	}

    void* cObject;
    c_b2ContactListenerBeginFixtureFixture beginFixtureFixture;
    c_b2ContactListenerEndFixtureFixture endFixtureFixture;
    c_b2ContactListenerBeginParticleFixture beginParticleFixture;
    c_b2ContactListenerEndParticleFixture endParticleFixture;
    c_b2ContactListenerBeginParticleParticle beginParticleParticle;
    c_b2ContactListenerEndParticleParticle endParticleParticle;
    c_b2ContactListenerPreSolve preSolve;
    c_b2ContactListenerPostSolve postSolve;
};

extern "C" {

    c_b2RayCastCallback* b2RayCastCallback_New() {
        return new c_b2RayCastCallback();
    }

    void b2RayCastCallback_Delete(c_b2RayCastCallback* self) {
        delete self;
    }

    void b2RayCastCallback_Bind(c_b2RayCastCallback* self,
								void* cObject,
								c_b2RayCastCallbackReportFixture reportFixture,
								c_b2RayCastCallbackReportParticle reportParticle,
								c_b2RayCastCallbackShouldQueryParticleSystem shouldQueryParticleSystem
    ) {
        self->cObject = cObject;
        self->reportFixture = reportFixture;
        self->reportParticle = reportParticle;
        self->shouldQueryParticleSystem = shouldQueryParticleSystem;
    }


    c_b2ContactListener* b2ContactListener_New() {
        return new c_b2ContactListener;
    }
    void b2ContactListener_Delete(c_b2ContactListener* self) {
        delete self;
    }
    void b2ContactListener_Bind(c_b2ContactListener* self,
    							void* cObject,
    							c_b2ContactListenerBeginFixtureFixture beginFixtureFixture,
    							c_b2ContactListenerEndFixtureFixture endFixtureFixture,
    							c_b2ContactListenerBeginParticleFixture beginParticleFixture,
    							c_b2ContactListenerEndParticleFixture endParticleFixture,
    							c_b2ContactListenerBeginParticleParticle beginParticleParticle,
    							c_b2ContactListenerEndParticleParticle endParticleParticle,
    							c_b2ContactListenerPreSolve preSolve,
    							c_b2ContactListenerPostSolve postSolve
    ) {
        self->cObject = cObject;
        self->beginFixtureFixture = beginFixtureFixture;
        self->endFixtureFixture = endFixtureFixture;
        self->beginParticleFixture = beginParticleFixture;
        self->endParticleFixture = endParticleFixture;
        self->beginParticleParticle = beginParticleParticle;
        self->endParticleParticle = endParticleParticle;
        self->preSolve = preSolve;
        self->postSolve = postSolve;
    }

} // extern C
