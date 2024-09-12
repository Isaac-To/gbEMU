mod hardware;
use hardware::mem::ROMAccess;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut memory = hardware::mem::Memory::new();
    let mut sys = hardware::System::new(&mut memory);
    memory.load_rom(args[1].as_str());
    println!("{:?}", memory.get_metadata());
    sys.run();
}
