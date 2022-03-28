# llvm-model
[![Project Status: Suspended – Initial development has started, but there has not yet been a stable, usable release; work has been stopped for the time being but the author(s) intend on resuming work.](https://www.repostatus.org/badges/latest/suspended.svg)](https://www.repostatus.org/#suspended)

Rust library for writing LLVM IR. Unlike [`inkwell`](https://github.com/TheDan64/inkwell) which continuously calls LLVM C API
functions, `llvm-model` instead only calls the at the last possible moment, when a module is finished being constructed.

Development has been paused until further notice, now that I realized one could call
[`inkwell::targets::TargetMachine::write_to_memory_buffer`](https://thedan64.github.io/inkwell/inkwell/targets/struct.TargetMachine.html#method.write_to_memory_buffer)
to "compile" an LLVM module into a native assembly or object file.
