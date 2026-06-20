#pragma once

#include "ConfigTypes.h"

class Audio {
public:
    static Audio Create();
    static Audio Create(Config::Audio config);

    void UpdateConfig(Config::Audio config);
    void PlaySound(const char* filePath) const;
    void Stop();

    void printSummary() const;

private:
    struct InternalAudioConfig {
        AudioBackend audioBackend = AudioBackend::XAudio2;
        Volume masterVolume = Volume::from_percent(100);
        Volume musicVolume = Volume::from_percent(80);
        Volume sfxVolume = Volume::from_percent(90);
        SpatialAudio spatialAudio = SpatialAudio::Disabled;
        SampleRate outputSampleRate = SampleRate::from_hz(48000);
        DiagnosticsLevel diagnosticsLevel = DiagnosticsLevel::Basic;
    };

    explicit Audio(InternalAudioConfig config);

    static InternalAudioConfig translate(Config::Audio config);
    static void validate(const Config::Audio& config);
    void ensureRunning() const;

    InternalAudioConfig config_;
    bool running_ = false;
};

