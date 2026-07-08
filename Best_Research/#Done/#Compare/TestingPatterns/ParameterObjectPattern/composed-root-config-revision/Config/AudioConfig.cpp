#include "AudioConfig.h"

#include <stdexcept>

namespace Config {

Volume Volume::from_percent(std::uint32_t value) {
    if (value > 100U) {
        throw std::invalid_argument("volume percent must be between 0 and 100");
    }
    return Volume{value};
}

SampleRate SampleRate::from_hz(std::uint32_t value) {
    if (value == 0U) {
        throw std::invalid_argument("sample rate must be > 0");
    }
    return SampleRate{value};
}

Audio makeDefaultAudioConfig() {
    return Audio{
        .audioBackend = AudioBackend::WASAPI,
        .masterVolume = Volume::from_percent(100),
        .musicVolume = Volume::from_percent(100),
        .sfxVolume = Volume::from_percent(100),
        .spatialAudio = SpatialAudio::Enabled,
        .outputSampleRate = SampleRate::from_hz(96000),
    };
}

} // namespace Config

const char* toString(Config::AudioBackend value) {
    switch (value) {
        case Config::AudioBackend::WASAPI: return "WASAPI";
        case Config::AudioBackend::XAudio2: return "XAudio2";
        case Config::AudioBackend::OpenAL: return "OpenAL";
    }
    return "Unknown";
}

const char* toString(Config::SpatialAudio value) {
    switch (value) {
        case Config::SpatialAudio::Disabled: return "Disabled";
        case Config::SpatialAudio::Enabled: return "Enabled";
    }
    return "Unknown";
}
