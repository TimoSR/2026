# Audio and GPU API Map

This file now runs real probes:

- `waveOutGetNumDevs` for audio output device count
- `D3D11CreateDevice` for hardware and WARP device creation

The goal is a minimal practical pattern: detect capability first, then choose your rendering/audio path.

`chooseGraphicsBackend(...)` demonstrates a concrete decision policy: hardware first, WARP fallback second.
