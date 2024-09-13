pub trait RWAccess {
    fn read_8(&self, addr: u16) -> u8;
    fn write_8(&mut self, addr: u16, val: u8);
    fn read_16(&self, addr: u16) -> u16;
    fn write_16(&mut self, addr: u16, val: u16);
}
