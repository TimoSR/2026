#include "Audio.h"

#include <iostream>
#include <stdexcept>
#include <utility>

Audio Audio::Create() {
    Audio audio(translate(Config::Audio{}));
    audio.running_ = true;
    std::cout << "Audio created with default profile.\n";
    audio.printSummary();
    return audio;
}

Audio Audio::Create(Config::Audio config) {
    Audio audio(translate(std::move(config)));
    audio.running_ = true;
    std::cout << "Audio created with custom profile.\n";
    audio.printSummary();
    return audio;
}

void Audio::UpdateConfig(Config::Audio config) {
    ensureRunning();
    config_ = translate(std::move(config));
    std::cout << "Audio configuration updated at runtime.\n";
    printSummary();
}

void Audio::PlaySound(const char* filePath) const {
    ensureRunning();
    if (filePath == nullptr || *filePath == '\0') {
        throw std::invalid_argument("sound filePath must be non-empty");
    }

    std::cout << "Playing sound stream: " << filePath
              << " [backend=" << toString(config_.audioBackend)
              << ", masterVolume=" << config_.masterVolume.percent_ << "%"
              << ", spatialAudio=" << toString(config_.spatialAudio) << "]\n";
}

void Audio::Stop() {
    ensureRunning();
    running_ = false;
    std::cout << "Audio stopped.\n";
}

void Audio::printSummary() const {
    std::cout << "Audio\n";
    std::cout << "  backend: " << toString(config_.audioBackend) << '\n';
    std::cout << "  masterVolume: " << config_.masterVolume.percent_ << "%\n";
    std::cout << "  musicVolume: " << config_.musicVolume.percent_ << "%\n";
    std::cout << "  sfxVolume: " << config_.sfxVolume.percent_ << "%\n";
    std::cout << "  spatialAudio: " << toString(config_.spatialAudio) << '\n';
    std::cout << "  outputSampleRate: " << config_.outputSampleRate.hz_ << " Hz\n";
    std::cout << "  diagnostics: " << toString(config_.diagnosticsLevel) << '\n';
}

Audio::Audio(InternalAudioConfig config)
    : config_(std::move(config)) {}

Audio::InternalAudioConfig Audio::translate(Config::Audio config) {
    validate(config);
    return InternalAudioConfig{
        .audioBackend = config.audioBackend,
        .masterVolume = config.masterVolume,
        .musicVolume = config.musicVolume,
        .sfxVolume = config.sfxVolume,
        .spatialAudio = config.spatialAudio,
        .outputSampleRate = config.outputSampleRate,
        .diagnosticsLevel = config.diagnosticsLevel,
    };
}

void Audio::validate(const Config::Audio& config) {
    if (config.masterVolume.percent_ < 0 || config.masterVolume.percent_ > 100) {
        throw std::invalid_argument("masterVolume must be between 0 and 100");
    }
    if (config.musicVolume.percent_ < 0 || config.musicVolume.percent_ > 100) {
        throw std::invalid_argument("musicVolume must be between 0 and 100");
    }
    if (config.sfxVolume.percent_ < 0 || config.sfxVolume.percent_ > 100) {
        throw std::invalid_argument("sfxVolume must be between 0 and 100");
    }
    if (config.outputSampleRate.hz_ <= 0) {
        throw std::invalid_argument("outputSampleRate must be > 0");
    }
}

void Audio::ensureRunning() const {
    if (!running_) {
        throw std::runtime_error("audio is not running");
    }
}
