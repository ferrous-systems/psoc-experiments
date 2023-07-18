# psoc6-demo

A demo project for the CY8C624ABZI-S2D44A0 SoC, as fitted to the
*CY8CPROTO-062-4343W PSoC 6 Wi-Fi BT Prototyping kit*.

Schematic: <https://www.infineon.com/dgdl/Infineon-CY8CPROTO-062-4343W_Schematic-PCBDesignData-v01_00-EN.pdf?fileId=8ac78c8c7d0d8da4017d0f010c6d183a>

## Building

There are multiple binary crates in this repository.

To run a Cortex-M4F binary:

``` console
$ export OPENOCD_ROOT=/where/you/installed/infineon/openocd
$ ${OPENOCD_ROOT}/bin/openocd -s ${OPENOCD_ROOT}/scripts
<now open another terminal>
$ rustup target add thumbv7em-none-eabihf
$ export RUST_GDB=arm-none-eabi-gdb
$ cd psoc6-demo-cm4
$ cargo run --release
```

To run a Cortex-M0+ binary:

``` console
$ export OPENOCD_ROOT=/where/you/installed/infineon/openocd
$ ${OPENOCD_ROOT}/bin/openocd -s ${OPENOCD_ROOT}/scripts
<now open another terminal>
$ rustup target add thumbv6m-none-eabihf
$ export RUST_GDB=arm-none-eabi-gdb
$ cd psoc6-demo-cm0
$ cargo run --release
```

Document AN215656 says:

> After CM0+ executes the system and security code, it executes the application
> code. In the application code, CM0+ may release the CM4 reset, causing CM4 to
> start executing its application code.

So if you want your code to run outside of the debugger, compile it for the
Cortex-M0+ (thumbv6m-none-eabi). Also if you want your code to run outside of
the debugger, you can't use semihosting.

## Interrupts

The SVD file (and hence the PAC) describes all 187-odd interrupts. However, the
Cortex-M0+ core only has 8 external and 8 internal interrupts. You therefore
can't use the PAC in "rt" mode on the Cortex-M0+ core - the interrupt vector
table won't fit.

Also, pending any kind of interrupt (e.g. `svc 0`) causes the core to double
fault. No idea why, but it stops things like RTIC working. Happens in either
core.

# License

This template is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
