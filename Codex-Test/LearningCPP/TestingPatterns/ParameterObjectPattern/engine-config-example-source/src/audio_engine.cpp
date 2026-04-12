#include "engine/audio_engine.hpp"

#include <algorithm>
#include <iostream>
#include <stdexcept>

namespace demo
{
    AudioEngine::AudioEngine(config::audio::Startup config)
        : masterVolume_(std::clamp(config.masterVolume, 0.0f, 1.0f))
    {
        std::cout << "[audio] create masterVolume=" << masterVolume_ << '\n';
    }

    AudioEngine AudioEngine::Create(config::audio::Startup config)
    {
        return AudioEngine(std::move(config));
    }

    PlaybackId AudioEngine::Play(const SoundId& soundId, config::audio::Playback config)
    {
        if (soundId.value.empty())
        {
            throw std::invalid_argument("AudioEngine::Play requires a valid SoundId.");
        }

        const float effectiveVolume = std::clamp(config.volume, 0.0f, 1.0f) * masterVolume_;
        const PlaybackId playbackId = nextPlaybackId_++;
        activeSounds_.insert_or_assign(playbackId, soundId);

        std::cout
            << "[audio] play id=" << playbackId
            << " sound=\"" << soundId.value << "\""
            << " loop=" << (config.loop ? "true" : "false")
            << " volume=" << effectiveVolume
            << '\n';

        return playbackId;
    }

    void AudioEngine::StopAll()
    {
        std::cout << "[audio] stopAll activeSounds=" << activeSounds_.size() << '\n';
        activeSounds_.clear();
    }
}
