#pragma once

#include "config/physics_config.hpp"

namespace demo
{
    class PhysicsEngine final
    {
    public:
        [[nodiscard]] static PhysicsEngine Create(config::physics::Startup config);

        PhysicsEngine(const PhysicsEngine&) = delete;
        PhysicsEngine& operator=(const PhysicsEngine&) = delete;
        PhysicsEngine(PhysicsEngine&&) noexcept = default;
        PhysicsEngine& operator=(PhysicsEngine&&) noexcept = default;

        [[nodiscard]] const Vec2& Gravity() const noexcept;

    private:
        explicit PhysicsEngine(config::physics::Startup config);

        Vec2 gravity_{};
    };
}
