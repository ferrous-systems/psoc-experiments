# psoc6-demo

A demo project for the CY8C624ABZI-S2D44A0 SoC, as fitted to the
*CY8CPROTO-062-4343W PSoC 6 Wi-Fi BT Prototyping kit*.

Schematic: <https://www.infineon.com/dgdl/Infineon-CY8CPROTO-062-4343W_Schematic-PCBDesignData-v01_00-EN.pdf?fileId=8ac78c8c7d0d8da4017d0f010c6d183a>

## Building

There are multiple binary crates in this repository.
There are multiple cores in this SoC.

### Prerequisites

1. Install correct Rust targets for both cores.

    ```sh
    rustup target add thumbv7em-none-eabihf
    rustup target add thumbv6m-none-eabi
    ```

2. Make sure you have GDB installed.
3. Make sure your `openocd` installation is the patched version from Infineon.
    Regular off-the-shelf won't work. Download the
    [Toolset from Infineon](https://softwaretools.infineon.com/tools/com.ifx.tb.tool.cypressprogrammer).
    We used Version 4.2.0.999 for this work.
4. Export the correct environment variable for `openocd`

    ```sh
    export OPENOCD_ROOT=/where/you/installed/infineon/openocd
    ```

    Please make sure this is available to all shell/terminal processes,
    so it is good to set this in your "rc".

### Run a Cortex-M4F binary

1. Open terminal

    ```sh
    # The scripts path might be different depending on whether you
    # have built and installed Infineons openocd yourself or if
    # have downloaded the pre-built Cypress toolset. This tutorial
    # assumes the latter.

    # Make sure you are in this repo
    cd /path/to/checkout/of/psoc-experiments

    ${OPENOCD_ROOT}/bin/openocd -s ${OPENOCD_ROOT}/scripts
    ```

2. Open another terminal

    ```sh
    # Make sure you are in this repo
    cd /path/to/checkout/of/psoc-experiments

    # If you have another installation of GDB,
    # e.g. Ubuntu calls its GDB `gdb-multiarch`, set
    # `export RUST_GDB=gdb-multiarch`
    export RUST_GDB=arm-none-eabi-gdb

    cd psoc6-cm4-hello-world
    cargo run --release
    ```

### Run a Cortex-M0+ binary

1. Open terminal

    ```sh
    # The scripts path might be different depending on whether you
    # have built and installed Infineons openocd yourself or if
    # have downloaded the pre-built Cypress toolset. This tutorial
    # assumes the latter.

    # Make sure you are in this repo
    cd /path/to/checkout/of/psoc-experiments

    ${OPENOCD_ROOT}/bin/openocd -s ${OPENOCD_ROOT}/scripts
    ```

2. Open another terminal
  
    ```sh
    # Make sure you are in this repo
    cd /path/to/checkout/of/psoc-experiments

    # If you have another installation of GDB,
    # e.g. Ubuntu calls its GDB `gdb-multiarch`, set
    # `export RUST_GDB=gdb-multiarch`
    export RUST_GDB=arm-none-eabi-gdb

    cd psoc6-cm0-hello-world
    cargo run --release
    ```

So if you want your code to run outside of the debugger, compile it for the
Cortex-M0+ (thumbv6m-none-eabi). Also if you want your code to run outside of
the debugger, you can't use semihosting.

### Run bootloader application

Check out [psoc6-cm0-bootloader/README.md](./psoc6-cm0-bootloader/README.md)
for instructions.

## Misc. notes

### Interrupts

The SVD file (and hence the PAC) describes all 187-odd interrupts. However, the
Cortex-M0+ core only has 8 external and 8 internal interrupts. You therefore
can't use the PAC in "rt" mode on the Cortex-M0+ core - the interrupt vector
table won't fit.

### Booting

Document AN215656 *PSoC™ 6 MCU dual-core system design* says:

> After CM0+ executes the system and security code, it executes the application
> code. In the application code, CM0+ may release the CM4 reset, causing CM4 to
> start executing its application code.

When the system comes out of reset normally, the Cortex-M0+ starts executing
from the on-board ROM and the Cortex-M4 is held in reset. The ROM bootloader
will at some point jump to the reset routine defined in the vector table at the
start of flash. However, it will not update the Cortex-M0+ VTOR register, so any
exceptions or interrupts will still be pointing at the ROM and will not work. In
practice this seems to cause a bounce off to random addresses and a double
fault.

The mechanism described in AN215656 *PSoC™ 6 MCU dual-core system design* is:

1. The ROM runs on Cortex-M0+.
2. The ROM jumps to your reset routine in flash.
3. The code running on the Cortex-M0+ should set up the Cortex-M4's VTOR, using
   a special Cypress system register (because the Cortex-M4's VTOR isn't
   available to the Cortex-M0+ as they are in the private core-local address
   range)..
4. The code on the Cortex-M0+ should use the same block to take the Cortex-M4 out of reset.

So I imagine your firmware would look like:

```text
0x1000_0000 +---------------------------+
            | Stack Ptr for Cortex-M0+  |
            | Reset Ptr for Cortex-M0+  |
            | Exceptions for Cortex-M0+ |
            ¦ ...                       ¦
            | Interrupts for Cortex-M0+ |
            ¦ ...                       ¦
            | Code/Data  for Cortex-M0+ |
            ¦ ...                       ¦
0x1001_0000 +---------------------------+
            | Stack Ptr for Cortex-M4   |
            | Reset Ptr for Cortex-M4   |
            | Exceptions for Cortex-M4  |
            ¦ ...                       ¦
            | Interrupts for Cortex-M4  |
            ¦ ...                       ¦
            | Code/Data  for Cortex-M4  |
            ¦ ...                       ¦
            +---------------------------+
```

Your Cortex-M0+ and Cortex-M4 binaries will need different `memory.x` files so
they each get their own piece of RAM and Flash, and you then need to join them
together. You could do that as a hex-merge, or by including the Cortex-M4
firmware as a static `[u8; nnnn]` within the Cortex-M0+ firmware, located within
an appropriate section so it is linked into memory in the Cortex-M0+ binary at
the same place the Cortex-M4 linker thought it was going to be.

## License

This template is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
