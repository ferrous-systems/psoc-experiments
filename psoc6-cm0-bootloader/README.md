# psoc6 bootloader

This is a promise that life will get easier this way.

In this example setup, we are using the CM4 RTIC application and loading it, via the bootloader.

## Build

1. Build the [psoc6-cm4-rtic](../psoc6-cm4-rtic/) first

    ```sh
    # get in
    pushd ../psoc6-cm4-rtic
    # build
    cargo build --release features=use-bootloader
    # get out
    popd
    ```

2. Copy the file, rename it so it matches the bootloader requirements (`app.bin`)

    ```sh
    arm-none-eabi-objcopy -O binary ../psoc6-cm4-rtic/target/thumbv7em-none-eabihf/release/psoc6-cm4-rtic ./src/app.bin
    ```
