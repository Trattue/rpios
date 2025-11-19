use core::ptr::read_volatile;

pub struct MmioAccess {
    pub base_address: *mut (),
}

impl MmioAccess {
    pub fn new(base_address: usize) -> Self {
        let ptr = base_address as *mut ();
        return MmioAccess { base_address: ptr };
    }

    pub unsafe fn read_at_offset<T>(self, offset: usize) -> T {
        let ptr = unsafe { self.base_address.byte_add(offset) } as *mut T;
        return unsafe { read_volatile(ptr) };
    }
}
