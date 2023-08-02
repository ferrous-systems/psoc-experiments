# A slightly less-tiny example for the PSoC6

This example uses no dependencies, but runs Rust code on the Cortex-M0+ core
of a PSoC6.

First, start Infineon `openocd` in another window, then run:

```console
$ cargo build
$ rust-gdb ./target/thumbv6m-none-eabi/debug/psoc6-cm0-tiny2
> target extended-remote :3333
> monitor reset halt
> load
> layout split
> stepi
```

Now you can single-step through the code.