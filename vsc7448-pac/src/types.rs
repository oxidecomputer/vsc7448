pub struct RegisterAddress<T> {
    pub addr: u32,
    _phantom: core::marker::PhantomData<*const T>,
}
// We can't #[derive(Copy, Clone)] because of PhantomData
impl<T> Copy for RegisterAddress<T> {}
impl<T> Clone for RegisterAddress<T> {
    fn clone(&self) -> Self {
        Self {
            addr: self.addr,
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<T> RegisterAddress<T>
where
    T: From<u32>,
    u32: From<T>,
{
    pub(crate) fn new(addr: u32) -> Self {
        Self {
            addr,
            _phantom: core::marker::PhantomData,
        }
    }
    pub fn from_addr_unchecked(addr: u32) -> Self {
        Self::new(addr)
    }
}

pub struct PhyRegisterAddress<T> {
    pub page: u16,
    pub addr: u8,

    // This is pub(crate) so that we can construct PhyRegisterAddress<T>
    // with a const fn, e.g. phy::STANDARD::MODE_CONTROL(); we can't have a
    // normal new() function because "trait objects in const fn are unstable"
    pub(crate) _phantom: core::marker::PhantomData<*const T>,
}
// We can't #[derive(Copy, Clone)] because of PhantomData
impl<T> Copy for PhyRegisterAddress<T> {}
impl<T> Clone for PhyRegisterAddress<T> {
    fn clone(&self) -> Self {
        Self {
            page: self.page,
            addr: self.addr,
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<T> PhyRegisterAddress<T>
where
    T: From<u16>,
    u16: From<T>,
{
    pub fn from_page_and_addr_unchecked(page: u16, addr: u8) -> Self {
        Self {
            page,
            addr,
            _phantom: core::marker::PhantomData,
        }
    }
}
