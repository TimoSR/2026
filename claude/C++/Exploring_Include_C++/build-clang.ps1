$ErrorActionPreference = "Stop"

$BuildDir = "build"
New-Item -ItemType Directory -Force $BuildDir | Out-Null

$AudioPcm = "$BuildDir\Audio-$PID.pcm"

clang++ -std=c++23 -Wall -Wextra -Wpedantic --precompile Audio\Audio.cppm -o $AudioPcm
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

clang++ -std=c++23 -Wall -Wextra -Wpedantic "-fmodule-file=Audio=$AudioPcm" -c Main.cpp -o $BuildDir\Main.obj
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
