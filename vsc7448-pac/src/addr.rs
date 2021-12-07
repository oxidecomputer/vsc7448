/// Represents a register address tagged with the type which should be used
/// when performing a read or write at that address.  The type is constrained
/// to be converted to/from a `u32` using `From`.
pub struct RegisterAddress<T> {
    /// Register address in the VSC7448's memory
    pub addr: u32,
    _phantom: std::marker::PhantomData<*const T>,
}

// We can't #[derive(Copy, Clone)] because of PhantomData
impl<T> Copy for RegisterAddress<T> {}
impl<T> Clone for RegisterAddress<T> {
    fn clone(&self) -> Self {
        Self {
            addr: self.addr,
            _phantom: std::marker::PhantomData,
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
            _phantom: std::marker::PhantomData,
        }
    }
}

