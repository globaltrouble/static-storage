#![no_std]

pub struct StaticStorage<T> {
    mask: u128,
    inner: Option<T>,
}

impl<T> StaticStorage<T> {
    const INIT_MASK: u128 = 0x158b76a71811b62d3e3fc72491dbaca9;
    const DEFAULT_MASK: u128 = 0;

    pub const fn empty() -> StaticStorage<T> {
        StaticStorage::<T> {
            mask: Self::DEFAULT_MASK,
            inner: None,
        }
    }

    pub fn init<F>(&mut self, f: F)
    where
        F: FnOnce() -> T,
    {
        if self.is_init() {
            return;
        }

        unsafe { core::ptr::write_volatile(&mut self.inner, Some(f())) };
        self.set_init();
    }

    pub fn as_mut(&mut self) -> Option<&mut T> {
        self.inner.as_mut()
    }

    pub fn as_ref(&self) -> Option<&T> {
        self.inner.as_ref()
    }

    pub fn is_init(&self) -> bool {
        let mask = unsafe { core::ptr::read_volatile(&self.mask) };

        mask == Self::INIT_MASK
    }

    fn set_init(&mut self) {
        unsafe { core::ptr::write_volatile(&mut self.mask, Self::INIT_MASK) };
    }
}
