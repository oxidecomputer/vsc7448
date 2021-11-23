# `mesa-parse`

This crate parses files from the [Microchip Ethernet Switch API](https://github.com/microchip-ung/mesa/)
(not the [other Mesa](https://www.mesa3d.org/)) and generates Rust code to
interact with register maps and associated metadata.

Right now, it produces user-facing **info** code, which lets us look up
registers by (string) name.

In the future, it will also produce code for an embedded system, where we
presumably know register names and just want to get their addresses (at compile
time, with zero cost).

## Example invocation
```
cargo run --release -- PATH/TO/mesa-v2021.09 jaguar2:jaguar2c > ../vsc7448-info/src/lib.rs
```
