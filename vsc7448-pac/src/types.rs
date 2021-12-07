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
}
