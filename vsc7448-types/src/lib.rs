use std::collections::HashMap;

#[derive(Debug)]
pub struct Field {
    pub brief: Option<String>,
    pub details: Option<String>,
    pub lo: usize,
    pub hi: usize,
}

#[derive(Debug)]
pub struct Register {
    pub addr: Address,
    pub brief: Option<String>,
    pub details: Option<String>,
    pub fields: HashMap<String, Field>,
}

#[derive(Debug)]
pub struct RegisterGroup {
    pub addr: Address,
    pub desc: String,
    pub regs: HashMap<String, Register>,
}

#[derive(Debug)]
pub struct Target {
    pub desc: String,
    pub groups: HashMap<String, RegisterGroup>,
}

#[derive(Copy, Clone, Debug)]
pub struct Address {
    pub base: usize,
    pub count: usize,
    pub width: usize,
}
