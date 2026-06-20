# Folder: 24_single_threaded_local_server/01_minimal_http_server

Run from project root:  
`C:\Users\timot\code\2026\Codex-Test\LearningCPP\cpp_quickstart_cheatsheet`

## Run Demo Server
```powershell
.\run_file.ps1 -File '24_single_threaded_local_server\01_minimal_http_server\minimal_http_server.cpp' -Std c++17 -Demo
```

Then in another shell:
```powershell
curl http://127.0.0.1:8080/health
curl http://127.0.0.1:8080/hello
```

## Run Tests
```powershell
.\run_file.ps1 -File '24_single_threaded_local_server\01_minimal_http_server\minimal_http_server.test.cpp' -Std c++17
```

