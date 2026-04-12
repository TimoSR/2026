#include "engine/physics_engine.hpp"

#include <iostream>

#include "engine/types.hpp"

namespace demo
{
    PhysicsEngine::PhysicsEngine(config::physics::Startup config)
        : gravity_(config.gravity)
    {
        std::cout << "[physics] create gravity=" << ToString(gravity_) << '\n';
    }

    PhysicsEngine PhysicsEngine::Create(config::physics::Startup config)
    {
        return PhysicsEngine(std::move(config));
    }

    const Vec2& PhysicsEngine::Gravity() const noexcept
    {
        return gravity_;
    }
}
