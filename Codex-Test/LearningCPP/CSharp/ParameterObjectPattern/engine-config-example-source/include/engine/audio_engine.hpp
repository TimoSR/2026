#pragma once

#include <unordered_map>

#include "config/audio_config.hpp"
#include "engine/types.hpp"

namespace demo
{
    class AudioEngine final
    {
    public:
        [[nodiscard]] static AudioEngine Create(config::audio::Startup config);

        AudioEngine(const AudioEngine&) = delete;
        AudioEngine& operator=(const AudioEngine&) = delete;
        AudioEngine(AudioEngine&&) noexcept = default;
        AudioEngine& operator=(AudioEngine&&) noexcept = default;

        PlaybackId Play(const SoundId& soundId, config::audio::Playback config);
        void StopAll();

    private:
        explicit AudioEngine(config::audio::Startup config);

        float masterVolume_ = 1.0f;
        PlaybackId nextPlaybackId_ = 1;
        std::unordered_map<PlaybackId, SoundId> activeSounds_;
    };
}
