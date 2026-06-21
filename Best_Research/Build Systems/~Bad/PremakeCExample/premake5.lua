workspace "assembly_subroutines_c_premake"
   configurations { "Debug", "Release" }
   architecture "x86_64"
   language "C"
   cdialect "C11"

project "assembly_subroutines_c_premake"
   kind "ConsoleApp"
   targetdir "build/bin/%{cfg.buildcfg}"
   objdir "build/obj/%{cfg.buildcfg}"
   files { "src/main.c" }

   filter "system:windows"
      files { "asm/x86_64_windows/add.asm" }

   filter { "system:linux or system:macosx", "architecture:x86_64" }
      files { "asm/x86_64/add.s" }

   filter "architecture:ARM64"
      files { "asm/aarch64/add.s" }

   filter "configurations:Debug"
      symbols "On"

   filter "configurations:Release"
      optimize "On"

   filter {}
