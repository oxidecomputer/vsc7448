# `mesa-parse`

This crate parses files from the [Microchip Ethernet Switch API](https://github.com/microchip-ung/mesa/)
(not the [other Mesa](https://www.mesa3d.org/)) and generates Rust code to
interact with register maps and associated metadata.

It produces two flavors of code:

- Embedded register maps, which let us interact with strongly-typed register
  addresses and values in a `#[no_std]` environment.
- User-facing **info** code, which lets us look up registers by (string) name
  or address, then access useful metadata.

## Invocation
```
cargo run --release -- --pac ../vsc7448-pac --info ../vsc7448-info PATH/TO/mesa-v2021.09 jaguar2:jaguar2c
```

(in fact, this is how `vsc7448-pac` and `vsc7448-info` are generated in the repo)
