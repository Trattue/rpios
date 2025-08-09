use core::fmt::Write;

use crate::hal::SerialWriter;

struct Pl011Uart(*mut usize);

impl Write for Pl011Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        todo!()
    }
}

impl SerialWriter for Pl011Uart {}

#[repr(C)]
pub struct UartRegisters {
    /// Data register
    dr: u32,
    /// RSRECR Register
    rsrect: [u32; 5],
    /// Flag register
    fr: [u32; 2],
    /// not in use
    ilpr: u32,
    /// Integer Baud rate divisor
    ibrd: u32,
    /// Fractional Baud rate divisor
    fbrd: u32,
    /// Line Control register
    lcrh: u32,
    /// Control register
    cr: u32,
    /// Interupt FIFO Level Select Register
    ifls: u32,
    /// Interupt Mask Set Clear Register
    imsc: u32,
    /// Raw Interupt Status Register
    ris: u32,
    /// Masked Interupt Status Register
    mis: u32,
    /// Interupt Clear Register
    icr: u32,
    /// DMA Control Register
    dmacr: [u32; 14],
    /// Test Control register
    itcr: u32,
    /// Integration test input reg
    itip: u32,
    /// Integration test output reg
    itop: u32,
    /// Test Data reg
    tdr: u32,
}

#[cfg(test)]
pub mod tests {
    use core::mem;

    use super::*;

    #[test]
    fn uart_registers_struct() {
        assert_eq!(mem::offset_of!(UartRegisters, dr), 0x0);
        assert_eq!(mem::offset_of!(UartRegisters, rsrect), 0x4);
        assert_eq!(mem::offset_of!(UartRegisters, fr), 0x18);
        assert_eq!(mem::offset_of!(UartRegisters, ilpr), 0x20);
        assert_eq!(mem::offset_of!(UartRegisters, ibrd), 0x24);
        assert_eq!(mem::offset_of!(UartRegisters, fbrd), 0x28);
        assert_eq!(mem::offset_of!(UartRegisters, lcrh), 0x2c);
        assert_eq!(mem::offset_of!(UartRegisters, cr), 0x30);
        assert_eq!(mem::offset_of!(UartRegisters, ifls), 0x34);
        assert_eq!(mem::offset_of!(UartRegisters, imsc), 0x38);
        assert_eq!(mem::offset_of!(UartRegisters, ris), 0x3c);
        assert_eq!(mem::offset_of!(UartRegisters, mis), 0x40);
        assert_eq!(mem::offset_of!(UartRegisters, icr), 0x44);
        assert_eq!(mem::offset_of!(UartRegisters, dmacr), 0x48);
        assert_eq!(mem::offset_of!(UartRegisters, itcr), 0x80);
        assert_eq!(mem::offset_of!(UartRegisters, itip), 0x84);
        assert_eq!(mem::offset_of!(UartRegisters, itop), 0x88);
        assert_eq!(mem::offset_of!(UartRegisters, tdr), 0x9c);
    }
}
