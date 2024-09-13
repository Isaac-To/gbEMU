use super::{
    cpu::opcodes::{cb_prefixed_opcode_get, unprefixed_opcode_get, Opcode},
    rw::RWAccess,
};
use std::fs;

pub struct Memory {
    rom_metadata: ROMMetadata,
    data: [u8; 0xFFFF],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            rom_metadata: ROMMetadata {
                // ASCII is valid utf8
                title: String::new(),
                cgb: 0,
                new_licensee_code: 0,
                sgb: 0,
                cartridge_type: 0,
                rom_size: 0,
                ram_size: 0,
                destination_code: 0,
                old_licensee_code: 0,
                mask_rom_version: 0,
                header_checksum: 0,
                global_checksum: 0,
            },
            data: [0; 0xFFFF],
        }
    }
}

/// Implement MemoryAccess for System
impl RWAccess for Memory {
    /// Read 8-bit value from memory
    fn read_8(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }
    /// Read 16-bit value from memory
    fn read_16(&self, addr: u16) -> u16 {
        self.read_8(addr) as u16 | (self.read_8(addr + 1) as u16) << 8
    }
    /// Write 8-bit value to memory
    fn write_8(&mut self, addr: u16, value: u8) {
        self.data[addr as usize] = value;
    }
    /// Write 16-bit value to memory
    fn write_16(&mut self, addr: u16, value: u16) {
        self.write_8(addr, value as u8);
        self.write_8(addr + 1, (value >> 8) as u8);
    }
}

#[derive(Clone, Debug)]
/// ROM Metadata
/// This struct contains all the metadata of a ROM
/// This is used to display information about the ROM
/// and to check if the ROM is valid
///
/// # Fields
/// * `title` - The title of the ROM [0x134..0x143] shares 0x143 with CGB Flag
/// * `cgb` - CGB Flag [0x143]
/// * `new_licensee_code` - New Licensee Code [0x144]
/// * `sgb` - SGB Flag [0x146]
/// * `cartridge_type` - Cartridge Type [0x147]
/// * `rom_size` - ROM Size [0x148]
/// * `ram_size` - RAM Size [0x149]
/// * `destination_code` - Destination Code [0x14A]
/// * `old_licensee_code` - Old Licensee Code [0x14B]
/// * `mask_rom_version` - Mask ROM Version [0x14C]
/// * `header_checksum` - Header Checksum [0x14D]
/// * `global_checksum` - Global Checksum [0x14E..0x14F]
pub struct ROMMetadata {
    title: String,
    cgb: u8,
    new_licensee_code: u8,
    sgb: u8,
    cartridge_type: u8,
    rom_size: u8,
    ram_size: u8,
    destination_code: u8,
    old_licensee_code: u8,
    mask_rom_version: u8,
    header_checksum: u8,
    global_checksum: u16,
}

/// ROM struct
pub struct ROM {
    location: String,
    loaded: bool,
    data: Vec<u8>,
    metadata: ROMMetadata,
}

/// ROMAccess trait for ROM
/// This trait is used to access ROMs
pub trait ROMAccess {
    fn load_rom(&mut self, location: &str) -> ();
    fn get_metadata(&mut self) -> ROMMetadata;
    fn decode(&mut self, addr: u16) -> (u16, Opcode);
}

/// Implement ROMAccess for ROM
impl ROMAccess for Memory {
    /// Create a new ROM from a file
    /// Reads file and loads metadata & data
    /// Will also check if header checksum is correct
    /// Global checksum is not checked
    /// Documented at: https://gbdev.gg8.se/wiki/articles/The_Cartridge_Header#014D_-_Header_Checksum
    fn load_rom(&mut self, location: &str) {
        let data: Vec<u8> =
            fs::read(location).expect(&format!("ERROR: Could not find rom at {}", location));
        self.rom_metadata = ROMMetadata {
            // ASCII is valid utf8
            title: String::from_utf8(data[0x134..0x143].to_vec())
                .expect(&format!("ERROR: Could not read title")),
            cgb: data[0x143],
            new_licensee_code: data[0x144],
            sgb: data[0x146],
            cartridge_type: data[0x147],
            rom_size: data[0x148],
            ram_size: data[0x149],
            destination_code: data[0x14A],
            old_licensee_code: data[0x14B],
            mask_rom_version: data[0x14C],
            header_checksum: data[0x14D],
            global_checksum: u16::from_le_bytes([data[0x14E], data[0x14F]]),
        };
        let mut checksum: u8 = 0;
        for i in 0x134..0x14D {
            checksum = checksum.wrapping_sub(data[i]).wrapping_sub(1);
        }
        if checksum != self.rom_metadata.header_checksum {
            panic!("Invalid ROM: Checksum failed");
        }
        // Copy the data into the main memory storage (the "first" 16 KiB chunk)
        for i in 0x00..0x3FFF {
            self.data[i] = data[i];
        }
    }

    /// Get Metadata from ROM
    fn get_metadata(&mut self) -> ROMMetadata {
        return self.rom_metadata.clone();
    }

    /// Decodes the instruction at address and returns end of address and the decoded instruction
    /// If used for linear progression through a program, it is expected that the caller function updates it's own address counter with the one outputted by this function
    /// Example for reading the instructions between 180 and 190:
    /// ```rust
    /// let mut addr = 0x180 as usize;
    /// while addr < 0x190 {
    ///    let (n_addr, opcode) = rom.decode(addr);
    ///    println!("{:x}\t {}", addr, opcode);
    ///    addr = n_addr;
    /// }
    /// ```
    fn decode(&mut self, addr: u16) -> (u16, Opcode) {
        let mut addr = addr; // Create a local copy for returning
        let mut opcode = self.read_8(addr);
        let mut instruction = unprefixed_opcode_get(&opcode);
        addr += 1; // Acknowledge reading of instruction
                   // SPECIAL CASE: 0xCB (CB Prefix) -> Lookup and execute following instruction from CB prefixed
        if opcode == 0xCB {
            opcode = self.read_8(addr);
            instruction = cb_prefixed_opcode_get(&opcode);
            addr += 1; // Acknowledge reading of CB instruction
        }
        // Patch in immediate values by reading following addresses from the ROM
        for i in 0..instruction.operands.len() {
            if instruction.operands[i].bytes != 0 {
                let mut bytes_vector = Vec::new();
                for i in addr..addr + instruction.operands[i].bytes as u16 {
                    bytes_vector.push(self.read_8(i));
                }
                // To allow for larger than 8 bit immediates (is formatted as 8 bit chunks and reconstructed for storage in u16), applies a big-endian concat
                let mut sum = 0 as u16;
                for bytes in bytes_vector.into_iter().rev() {
                    sum = sum << 8 | bytes as u16;
                }
                instruction.operands[i].value = sum;
                addr += instruction.operands[i].bytes as u16 // Acknowledge reading of following bytes
            } // ELSE skip as it's not an immediate
        }
        return (addr, instruction); // Address should be updated in caller to prevent double execution of instructions
    }
}
