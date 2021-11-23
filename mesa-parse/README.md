# `mesa-parse`

This crate parses files from the [Microchip Ethernet Switch API](https://github.com/microchip-ung/mesa/)
(not the [other Mesa](https://www.mesa3d.org/)) and generates machine-readable
register maps and associated metadata.

## Example invocation
```
cargo run --release -- PATH/TO/mesa-v2021.09 jaguar2:jaguar2c
```
