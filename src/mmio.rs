#[macro_export]
macro_rules! mmio_read {
    ($expr:expr) => {
        unsafe { expr.read_volatile() }
    };
    ($expr:expr, $ident:ident) => {{
        use core::ptr::addr_of;
        unsafe { addr_of!((*$expr).$ident).read_volatile() }
    }};
}

#[macro_export]
macro_rules! mmio_write {
    ($expr:expr, $data:expr) => {
        unsafe { $expr.write_volatile($data) };
    };
    ($expr:expr, $ident:ident, $data:expr) => {
        unsafe { (&raw mut (*$expr).$ident).write_volatile($data) };
    };
}
