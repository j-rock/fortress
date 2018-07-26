#ifndef C_B2_WORLD_CALLBACKS
#define C_B2_WORLD_CALLBACKS

#ifdef __cplusplus
extern "C" {
#endif

struct c_b2RayCastCallback;

typedef float32 (*c_b2RayCastCallbackReportFixture)(void* cObject, b2Fixture* fixture, const b2Vec2& point, const b2Vec2& normal, float32 fraction);
typedef float32 (*c_b2RayCastCallbackReportParticle)(void* cObject, const b2ParticleSystem* particleSystem, int32 index, const b2Vec2& point, const b2Vec2& normal, float32 fraction);
typedef bool (*c_b2RayCastCallbackShouldQueryParticleSystem)(void* cObject, const b2ParticleSystem* particleSystem);

c_b2RayCastCallback* b2RayCastCallback_New();
void b2RayCastCallback_Delete(c_b2RayCastCallback* self);
void b2RayCastCallback_Bind(c_b2RayCastCallback* self,
							void* cObject,
							c_b2RayCastCallbackReportFixture reportFixture,
							c_b2RayCastCallbackReportParticle reportParticle,
							c_b2RayCastCallbackShouldQueryParticleSystem shouldQueryParticleSystem
);

struct c_b2ContactListener;

typedef void (*c_b2ContactListenerBeginFixtureFixture)(void* cObject, b2Contact* contact);
typedef void (*c_b2ContactListenerEndFixtureFixture)(void* cObject, b2Contact* contact);
typedef void (*c_b2ContactListenerBeginParticleFixture)(void* cObject, b2ParticleSystem* particleSystem, b2ParticleBodyContact* particleBodyContact);
typedef void (*c_b2ContactListenerEndParticleFixture)(void* cObject, b2Fixture* fixture, b2ParticleSystem* particleSystem, int32 index);
typedef void (*c_b2ContactListenerBeginParticleParticle)(void* cObject, b2ParticleSystem* particleSystem, b2ParticleContact* particleContact);
typedef void (*c_b2ContactListenerEndParticleParticle)(void* cObject, b2ParticleSystem* particleSystem, int32 indexA, int32 indexB);
typedef void (*c_b2ContactListenerPreSolve)(void* cObject, b2Contact* contact, const b2Manifold* oldManifold);
typedef void (*c_b2ContactListenerPostSolve)(void* cObject, b2Contact* contact, const b2ContactImpulse* impulse);

c_b2ContactListener* b2ContactListener_New();
void b2ContactListener_Delete(c_b2ContactListener* self);
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
);

#ifdef __cplusplus
} // extern C
#endif

#endif
