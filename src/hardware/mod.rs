pub mod cpu;
pub mod mem;
pub mod rw;

use cpu::Execution;
use mem::Memory;

/// System struct
pub struct System {
    /// CPU
    cpu: cpu::CPU,

    /// Memory
    memory: *mut mem::Memory,
}

impl System {
    /// Create a new System
    pub fn new(mem_ptr: &mut Memory) -> System {
        System {
            cpu: cpu::CPU::new(mem_ptr),
            memory: mem_ptr,
        }
    }

    /// Run the system
    pub fn run(&mut self) {
        // let clock_interval = 477; // native nanoseconds
        let clock_interval = 1000000000; // debug nanoseconds
        loop {
            let now = std::time::Instant::now();
            self.cpu.step();
            let elapsed = now.elapsed();
            if elapsed.as_nanos() < clock_interval {
                std::thread::sleep(std::time::Duration::from_nanos(
                    (clock_interval - elapsed.as_nanos()) as u64,
                ));
                // println!("Sleeping for: {} ns", clock_interval - elapsed.as_nanos());
            }
        }
    }
}
