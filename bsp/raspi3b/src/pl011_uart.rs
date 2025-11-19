use common::mmio::MmioAccess;
use hal::uart::Uart;

pub struct Pl011Uart {}

impl Uart for Pl011Uart {
    fn init(&mut self) {
        // Disable UART
        todo!();

        // Wait for end of transmission/reception
        todo!();

        // Flush transmit FIFO
        todo!();

        // Reprogram control register:
        // - Calculate baud rate divisor
        // - Configure line control
        todo!();

        // Enable UART
        todo!();
    }

    fn write(&mut self, b: u8) {
        todo!()
    }

    fn try_read(&mut self) -> Option<u8> {
        todo!()
    }

    fn uninit(&mut self) {
        todo!()
    }
}

pub trait Bits {
    fn bit(self, index: Self) -> bool;
    fn bits(self, index_start: Self, index_end: Self) -> Self;
}

impl Bits for u32 {
    fn bit(self, index: Self) -> bool {
        return ((self >> index) & 1) == 1;
    }

    fn bits(self, index_start: Self, index_end: Self) -> Self {
        return (self >> index_end) & (Self::pow(2, index_start) - 1);
    }
}

pub struct UartRegisters {
    pub mmio: MmioAccess,
}

impl UartRegisters {
    const DR: usize = 0x0;
    const RSRECR: usize = 0x4;
    const FR: usize = 0x18;
    const ILPR: usize = 0x20;
    const IBRD: usize = 0x24;
    const FBRD: usize = 0x28;
    const LCRH: usize = 0x2c;
    const CR: usize = 0x30;
    const IFLS: usize = 0x34;
    const IMSC: usize = 0x38;
    const RIS: usize = 0x3c;
    const MIS: usize = 0x40;
    const ICR: usize = 0x44;
    const DMACR: usize = 0x48;
    const ITCR: usize = 0x80;
    const ITIP: usize = 0x84;
    const ITOP: usize = 0x88;
    const TDR: usize = 0x8c;

    pub fn new(base_address: usize) -> Self {
        UartRegisters {
            mmio: MmioAccess::new(base_address),
        }
    }

    pub fn dr(self) -> UartDr {
        let value = unsafe { self.mmio.read_at_offset(Self::DR) };
        return UartDr { value };
    }

    pub fn rsrecr(self) -> UartRsrecr {
        let value = unsafe { self.mmio.read_at_offset(Self::RSRECR) };
        return UartRsrecr { value };
    }

    pub fn fr(self) -> UartFr {
        let value = unsafe { self.mmio.read_at_offset(Self::FR) };
        return UartFr { value };
    }

    pub fn ilpr(self) -> u32 {
        return unsafe { self.mmio.read_at_offset(Self::ILPR) };
    }

    pub fn ibrd(self) -> UartIbrd {
        let value = unsafe { self.mmio.read_at_offset(Self::IBRD) };
        return UartIbrd { value };
    }

    pub fn fbrd(self) -> UartFbrd {
        let value = unsafe { self.mmio.read_at_offset(Self::FBRD) };
        return UartFbrd { value };
    }

    pub fn lcrh(self) -> UartLcrh {
        let value = unsafe { self.mmio.read_at_offset(Self::LCRH) };
        return UartLcrh { value };
    }

    pub fn cr(self) -> UartCr {
        let value = unsafe { self.mmio.read_at_offset(Self::CR) };
        return UartCr { value };
    }

    pub fn ifls(self) -> UartIfls {
        let value = unsafe { self.mmio.read_at_offset(Self::IFLS) };
        return UartIfls { value };
    }

    pub fn imsc(self) -> UartImsc {
        let value = unsafe { self.mmio.read_at_offset(Self::IMSC) };
        return UartImsc { value };
    }

    pub fn ris(self) -> UartRis {
        let value = unsafe { self.mmio.read_at_offset(Self::RIS) };
        return UartRis { value };
    }

    pub fn mis(self) -> UartMis {
        let value = unsafe { self.mmio.read_at_offset(Self::MIS) };
        return UartMis { value };
    }

    pub fn icr(self) -> UartIcr {
        let value = unsafe { self.mmio.read_at_offset(Self::ICR) };
        return UartIcr { value };
    }

    pub fn dmacr(self) -> UartDmacr {
        let value = unsafe { self.mmio.read_at_offset(Self::DMACR) };
        return UartDmacr { value };
    }

    pub fn itcr(self) -> UartItcr {
        let value = unsafe { self.mmio.read_at_offset(Self::ITCR) };
        return UartItcr { value };
    }

    pub fn itip(self) -> UartItip {
        let value = unsafe { self.mmio.read_at_offset(Self::ITIP) };
        return UartItip { value };
    }

    pub fn itop(self) -> UartItop {
        let value = unsafe { self.mmio.read_at_offset(Self::ITOP) };
        return UartItop { value };
    }

    pub fn tdr(self) -> UartTdr {
        let value = unsafe { self.mmio.read_at_offset(Self::TDR) };
        return UartTdr { value };
    }
}

pub struct UartDr {
    pub value: u32,
}

impl UartDr {
    const OE: u32 = 11;
    const BE: u32 = 10;
    const PE: u32 = 9;
    const FE: u32 = 8;
    const DATA_START: u32 = 7;
    const DATA_END: u32 = 0;

    pub fn oe(self) -> bool {
        return self.value.bit(Self::OE);
    }

    pub fn be(self) -> bool {
        return self.value.bit(Self::BE);
    }

    pub fn pe(self) -> bool {
        return self.value.bit(Self::PE);
    }

    pub fn fe(self) -> bool {
        return self.value.bit(Self::FE);
    }

    pub fn data(self) -> u8 {
        return self.value.bits(Self::DATA_START, Self::DATA_END) as u8;
    }
}

pub struct UartRsrecr {
    pub value: u32,
}

impl UartRsrecr {
    const OE: u32 = 3;
    const BE: u32 = 2;
    const PE: u32 = 1;
    const FE: u32 = 0;

    pub fn oe(self) -> bool {
        return self.value.bit(Self::OE);
    }

    pub fn be(self) -> bool {
        return self.value.bit(Self::BE);
    }

    pub fn pe(self) -> bool {
        return self.value.bit(Self::PE);
    }

    pub fn fe(self) -> bool {
        return self.value.bit(Self::FE);
    }
}

pub struct UartFr {
    pub value: u32,
}

impl UartFr {
    const RI: u32 = 8;
    const TXFE: u32 = 7;
    const RXFF: u32 = 6;
    const TXFF: u32 = 5;
    const RXFE: u32 = 4;
    const BUSY: u32 = 3;
    const DCD: u32 = 2;
    const DSR: u32 = 1;
    const CTS: u32 = 0;

    pub fn ri(self) -> bool {
        return self.value.bit(Self::RI);
    }

    pub fn txfe(self) -> bool {
        return self.value.bit(Self::TXFE);
    }

    pub fn rxff(self) -> bool {
        return self.value.bit(Self::RXFF);
    }

    pub fn txff(self) -> bool {
        return self.value.bit(Self::TXFF);
    }
    pub fn rxfe(self) -> bool {
        return self.value.bit(Self::RXFE);
    }

    pub fn busy(self) -> bool {
        return self.value.bit(Self::BUSY);
    }

    pub fn dcd(self) -> bool {
        return self.value.bit(Self::DCD);
    }

    pub fn dsr(self) -> bool {
        return self.value.bit(Self::DSR);
    }

    pub fn cts(self) -> bool {
        return self.value.bit(Self::CTS);
    }
}

pub struct UartIbrd {
    pub value: u32,
}

impl UartIbrd {
    const IBRD_START: u32 = 15;
    const IBRD_END: u32 = 0;

    pub fn ibrd(self) -> u16 {
        return self.value.bits(Self::IBRD_START, Self::IBRD_END) as u16;
    }
}

pub struct UartFbrd {
    pub value: u32,
}

impl UartFbrd {
    const FBRD_START: u32 = 5;
    const FBRD_END: u32 = 0;

    pub fn fbrd(self) -> u8 {
        return self.value.bits(Self::FBRD_START, Self::FBRD_END) as u8;
    }
}

pub struct UartLcrh {
    pub value: u32,
}

impl UartLcrh {
    const SPS: u32 = 7;
    const WLEN_START: u32 = 6;
    const WLEN_END: u32 = 5;
    const FEN: u32 = 4;
    const STP2: u32 = 3;
    const EPS: u32 = 2;
    const PEN: u32 = 1;
    const BRK: u32 = 0;

    pub fn sps(self) -> bool {
        return self.value.bit(Self::SPS);
    }

    pub fn wlen(self) -> u8 {
        return self.value.bits(Self::WLEN_START, Self::WLEN_END) as u8;
    }

    pub fn fen(self) -> bool {
        return self.value.bit(Self::FEN);
    }

    pub fn stp2(self) -> bool {
        return self.value.bit(Self::STP2);
    }

    pub fn eps(self) -> bool {
        return self.value.bit(Self::EPS);
    }

    pub fn pen(self) -> bool {
        return self.value.bit(Self::PEN);
    }

    pub fn brk(self) -> bool {
        return self.value.bit(Self::BRK);
    }
}

pub struct UartCr {
    pub value: u32,
}

impl UartCr {
    const CTSEN: u32 = 15;
    const RTSEN: u32 = 14;
    const OUT2: u32 = 13;
    const OUT1: u32 = 12;
    const RTS: u32 = 11;
    const DTR: u32 = 10;
    const RXE: u32 = 9;
    const TXE: u32 = 8;
    const LBE: u32 = 7;
    const SIRLP: u32 = 2;
    const SIREN: u32 = 1;
    const UARTEN: u32 = 0;

    pub fn ctsen(self) -> bool {
        return self.value.bit(Self::CTSEN);
    }

    pub fn rtsen(self) -> bool {
        return self.value.bit(Self::RTSEN);
    }

    pub fn out2(self) -> bool {
        return self.value.bit(Self::OUT2);
    }

    pub fn out1(self) -> bool {
        return self.value.bit(Self::OUT1);
    }

    pub fn rts(self) -> bool {
        return self.value.bit(Self::RTS);
    }

    pub fn dtr(self) -> bool {
        return self.value.bit(Self::DTR);
    }

    pub fn rxe(self) -> bool {
        return self.value.bit(Self::RXE);
    }

    pub fn txe(self) -> bool {
        return self.value.bit(Self::TXE);
    }

    pub fn lbe(self) -> bool {
        return self.value.bit(Self::LBE);
    }

    pub fn sirlp(self) -> bool {
        return self.value.bit(Self::SIRLP);
    }

    pub fn siren(self) -> bool {
        return self.value.bit(Self::SIREN);
    }

    pub fn uarten(self) -> bool {
        return self.value.bit(Self::UARTEN);
    }
}

pub struct UartIfls {
    pub value: u32,
}

impl UartIfls {
    const RXIFPSEL_START: u32 = 15;
    const RXIFPSEL_END: u32 = 15;
    const TXIFPSEL_START: u32 = 15;
    const TXIFPSEL_END: u32 = 15;
    const RXIFLSEL_START: u32 = 15;
    const RXIFLSEL_END: u32 = 15;
    const TXIFLSEL_START: u32 = 15;
    const TXIFLSEL_END: u32 = 15;

    pub fn rxifpsel(self) -> u8 {
        return self.value.bits(Self::RXIFPSEL_START, Self::RXIFPSEL_END) as u8;
    }

    pub fn txifpsel(self) -> u8 {
        return self.value.bits(Self::TXIFPSEL_START, Self::TXIFPSEL_END) as u8;
    }

    pub fn rxiflsel(self) -> u8 {
        return self.value.bits(Self::RXIFLSEL_START, Self::RXIFLSEL_END) as u8;
    }

    pub fn txiflsel(self) -> u8 {
        return self.value.bits(Self::TXIFLSEL_START, Self::TXIFLSEL_END) as u8;
    }
}

pub struct UartImsc {
    pub value: u32,
}

impl UartImsc {
    const OEIM: u32 = 10;
    const BEIM: u32 = 9;
    const PEIM: u32 = 8;
    const FEIM: u32 = 7;
    const RTIM: u32 = 6;
    const TXIM: u32 = 5;
    const RXIM: u32 = 4;
    const DSRMIM: u32 = 3;
    const DCDMIM: u32 = 2;
    const CTSMIM: u32 = 1;
    const RIMIM: u32 = 0;

    pub fn oeim(self) -> bool {
        return self.value.bit(Self::OEIM);
    }

    pub fn beim(self) -> bool {
        return self.value.bit(Self::BEIM);
    }

    pub fn peim(self) -> bool {
        return self.value.bit(Self::PEIM);
    }

    pub fn feim(self) -> bool {
        return self.value.bit(Self::FEIM);
    }

    pub fn rtim(self) -> bool {
        return self.value.bit(Self::RTIM);
    }

    pub fn txim(self) -> bool {
        return self.value.bit(Self::TXIM);
    }

    pub fn rxim(self) -> bool {
        return self.value.bit(Self::RXIM);
    }

    pub fn dsrmim(self) -> bool {
        return self.value.bit(Self::DSRMIM);
    }

    pub fn dcdmim(self) -> bool {
        return self.value.bit(Self::DCDMIM);
    }

    pub fn ctsmim(self) -> bool {
        return self.value.bit(Self::CTSMIM);
    }

    pub fn rimim(self) -> bool {
        return self.value.bit(Self::RIMIM);
    }
}

pub struct UartRis {
    pub value: u32,
}

impl UartRis {
    const OERIS: u32 = 10;
    const BERIS: u32 = 9;
    const PERIS: u32 = 8;
    const FERIS: u32 = 7;
    const RTRIS: u32 = 6;
    const TXRIS: u32 = 5;
    const RXRIS: u32 = 4;
    const DSRRMIS: u32 = 3;
    const DCDRMIS: u32 = 2;
    const CTSRMIS: u32 = 1;
    const RIRMIS: u32 = 0;

    pub fn oeris(self) -> bool {
        return self.value.bit(Self::OERIS);
    }

    pub fn beris(self) -> bool {
        return self.value.bit(Self::BERIS);
    }

    pub fn peris(self) -> bool {
        return self.value.bit(Self::PERIS);
    }

    pub fn feris(self) -> bool {
        return self.value.bit(Self::FERIS);
    }

    pub fn rtris(self) -> bool {
        return self.value.bit(Self::RTRIS);
    }

    pub fn txris(self) -> bool {
        return self.value.bit(Self::TXRIS);
    }

    pub fn rxris(self) -> bool {
        return self.value.bit(Self::RXRIS);
    }

    pub fn dsrrmis(self) -> bool {
        return self.value.bit(Self::DSRRMIS);
    }

    pub fn dcdrmis(self) -> bool {
        return self.value.bit(Self::DCDRMIS);
    }

    pub fn ctsrmis(self) -> bool {
        return self.value.bit(Self::CTSRMIS);
    }

    pub fn rirmis(self) -> bool {
        return self.value.bit(Self::RIRMIS);
    }
}

pub struct UartMis {
    pub value: u32,
}

impl UartMis {
    const OEMIS: u32 = 10;
    const BEMIS: u32 = 9;
    const PEMIS: u32 = 8;
    const FEMIS: u32 = 7;
    const RTMIS: u32 = 6;
    const TXMIS: u32 = 5;
    const RXMIS: u32 = 4;
    const DSRMMIS: u32 = 3;
    const DCDMMIS: u32 = 2;
    const CTSMMIS: u32 = 1;
    const RIMMIS: u32 = 0;

    pub fn oemis(self) -> bool {
        return self.value.bit(Self::OEMIS);
    }

    pub fn bemis(self) -> bool {
        return self.value.bit(Self::BEMIS);
    }

    pub fn pemis(self) -> bool {
        return self.value.bit(Self::PEMIS);
    }

    pub fn femis(self) -> bool {
        return self.value.bit(Self::FEMIS);
    }

    pub fn rtmis(self) -> bool {
        return self.value.bit(Self::RTMIS);
    }

    pub fn txmis(self) -> bool {
        return self.value.bit(Self::TXMIS);
    }

    pub fn rxmis(self) -> bool {
        return self.value.bit(Self::RXMIS);
    }

    pub fn dsrmmis(self) -> bool {
        return self.value.bit(Self::DSRMMIS);
    }

    pub fn dcdmmis(self) -> bool {
        return self.value.bit(Self::DCDMMIS);
    }

    pub fn ctsmmis(self) -> bool {
        return self.value.bit(Self::CTSMMIS);
    }

    pub fn rimmis(self) -> bool {
        return self.value.bit(Self::RIMMIS);
    }
}

pub struct UartIcr {
    pub value: u32,
}

impl UartIcr {
    const OEIC: u32 = 10;
    const BEIC: u32 = 9;
    const PEIC: u32 = 8;
    const FEIC: u32 = 7;
    const RTIC: u32 = 6;
    const TXIC: u32 = 5;
    const RXIC: u32 = 4;
    const DSRMIC: u32 = 3;
    const DCDMIC: u32 = 2;
    const CTSMIC: u32 = 1;
    const RIMIC: u32 = 0;

    pub fn oeic(self) -> bool {
        return self.value.bit(Self::OEIC);
    }

    pub fn beic(self) -> bool {
        return self.value.bit(Self::BEIC);
    }

    pub fn peic(self) -> bool {
        return self.value.bit(Self::PEIC);
    }

    pub fn feic(self) -> bool {
        return self.value.bit(Self::FEIC);
    }

    pub fn rtic(self) -> bool {
        return self.value.bit(Self::RTIC);
    }

    pub fn txic(self) -> bool {
        return self.value.bit(Self::TXIC);
    }

    pub fn rxic(self) -> bool {
        return self.value.bit(Self::RXIC);
    }

    pub fn dsrmic(self) -> bool {
        return self.value.bit(Self::DSRMIC);
    }

    pub fn dcdmic(self) -> bool {
        return self.value.bit(Self::DCDMIC);
    }

    pub fn ctsmic(self) -> bool {
        return self.value.bit(Self::CTSMIC);
    }

    pub fn rimic(self) -> bool {
        return self.value.bit(Self::RIMIC);
    }
}

pub struct UartDmacr {
    pub value: u32,
}

impl UartDmacr {
    const DMAONERR: u32 = 2;
    const TXDMAE: u32 = 1;
    const RXDMAE: u32 = 0;

    pub fn dmaonerr(self) -> bool {
        return self.value.bit(Self::DMAONERR);
    }

    pub fn txdmae(self) -> bool {
        return self.value.bit(Self::TXDMAE);
    }

    pub fn rxdmae(self) -> bool {
        return self.value.bit(Self::RXDMAE);
    }
}

pub struct UartItcr {
    pub value: u32,
}

impl UartItcr {
    const ITCR1: u32 = 1;
    const ITCR0: u32 = 0;

    pub fn itcr1(self) -> bool {
        return self.value.bit(Self::ITCR1);
    }

    pub fn itcr0(self) -> bool {
        return self.value.bit(Self::ITCR0);
    }
}

pub struct UartItip {
    pub value: u32,
}

impl UartItip {
    const ITIP3: u32 = 3;
    const ITIP0: u32 = 0;

    pub fn itip3(self) -> bool {
        return self.value.bit(Self::ITIP3);
    }

    pub fn itip0(self) -> bool {
        return self.value.bit(Self::ITIP0);
    }
}

pub struct UartItop {
    pub value: u32,
}

impl UartItop {
    const ITOP11: u32 = 11;
    const ITOP10: u32 = 10;
    const ITOP9: u32 = 9;
    const ITOP8: u32 = 8;
    const ITOP7: u32 = 7;
    const ITOP6: u32 = 6;
    const ITOP3: u32 = 3;
    const ITOP0: u32 = 0;

    pub fn itop11(self) -> bool {
        return self.value.bit(Self::ITOP11);
    }

    pub fn itop10(self) -> bool {
        return self.value.bit(Self::ITOP10);
    }

    pub fn itop9(self) -> bool {
        return self.value.bit(Self::ITOP9);
    }

    pub fn itop8(self) -> bool {
        return self.value.bit(Self::ITOP8);
    }

    pub fn itop7(self) -> bool {
        return self.value.bit(Self::ITOP7);
    }

    pub fn itop6(self) -> bool {
        return self.value.bit(Self::ITOP6);
    }

    pub fn itop3(self) -> bool {
        return self.value.bit(Self::ITOP3);
    }

    pub fn itop0(self) -> bool {
        return self.value.bit(Self::ITOP0);
    }
}

pub struct UartTdr {
    pub value: u32,
}

impl UartTdr {
    const TDR10_0_START: u32 = 11;
    const TDR10_0_END: u32 = 11;

    pub fn tdr10_0(self) -> u16 {
        return self.value.bits(Self::TDR10_0_START, Self::TDR10_0_END) as u16;
    }
}
