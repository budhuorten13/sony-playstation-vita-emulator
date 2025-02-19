use std::fs;
use std::path::Path;

pub struct Emulator {
    rom_path: String,
}

impl Emulator {
    pub fn new(rom_path: String) -> Self {
        Emulator { rom_path }
    }

    pub fn load_rom(&self) -> Result<(), String> {
        if Path::new(&self.rom_path).exists() {
            Ok(())
        } else {
            Err("ROM file not found".to_string())
        }
    }

    pub fn start(&self) {
        // Logic to start the emulator
    }
}