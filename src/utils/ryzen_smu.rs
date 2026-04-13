use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;

const SMU_DRIVER_PATH: &str = "/sys/kernel/ryzen_smu_drv";

#[derive(Debug)]
pub enum SmuError {
    Io(io::Error),
    DriverNotLoaded,
    CommandFailed(u32),
    Timeout,
    InvalidSize,
}

impl From<io::Error> for SmuError {
    fn from(err: io::Error) -> Self {
        SmuError::Io(err)
    }
}

pub struct RyzenSmu {
    path: String,
}

impl RyzenSmu {
    pub fn new() -> Result<Self, SmuError> {
        if !Path::new(SMU_DRIVER_PATH).exists() {
            return Err(SmuError::DriverNotLoaded);
        }
        Ok(Self {
            path: SMU_DRIVER_PATH.to_string(),
        })
    }

    pub fn is_supported() -> bool {
        Path::new(SMU_DRIVER_PATH).exists()
    }

    pub fn get_driver_version(&self) -> Result<String, SmuError> {
        self.read_string_file("drv_version")
    }

    pub fn get_smu_version(&self) -> Result<String, SmuError> {
        self.read_string_file("version")
    }

    pub fn get_codename(&self) -> Result<u32, SmuError> {
        let s = self.read_string_file("codename")?;
        s.trim().parse::<u32>().map_err(|_| {
            SmuError::Io(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid codename format",
            ))
        })
    }

    fn read_string_file(&self, filename: &str) -> Result<String, SmuError> {
        let path = format!("{}/{}", self.path, filename);
        let content = fs::read_to_string(path)?;
        Ok(content.trim().to_string())
    }

    /// Sends a command to the SMU.
    /// `args` should be an array of 6 u32 values.
    /// Returns the result status code.
    pub fn send_command(&self, cmd_id: u32, args: &mut [u32; 6]) -> Result<u32, SmuError> {
        // 1. Write arguments to smu_args
        let args_path = format!("{}/smu_args", self.path);
        let mut args_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&args_path)?;

        let mut buf = [0u8; 24];
        for (i, arg) in args.iter().enumerate() {
            let bytes = arg.to_le_bytes();
            buf[i * 4..(i + 1) * 4].copy_from_slice(&bytes);
        }

        args_file.write_all(&buf)?;

        // 2. Write command ID to rsmu_cmd (or mp1_smu_cmd if rsmu not present)
        // The driver documentation says rsmu_cmd is not present on Vangogh, so we might need fallback.
        // For now, let's try rsmu_cmd first, then mp1_smu_cmd.
        let cmd_path = if Path::new(&format!("{}/rsmu_cmd", self.path)).exists() {
            format!("{}/rsmu_cmd", self.path)
        } else {
            format!("{}/mp1_smu_cmd", self.path)
        };

        let mut cmd_file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&cmd_path)?;

        // Write command ID (as string or binary? README says "printf '\x53' | sudo tee ...")
        // The README example uses binary write for smu_args, but for rsmu_cmd it pipes printf output.
        // Wait, `printf '\x53'` outputs a single byte 0x53 (ASCII 'S').
        // But the driver docs say "accepts either an 8-bit or 32-bit command ID".
        // And "When this file is read, it produces the result... as a 32 bit little-endian encoded value."
        // Let's try writing the u32 as little endian bytes.
        
        cmd_file.write_all(&cmd_id.to_le_bytes())?;

        // 3. Read back status
        // We need to read from the cmd file to get the status.
        // The driver blocks on read until command completes (or timeouts).
        
        // Rewind to start to read the result
        cmd_file.seek(SeekFrom::Start(0))?;
        
        let mut result_buf = [0u8; 4];
        cmd_file.read_exact(&mut result_buf)?;
        let status = u32::from_le_bytes(result_buf);

        // 4. Read back arguments (results)
        args_file.seek(SeekFrom::Start(0))?;
        args_file.read_exact(&mut buf)?;
        
        for i in 0..6 {
            let bytes: [u8; 4] = buf[i * 4..(i + 1) * 4].try_into().unwrap();
            args[i] = u32::from_le_bytes(bytes);
        }

        match status {
            1 => Ok(status), // OK
            0 => Ok(status), // WAITING (Should not happen on blocking read?)
            _ => Err(SmuError::CommandFailed(status)),
        }
    }

    pub fn read_smn(&self, address: u32) -> Result<u32, SmuError> {
        let path = format!("{}/smn", self.path);
        let mut file = OpenOptions::new().read(true).write(true).open(path)?;

        // Write address (32-bit)
        file.write_all(&address.to_le_bytes())?;

        // Read result (32-bit)
        file.seek(SeekFrom::Start(0))?;
        let mut buf = [0u8; 4];
        file.read_exact(&mut buf)?;
        
        Ok(u32::from_le_bytes(buf))
    }

    pub fn write_smn(&self, address: u32, value: u32) -> Result<(), SmuError> {
        let path = format!("{}/smn", self.path);
        let mut file = OpenOptions::new().write(true).open(path)?;

        // Write address (32-bit) + value (32-bit) = 64 bits
        let mut buf = [0u8; 8];
        buf[0..4].copy_from_slice(&address.to_le_bytes());
        buf[4..8].copy_from_slice(&value.to_le_bytes());

        file.write_all(&buf)?;
        Ok(())
    }

    pub fn read_pm_table(&self) -> Result<Vec<f32>, SmuError> {
        let path = format!("{}/pm_table", self.path);
        let mut file = File::open(path)?;
        
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;

        // Convert bytes to f32 (little endian)
        let mut values = Vec::new();
        for chunk in buf.chunks_exact(4) {
            let bytes: [u8; 4] = chunk.try_into().unwrap();
            values.push(f32::from_le_bytes(bytes));
        }

        Ok(values)
    }
}
