# A even-less-tiny example for the PSoC6

This example uses cortex-m-rt to handle the start-up sequence, and
cortex-m-semihosting to give us some console output.

First, start Infineon `openocd` in another window, then run:

```console
$ cargo run
```

You will be dropped into GDB where you can single-step through the code.
