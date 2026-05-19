// src/main.cpp
#define FMT_HEADER_ONLY
#include <fmt/core.h>

#include <SDL.h>
#include <chrono>
#include <thread>

int sdl_panic(const char* msg) {
    fmt::print(stderr, "[SDL ERROR] {}: {}\n", msg, SDL_GetError());
    return 1;
}

int main(int argc, char** argv) {
    (void)argc; (void)argv;

    fmt::print("hello from ccargo + fmt + SDL2 👋\n");

    if (SDL_Init(SDL_INIT_VIDEO) != 0) {
        return sdl_panic("SDL_Init failed");
    }

    SDL_Window* win = SDL_CreateWindow(
        "ccargo SDL2 demo",
        SDL_WINDOWPOS_CENTERED,
        SDL_WINDOWPOS_CENTERED,
        800, 600,
        SDL_WINDOW_SHOWN
    );
    if (!win) {
        SDL_Quit();
        return sdl_panic("SDL_CreateWindow failed");
    }

    SDL_Renderer* ren = SDL_CreateRenderer(win, -1, SDL_RENDERER_ACCELERATED | SDL_RENDERER_PRESENTVSYNC);
    if (!ren) {
        SDL_DestroyWindow(win);
        SDL_Quit();
        return sdl_panic("SDL_CreateRenderer failed");
    }

    bool running = true;
    while (running) {
        SDL_Event e;
        while (SDL_PollEvent(&e)) {
            if (e.type == SDL_QUIT) running = false;
            if (e.type == SDL_KEYDOWN && e.key.keysym.sym == SDLK_ESCAPE) running = false;
        }

        // Clear with a nice gray
        SDL_SetRenderDrawColor(ren, 30, 30, 36, 255);
        SDL_RenderClear(ren);

        // Draw a white rectangle
        SDL_Rect r{ 100, 100, 200, 150 };
        SDL_SetRenderDrawColor(ren, 240, 240, 240, 255);
        SDL_RenderFillRect(ren, &r);

        SDL_RenderPresent(ren);
    }

    SDL_DestroyRenderer(ren);
    SDL_DestroyWindow(win);
    SDL_Quit();
    return 0;
}
