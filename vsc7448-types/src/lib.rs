use std::collections::HashMap;

#[derive(Debug)]
pub struct Field<S> {
    pub brief: Option<S>,
    pub details: Option<S>,
    pub lo: usize,
    pub hi: usize,
}

#[derive(Debug)]
pub struct Register<S> {
    pub addr: Address,
    pub brief: Option<S>,
    pub details: Option<S>,
    pub fields: HashMap<S, Field<S>>,
}

#[derive(Debug)]
pub struct RegisterGroup<S> {
    pub addr: Address,
    pub desc: S,
    pub regs: HashMap<S, Register<S>>,
}

#[derive(Debug)]
pub struct BaseTarget<S> {
    pub desc: S,
    pub groups: HashMap<S, RegisterGroup<S>>,
}
pub type OwnedTarget = BaseTarget<String>;
pub type Target = BaseTarget<&'static str>;

#[derive(Copy, Clone, Debug)]
pub struct Address {
    pub base: usize,
    pub count: usize,
    pub width: usize,
}
