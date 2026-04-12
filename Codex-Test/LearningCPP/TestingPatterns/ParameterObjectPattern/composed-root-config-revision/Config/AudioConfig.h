#pragma once

#include <cstdint>

namespace Config {

enum class AudioBackend {
    WASAPI,
    XAudio2,
    OpenAL,
};

enum class SpatialAudio {
    Disabled,
    Enabled,
};

struct Volume {
    std::uint32_t percent {100};

    static Volume from_percent(std::uint32_t value);
};

struct SampleRate {
    std::uint32_t hz {48000};

    static SampleRate from_hz(std::uint32_t value);
};

struct Audio {
    AudioBackend audioBackend {AudioBackend::WASAPI};
    Volume masterVolume {Volume::from_percent(100)};
    Volume musicVolume {Volume::from_percent(100)};
    Volume sfxVolume {Volume::from_percent(100)};
    SpatialAudio spatialAudio {SpatialAudio::Enabled};
    SampleRate outputSampleRate {SampleRate::from_hz(48000)};
};

Audio makeDefaultAudioConfig();

} // namespace Config

const char* toString(Config::AudioBackend value);
const char* toString(Config::SpatialAudio value);
