pub trait Uart {
    fn init(&mut self);
    fn write(&mut self, b: u8);
    fn try_read(&mut self) -> Option<u8>;
    fn uninit(&mut self);
}
