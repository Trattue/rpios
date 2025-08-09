use core::fmt::Write;

mod drivers;

pub trait SerialWriter: Write {}
