target extended-remote :3333
set print asm-demangle on
set backtrace limit 32
monitor arm semihosting enable
monitor reset halt
load
layout split
stepi
