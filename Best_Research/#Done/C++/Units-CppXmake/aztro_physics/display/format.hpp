template <typename Quantity, typename Unit> struct std::formatter<aztro::physics::QuantityDisplay<Quantity, Unit>, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(aztro::physics::QuantityDisplay<Quantity, Unit> display, std::format_context& context) const;
};

template <> struct std::formatter<aztro::physics::length::Length, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(aztro::physics::length::Length value, std::format_context& context) const;
};

template <> struct std::formatter<aztro::physics::time::Time, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(aztro::physics::time::Time value, std::format_context& context) const;
};

template <> struct std::formatter<aztro::physics::mass::Mass, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(aztro::physics::mass::Mass value, std::format_context& context) const;
};

template <> struct std::formatter<aztro::physics::velocity::Velocity, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(aztro::physics::velocity::Velocity value, std::format_context& context) const;
};

template <> struct std::formatter<aztro::physics::acceleration::Acceleration, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(aztro::physics::acceleration::Acceleration value, std::format_context& context) const;
};

template <> struct std::formatter<aztro::physics::force::Force, char>
{
        constexpr std::format_parse_context::iterator parse(std::format_parse_context& context)
        {
            return context.begin();
        }

        std::format_context::iterator format(aztro::physics::force::Force value, std::format_context& context) const;
};
