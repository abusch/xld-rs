use std::fs::{read_dir, File};
use std::io::Read;

use failure::Error;

pub struct Monitors {
    pub laptop_lid_closed: bool,
}

impl Monitors {
    pub fn new() -> Monitors {
        Monitors {
            laptop_lid_closed: Self::calculate_laptop_lid_closed().unwrap_or(false),
        }
    }

    fn calculate_laptop_lid_closed() -> Result<bool, Error> {
        const LAPTOP_LID_ROOT_PATH: &str = "/proc/acpi/button/lid";
        let readdir = read_dir(LAPTOP_LID_ROOT_PATH)?;
        for entry in readdir {
            if let Ok(entry) = entry {
                if entry.file_name().as_os_str() == "." || entry.file_name().as_os_str() == ".." {
                    continue;
                }
                let mut path = entry.path();
                path.push("state");
                let mut lid_file = File::open(path)?;
                let mut buf = String::new();
                lid_file.read_to_string(&mut buf)?;
                if buf.contains("closed") {
                    return Ok(true);
                }
            } else {
                // TODO log
            }
        }
        Ok(false)
    }
}

