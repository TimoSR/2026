// public types
/// Requested renderer settings that can be changed by product UI components.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GraphicsSettings
{
    /// Enables multisample antialiasing for the scene render targets.
    pub is_multisample_antialiasing_enabled: bool,

    /// Enables temporal antialiasing after the scene has been rendered.
    pub is_temporal_antialiasing_enabled: bool,
}
// public types
