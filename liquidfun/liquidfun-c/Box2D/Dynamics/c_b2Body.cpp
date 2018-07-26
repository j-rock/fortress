#include <Box2D/Box2D.h>
#include "c_b2Body.h"

extern "C" {

	b2Fixture* b2Body_CreateFixture(b2Body* self, const b2FixtureDef* def) {
	    return self->CreateFixture(def);
	}

	b2Fixture* b2Body_CreateFixture_FromShape(b2Body* self, const b2Shape* shape, float32 density) {
	    return self->CreateFixture(shape, density);
	}

	void b2Body_DestroyFixture(b2Body* self, b2Fixture* fixture) {
		return self->DestroyFixture(fixture);
	}

	void b2Body_SetTransform(b2Body* self, const b2Vec2& position, float32 angle) {
		return self->SetTransform(position, angle);
	}

	const b2Transform* b2Body_GetTransform(b2Body* self) {
		return &self->GetTransform();
	}

	const b2Vec2* b2Body_GetPosition(const b2Body* self) {
	    return &self->GetPosition();
	}

	float32 b2Body_GetAngle(const b2Body* self) {
	    return self->GetAngle();
	}

	const b2Vec2* b2Body_GetWorldCenter(b2Body* self) {
		return &self->GetWorldCenter();
	}

	const b2Vec2* b2Body_GetLocalCenter(b2Body* self) {
		return &self->GetLocalCenter();
	}

	void b2Body_SetLinearVelocity(b2Body* self, const b2Vec2& v) {
		self->SetLinearVelocity(v);
	}

	const b2Vec2* b2Body_GetLinearVelocity(b2Body* self) {
		return &self->GetLinearVelocity();
	}

	void b2Body_SetAngularVelocity(b2Body* self, float32 omega) {
		self->SetAngularVelocity(omega);
	}

	float32 b2Body_GetAngularVelocity(b2Body* self) {
		return self->GetAngularVelocity();
	}

	void b2Body_ApplyForce(b2Body* self, const b2Vec2& force, const b2Vec2& point, bool wake) {
		self->ApplyForce(force, point, wake);
	}

	void b2Body_ApplyForceToCenter(b2Body* self, const b2Vec2& force, bool wake) {
		self->ApplyForceToCenter(force, wake);
	}

	void b2Body_ApplyTorque(b2Body* self, float32 torque, bool wake) {
		self->ApplyTorque(torque, wake);
	}

	void b2Body_ApplyLinearImpulse(b2Body* self, const b2Vec2& impulse, const b2Vec2& point, bool wake) {
		self->ApplyLinearImpulse(impulse, point, wake);
	}

	void b2Body_ApplyAngularImpulse(b2Body* self, float32 impulse, bool wake) {
		self->ApplyAngularImpulse(impulse, wake);
	}

	float32 b2Body_GetMass(b2Body* self) {
		return self->GetMass();
	}

	float32 b2Body_GetInertia(b2Body* self) {
		return self->GetInertia();
	}

	void b2Body_GetMassData(b2Body* self, b2MassData* data) {
		self->GetMassData(data);
	}

	void b2Body_SetMassData(b2Body* self, const b2MassData* data) {
		self->SetMassData(data);
	}

	void b2Body_ResetMassData(b2Body* self) {
		self->ResetMassData();
	}

	c_b2Vec2 b2Body_GetWorldPoint(b2Body* self, const b2Vec2& localPoint) {
		b2Vec2 tmp = self->GetWorldPoint(localPoint);
		return *cast(&tmp);
	}

	c_b2Vec2 b2Body_GetWorldVector(b2Body* self, const b2Vec2& localVector) {
		b2Vec2 tmp = self->GetWorldVector(localVector);
		return *cast(&tmp);
	}

	c_b2Vec2 b2Body_GetLocalPoint(const b2Body* self, const b2Vec2& worldPoint) {
		b2Vec2 tmp = self->GetLocalPoint(worldPoint);
        return *cast(&tmp);
	}

	c_b2Vec2 b2Body_GetLocalVector(b2Body* self, const b2Vec2& worldVector) {
		b2Vec2 tmp = self->GetLocalVector(worldVector);
        return *cast(&tmp);
	}

	c_b2Vec2 b2Body_GetLinearVelocityFromWorldPoint(b2Body* self, const b2Vec2& worldPoint) {
		b2Vec2 tmp = self->GetLinearVelocityFromWorldPoint(worldPoint);
        return *cast(&tmp);
	}

	c_b2Vec2 b2Body_GetLinearVelocityFromLocalPoint(b2Body* self, const b2Vec2& localPoint) {
		b2Vec2 tmp = self->GetLinearVelocityFromLocalPoint(localPoint);
        return *cast(&tmp);
	}

	float32 b2Body_GetLinearDamping(b2Body* self) {
		return self->GetLinearDamping();
	}

	void b2Body_SetLinearDamping(b2Body* self, float32 linearDamping) {
		self->SetLinearDamping(linearDamping);
	}

	float32 b2Body_GetAngularDamping(b2Body* self) {
		return self->GetAngularDamping();
	}

	void b2Body_SetAngularDamping(b2Body* self, float32 angularDamping) {
		return self->SetAngularDamping(angularDamping);
	}

	float32 b2Body_GetGravityScale(b2Body* self) {
		return self->GetGravityScale();
	}

	void b2Body_SetGravityScale(b2Body* self, float32 scale) {
		self->SetGravityScale(scale);
	}

	void b2Body_SetType(b2Body* self, b2BodyType type) {
		self->SetType(type);
	}

	b2BodyType b2Body_GetType(b2Body* self) {
		return self->GetType();
	}

	void b2Body_SetBullet(b2Body* self, bool flag) {
		self->SetBullet(flag);
	}

	bool b2Body_IsBullet(b2Body* self) {
		return self->IsBullet();
	}

	void b2Body_SetSleepingAllowed(b2Body* self, bool flag) {
		self->SetSleepingAllowed(flag);
	}
	bool b2Body_IsSleepingAllowed(b2Body* self) {
		return self->IsSleepingAllowed();
	}

	void b2Body_SetAwake(b2Body* self, bool flag) {
		self->SetAwake(flag);
	}

	bool b2Body_IsAwake(b2Body* self) {
		return self->IsAwake();
	}

	void b2Body_SetActive(b2Body* self, bool flag) {
		self->SetActive(flag);
	}

	bool b2Body_IsActive(b2Body* self) {
		return self->IsActive();
	}

	void b2Body_SetFixedRotation(b2Body* self, bool flag) {
		self->SetFixedRotation(flag);
	}

	bool b2Body_IsFixedRotation(b2Body* self) {
		return self->IsFixedRotation();
	}

	b2Fixture* b2Body_GetFixtureList(b2Body* self) {
		return self->GetFixtureList();
	}

	b2JointEdge* b2Body_GetJointList(b2Body* self) {
		return self->GetJointList();
	}

	b2ContactEdge* b2Body_GetContactList(b2Body* self) {
		return self->GetContactList();
	}

	b2Body* b2Body_GetNext(b2Body* self) {
		return self->GetNext();
	}

	void* b2Body_GetUserData(const b2Body* self) {
		return self->GetUserData();
	}

	void b2Body_SetUserData(b2Body* self, void* data) {
		return self->SetUserData(data);
	}

	b2World* b2Body_GetWorld(b2Body* self) {
		return self->GetWorld();
	}

} // extern C
