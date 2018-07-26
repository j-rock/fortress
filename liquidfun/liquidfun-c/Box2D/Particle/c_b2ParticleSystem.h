#ifndef C_B2_PARTICLE_SYSTEM
#define C_B2_PARTICLE_SYSTEM

#ifdef __cplusplus
extern "C" {
#endif

	void b2ParticleContact_SetIndices(b2ParticleContact* self, int32 a, int32 b);
	void b2ParticleContact_SetWeight(b2ParticleContact* self, float32 w);
	void b2ParticleContact_SetNormal(b2ParticleContact* self, b2Vec2& n);
	void b2ParticleContact_SetFlags(b2ParticleContact* self, uint32 f);
	int32 b2ParticleContact_GetIndexA(b2ParticleContact* self);
	int32 b2ParticleContact_GetIndexB(b2ParticleContact* self);
	float32 b2ParticleContact_GetWeight(b2ParticleContact* self);
	const b2Vec2* b2ParticleContact_GetNormal(b2ParticleContact* self);
	uint32 b2ParticleContact_GetFlags(b2ParticleContact* self);
	bool b2ParticleContact_ApproximatelyEqual(b2ParticleContact* self, b2ParticleContact& rhs);

	int32 b2ParticleSystem_CreateParticle(b2ParticleSystem* self, const b2ParticleDef& def);
	void b2ParticleSystem_DestroyParticle(b2ParticleSystem* self, int32 index);
	b2ParticleSystem* b2ParticleSystem_GetNext(b2ParticleSystem* self);
	int32 b2ParticleSystem_GetParticleCount(b2ParticleSystem* self);
	uint32 b2ParticleSystem_GetParticleFlags(b2ParticleSystem* self, const int32 index);
	b2Vec2* b2ParticleSystem_GetPositionBuffer(b2ParticleSystem* self);

#ifdef __cplusplus
} // extern C
#endif
#endif
