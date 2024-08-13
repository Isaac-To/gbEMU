use std::fs;

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
    fn new(location: &str) -> Self;
    fn get_metadata(&self) -> ROMMetadata;
}

/// Implement ROMAccess for ROM
impl ROMAccess for ROM {
    /// Create a new ROM from a file
    /// Reads file and loads metadata & data
    /// Will also check if header checksum is correct
    /// Global checksum is not checked
    /// Documented at: https://gbdev.gg8.se/wiki/articles/The_Cartridge_Header#014D_-_Header_Checksum
    fn new(location: &str) -> Self {
        let data = fs::read(location).unwrap();
        let metadata = ROMMetadata {
            // ASCII is valid utf8
            title: String::from_utf8(data[0x134..0x143].to_vec()).unwrap(),
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
        if checksum != metadata.header_checksum {
            panic!("Invalid ROM: Checksum failed");
        }
        ROM {
            location: location.to_string(),
            loaded: false,
            data: Vec::new(),
            metadata: metadata,
        }
    }
    /// Get the metadata of the ROM (READONLY)
    fn get_metadata(&self) -> ROMMetadata {
        self.metadata.clone()
    }
}
