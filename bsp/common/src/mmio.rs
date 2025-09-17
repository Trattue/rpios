/// Reads an MMIO register or the field of an MMIO struct.
///
/// # Safety
///
/// - `$expr` must be a valid memory address to read from.
/// - If supplied, `$ident` must be a valid field of `$expr` that points to valid memory to read from.
#[macro_export]
macro_rules! mmio_read {
    // Pointer to register
    ($expr:expr) => {
        unsafe { expr.read_volatile() }
    };
    // Pointer to struct of registers
    ($expr:expr, $ident:ident) => {{
        use core::ptr::addr_of;
        unsafe { (&raw const (*$expr).$ident).read_volatile() }
    }};
}

/// Writes to an MMIO register or to the field of an MMIO struct.
///
/// # Safety
///
/// - `$expr` must be a valid memory address to read from.
/// - If supplied, `$ident` must be a valid field of `$expr` that points to valid memory to read from.
#[macro_export]
macro_rules! mmio_write {
    // Pointer to register
    ($expr:expr, $data:expr) => {
        unsafe { $expr.write_volatile($data) };
    };
    // Pointer to struct of registers
    ($expr:expr, $ident:ident, $data:expr) => {
        unsafe { (&raw mut (*$expr).$ident).write_volatile($data) };
    };
}
