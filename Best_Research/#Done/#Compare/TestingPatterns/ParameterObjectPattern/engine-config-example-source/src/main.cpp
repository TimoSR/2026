#include <cstdlib>
#include <exception>
#include <iostream>

#include "config/app_configs.hpp"
#include "engine/asset_store.hpp"
#include "engine/audio_engine.hpp"
#include "engine/os.hpp"
#include "engine/physics_engine.hpp"
#include "engine/render_engine.hpp"
#include "engine/window.hpp"

int main()
{
    using namespace demo;

    try
    {
        FileSystem fileSystem = OS::FileSystem();

        RenderEngine renderEngine = RenderEngine::Create(config::app::CreateRenderStartup());
        AudioEngine audioEngine = AudioEngine::Create(config::app::CreateAudioStartup());
        PhysicsEngine physicsEngine = PhysicsEngine::Create(config::app::CreatePhysicsStartup());
        Window window = Window::Open(config::app::CreateWindowStartup());
        AssetStore assets = AssetStore::Create(fileSystem);

        const ImageId whale = assets.LoadImage("whale", "whale.png");
        const SoundId music = assets.LoadSound("music", "music.ogg");

        audioEngine.Play(music, config::app::CreateMusicPlayback());

        {
            RenderEngine::Frame frame = renderEngine.BeginFrame(window);
            frame.DrawText("Hello World!", 400, 300);
            frame.DrawImage(whale, 300, 200);
        }

        window.UpdateConfig(config::app::CreateWindowRuntimeUpdate());
        renderEngine.UpdateConfig(config::app::CreateRenderRuntimeUpdate());

        {
            RenderEngine::Frame frame = renderEngine.BeginFrame(window);
            frame.DrawText("After runtime config update", 180, 120);
            frame.DrawImage(whale, 300, 200);
        }

        std::cout << "[physics] gravity in use=" << ToString(physicsEngine.Gravity()) << '\n';

        audioEngine.StopAll();
        renderEngine.Stop();
        window.Close();

        return EXIT_SUCCESS;
    }
    catch (const std::exception& exception)
    {
        std::cerr << "[fatal] " << exception.what() << '\n';
        return EXIT_FAILURE;
    }
}
