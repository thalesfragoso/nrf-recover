# nrf-recover

Unlocks nRF52 devices using a CMSIS-DAP or JLink probe.

## Installation

Using cargo:

```console
$ cargo install nrf-recover
```

## Usage

This utility uses [probe-rs](https://crates.io/crates/probe-rs) to interface with the debug probe, `probe-rs` requires `libusb` and udev rules properly configured.

Connect your microcontroller to the debug probe and run:

```console
$ nrf-recover
```

This process has a timeout of 15 seconds, but it usually completes in less than one second. If you get a timeout, you should reset the microcontroller and try again. Please open an issue if you can not unlock the chip after several tries.

**Attention**: This process will erase the entire code flash and UICR area of the device, in addition to the entire RAM.

There is a flag to enable mass erase with a st-link, this is useful in case your chip is in a bad state and you only have a st-link, but it will **not** work if your chip is indeed locked.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
