# nrf-unlock

Unlocks nRF52 devices using a CMSIS-DAP or JLink probe.

## Installation

Using cargo:

```console
$ cargo install nrf-unlock
```

## Usage

This utility uses [probe-rs](https://crates.io/crates/probe-rs) to interface with the debug probe, `probe-rs` requires `libusb` and udev rules properly configured.

Connect your microcontroller to the debug probe and run:

```console
$ nrf-unlock
```

**Attention**: This process will erase the entire code flash and UICR area of the device, in addition to the entire RAM.

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
