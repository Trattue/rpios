const UART: *mut UartRegisters = 0x7E20100 as _;

#[repr(C)]
pub struct UartRegisters {
    dr: u32,
    rsrect: [u32; 5],
    fr: [u32; 2],
    ilpr: u32,
    ibrd: u32,
    fbrd: u32,
    lcrh: u32,
    cr: u32,
    ifls: u32,
    imsc: u32,
    ris: u32,
    mis: u32,
    icr: u32,
    dmacr: [u32; 14],
    itcr: u32,
    itip: u32,
    itop: u32,
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
