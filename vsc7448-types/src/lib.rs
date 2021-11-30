//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::HashMap;

/// Represents a single bitfield within a register, containing bits `lo`
/// (inclusive) through `hi` (exclusive).  For example, a bitfield that
/// contains the entire register has `lo: 0, hi: 32`.
///
/// Typically parameterized with either `String` or `'static str`, depending on
/// how it's used in the program.
#[derive(Debug)]
pub struct Field<S> {
    pub brief: Option<S>,
    pub details: Option<S>,
    pub lo: u8,
    pub hi: u8,
}

/// Represents a single register (or register array) within a register group
///
/// Typically parameterized with either `String` or `'static str`, depending on
/// how it's used in the program.
#[derive(Debug)]
pub struct Register<S> {
    pub addr: Address,
    pub brief: Option<S>,
    pub details: Option<S>,
    pub fields: HashMap<S, Field<S>>,
}

/// A register group (or register group array) within a target
///
/// Typically parameterized with either `String` or `'static str`, depending on
/// how it's used in the program.
#[derive(Debug)]
pub struct RegisterGroup<S> {
    pub addr: Address,
    pub desc: S,
    pub regs: HashMap<S, Register<S>>,
}

/// A top-level "target" within the VSC7448.
///
/// Typically parameterized with either `String` or `'static str`, depending on
/// how it's used in the program.
#[derive(Debug)]
pub struct BaseTarget<S> {
    pub desc: S,
    pub groups: HashMap<S, RegisterGroup<S>>,
}
pub type OwnedTarget = BaseTarget<String>;
pub type Target = BaseTarget<&'static str>;

/// Represents the address of an item or array of items.  `base` and `width`
/// are given in 32-bit words, so the physical address is decoded as
/// `(base + index * width) * 4`
#[derive(Copy, Clone, Debug)]
pub struct Address {
    pub base: u32,
    pub count: u32,
    pub width: u32,
}

/// Represents a PHY register page
#[derive(Debug)]
pub struct Page<S> {
    pub desc: S,
    pub base: u32,
    pub regs: HashMap<S, Register<S>>,
}
