project('assembly_subroutines_c_meson', 'c',
  default_options: ['c_std=c11'])

cpu = host_machine.cpu_family()
system = host_machine.system()

sources = ['src/main.c']

if cpu == 'x86_64'
  if system == 'windows'
    sources += 'asm/x86_64_windows/add_clang.s'
  elif system == 'linux' or system == 'darwin'
    sources += 'asm/x86_64/add.s'
  else
    error('Unsupported x86_64 platform: ' + system)
  endif
elif cpu == 'aarch64' or cpu == 'arm64'
  sources += 'asm/aarch64/add.s'
else
  error('Unsupported architecture: ' + cpu)
endif

executable('assembly_subroutines_c_meson', sources)
