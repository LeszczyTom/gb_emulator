use std::path::PathBuf;

const MEMORY_SIZE: usize = u16::MAX as usize + 1;

pub struct MMU {
    pub(crate) mem: [u8; MEMORY_SIZE],

    #[cfg(test)]
    pub(crate) serial_output: [u8; 6],
}

impl Default for MMU {
    fn default() -> MMU {
        MMU {
            mem: [0; MEMORY_SIZE],

            #[cfg(test)]
            serial_output: [0; 6],
        }
    }
}

impl MMU {
    pub fn get(&self, address: u16) -> u8 {
        self.mem[address as usize]
    }

    pub fn set(&mut self, value: u8, address: u16) {
        match address {
            0xFF01 => self.handle_serial(value),
            0xFF04 => {
                // Reset timer registers if writting to DIV
                self.mem[0xFF04] = 0;
                self.mem[0xFF05] = 0;
            }
            0xFF44 => {
                // Reset LY register if writting to it
                self.mem[0xFF44] = 0;
            }
            _ => (),
        }

        self.mem[address as usize] = value;
    }

    fn handle_serial(&mut self, value: u8) {
        print!("{}", value as char);

        #[cfg(test)]
        {
            self.serial_output.rotate_left(1);
            *self.serial_output.last_mut().unwrap() = value;
        }
    }

    pub fn set_interrupt_flag(&mut self, flag: u8) {
        self.mem[0xFF0F] |= 1 << flag;
    }

    pub fn load_file(
        &mut self,
        file_path: PathBuf,
    ) -> Result<(), Box<dyn std::error::Error + 'static>> {
        let rom = std::fs::read(file_path)?;
        self.mem[0..0x8000].copy_from_slice(&rom);
        Ok(())
    }
}
