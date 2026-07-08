#include "Audio.h"
#include <iostream>
#include <stdexcept>
#include <utility>

Audio Audio::Create() {
    const Config::Diagnostics diagnostics = Config::makeDefaultDiagnosticsConfig();
    const Config::Audio audioConfig = Config::makeDefaultAudioConfig();
    return Create(Config::AudioSettingsView{
        .diagnostics = diagnostics,
        .audio = audioConfig,
    });
}

Audio Audio::Create(const Config::App& config) {
    Config::validateConfig(config);
    return Create(Config::getAudioSettings(config));
}

Audio Audio::Create(Config::AudioSettingsView settings) {
    Audio audio(translate(settings));
    audio.running_ = true;
    std::cout << "Audio created with composed audio settings.\n";
    audio.printSummary();
    return audio;
}

void Audio::UpdateConfig(const Config::App& config) {
    Config::validateConfig(config);
    UpdateConfig(Config::getAudioSettings(config));
}

void Audio::UpdateConfig(Config::AudioSettingsView settings) {
    ensureRunning();
    config_ = translate(settings);
    std::cout << "Audio configuration updated from shared app config.\n";
    printSummary();
}

void Audio::PlaySound(const char* filePath) const {
    ensureRunning();
    if (filePath == nullptr || *filePath == '\0') {
        throw std::invalid_argument("sound filePath must be non-empty");
    }

    std::cout << "Playing sound stream: " << filePath
              << " [backend=" << toString(config_.audioBackend)
              << ", masterVolume=" << config_.masterVolume.percent << "%"
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
    std::cout << "  masterVolume: " << config_.masterVolume.percent << "%\n";
    std::cout << "  musicVolume: " << config_.musicVolume.percent << "%\n";
    std::cout << "  sfxVolume: " << config_.sfxVolume.percent << "%\n";
    std::cout << "  spatialAudio: " << toString(config_.spatialAudio) << '\n';
    std::cout << "  outputSampleRate: " << config_.outputSampleRate.hz << " Hz\n";
    std::cout << "  diagnostics: " << toString(config_.diagnosticsLevel) << '\n';
}

Audio::Audio(InternalAudioConfig config)
    : config_(std::move(config)) {}

Audio::InternalAudioConfig Audio::translate(Config::AudioSettingsView settings) {
    validate(settings);
    return InternalAudioConfig{
        .audioBackend = settings.audio.audioBackend,
        .masterVolume = settings.audio.masterVolume,
        .musicVolume = settings.audio.musicVolume,
        .sfxVolume = settings.audio.sfxVolume,
        .spatialAudio = settings.audio.spatialAudio,
        .outputSampleRate = settings.audio.outputSampleRate,
        .diagnosticsLevel = settings.diagnostics.level,
    };
}

void Audio::validate(Config::AudioSettingsView settings) {
    if (settings.audio.masterVolume.percent > 100U) {
        throw std::invalid_argument("masterVolume must be between 0 and 100");
    }

    if (settings.audio.musicVolume.percent > 100U) {
        throw std::invalid_argument("musicVolume must be between 0 and 100");
    }

    if (settings.audio.sfxVolume.percent > 100U) {
        throw std::invalid_argument("sfxVolume must be between 0 and 100");
    }

    if (settings.audio.outputSampleRate.hz < 8000U) {
        throw std::invalid_argument("outputSampleRate must be >= 8000");
    }
}

void Audio::ensureRunning() const {
    if (!running_) {
        throw std::runtime_error("audio is not running");
    }
}
