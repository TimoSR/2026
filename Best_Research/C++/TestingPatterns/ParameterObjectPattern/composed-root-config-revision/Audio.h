#pragma once

#include "Config/AppViews.h"

class Audio {
public:
    static Audio Create();
    static Audio Create(const Config::App& config);
    static Audio Create(Config::AudioSettingsView settings);

    void UpdateConfig(const Config::App& config);
    void UpdateConfig(Config::AudioSettingsView settings);

    void PlaySound(const char* filePath) const;
    void Stop();

    void printSummary() const;

private:
    struct InternalAudioConfig {
        Config::AudioBackend audioBackend = Config::AudioBackend::WASAPI;
        Config::Volume masterVolume = Config::Volume::from_percent(100);
        Config::Volume musicVolume = Config::Volume::from_percent(100);
        Config::Volume sfxVolume = Config::Volume::from_percent(100);
        Config::SpatialAudio spatialAudio = Config::SpatialAudio::Enabled;
        Config::SampleRate outputSampleRate = Config::SampleRate::from_hz(48000);
        Config::DiagnosticsLevel diagnosticsLevel = Config::DiagnosticsLevel::Basic;
    };

    explicit Audio(InternalAudioConfig config);

    static InternalAudioConfig translate(Config::AudioSettingsView settings);
    static void validate(Config::AudioSettingsView settings);
    void ensureRunning() const;

    InternalAudioConfig config_;
    bool running_ = false;
};
