#include "RenderEngine.h"

int main() {
    RenderEngine engine = RenderEngine::Run();

    engine.LoadImage("whale", "whale.png");
    engine.PlaySound("music.ogg");

    engine.DrawText("Hello World!", 400, 300);
    engine.DrawImage("whale", 300, 200);

    engine.UpdateConfiguration({
        .renderBackend = RenderBackend::DirectX12,
        .resolution = Resolution::UHD4K(),
        .shadowQuality = ShadowQuality::Ultra,
        .antiAliasing = AntiAliasing::FXAA(),
        .targetFramesPerSecond = 144,
        .vSync = VSync::Enabled,
        .diagnosticsLevel = DiagnosticsLevel::Full,
    });

    engine.DrawText("After runtime config update", 180, 120);
    engine.Stop();
    return 0;
}
