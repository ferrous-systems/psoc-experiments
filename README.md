# psoc6-demo

A demo project for the CY8C624ABZI-S2D44A0 SoC, as fitted to the
*CY8CPROTO-062-4343W PSoC 6 Wi-Fi BT Prototyping kit*.

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
