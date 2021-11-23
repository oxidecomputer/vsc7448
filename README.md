# `vsc7448`
This repository contains a set of crates for working with the [Microchip VSC7448](https://www.microsemi.com/product-directory/ethernet-switches/3983-vsc7448)
ethernet switch IC.

It's useful if you're building a 10G ethernet switch from scratch, which is...
very few people.

- `vsc7448-types` are common types
- `mesa-parse` parses `MESA` headers and generates `vsc7448-info`
- `vsc7448-info` exposes a register map as a userland data structure
- `vsc7448-sys` (coming soon) exposes the same register map as raw pointers
  (for use in embedded devices)

## License
Copyright 2021 Oxide Computer Company

Released under the [MPL v2.0](https://www.mozilla.org/en-US/MPL/2.0/)
