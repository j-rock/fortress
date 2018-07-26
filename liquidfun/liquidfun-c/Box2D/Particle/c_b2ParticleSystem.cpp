#include <Box2D/Box2D.h>
#include "c_b2ParticleSystem.h"

extern "C" {

	void b2ParticleContact_SetIndices(b2ParticleContact* self, int32 a, int32 b) {
		return self->SetIndices(a, b);
	}
	void b2ParticleContact_SetWeight(b2ParticleContact* self, float32 w) {
		return self->SetWeight(w);
	}
	void b2ParticleContact_SetNormal(b2ParticleContact* self, b2Vec2& n) {
		return self->SetNormal(n);
	}
	void b2ParticleContact_SetFlags(b2ParticleContact* self, uint32 f) {
		return self->SetFlags(f);
	}
	int32 b2ParticleContact_GetIndexA(b2ParticleContact* self) {
		return self->GetIndexA();
	}
	int32 b2ParticleContact_GetIndexB(b2ParticleContact* self) {
		return self->GetIndexB();
	}
	float32 b2ParticleContact_GetWeight(b2ParticleContact* self) {
		return self->GetWeight();
	}
	const b2Vec2* b2ParticleContact_GetNormal(b2ParticleContact* self) {
		return &self->GetNormal();
	}
	uint32 b2ParticleContact_GetFlags(b2ParticleContact* self) {
		return self->GetFlags();
	}
	bool b2ParticleContact_ApproximatelyEqual(b2ParticleContact* self, b2ParticleContact& rhs) {
		return self->ApproximatelyEqual(rhs);
	}


	int32 b2ParticleSystem_CreateParticle(b2ParticleSystem* self, const b2ParticleDef& def) {
		return self->CreateParticle(def);
	}

	void b2ParticleSystem_DestroyParticle(b2ParticleSystem* self, int32 index) {
		self->DestroyParticle(index);
	}

	int32 b2ParticleSystem_GetParticleCount(b2ParticleSystem* self) {
		return self->GetParticleCount();
	}

	b2ParticleSystem* b2ParticleSystem_GetNext(b2ParticleSystem* self) {
		return self->GetNext();
	}

	uint32 b2ParticleSystem_GetParticleFlags(b2ParticleSystem* self, const int32 index) {
		return self->GetParticleFlags(index);
	}

	b2Vec2* b2ParticleSystem_GetPositionBuffer(b2ParticleSystem* self) {
		return self->GetPositionBuffer();
	}

} // extern C
