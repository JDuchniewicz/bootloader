target remote :3333
set backtrace limit 32
load
monitor reset
break bootloader::reset::reset_handler
break bootloader::main
monitor halt
