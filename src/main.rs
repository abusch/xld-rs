#![allow(dead_code)]
extern crate x11;
extern crate clap;
extern crate failure;

mod edid;
mod mode;
mod output;
mod xrandrutils;

use std::fs::{read_dir, File};
use std::io::Read;

use failure::Error;


struct Monitors {
    laptop_lid_closed: bool,
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

pub struct Pos {
    x: i32,
    y: i32,
}

struct Settings {
    info: bool,
    noop: bool,
    mirror: bool,
    order: Vec<String>,
    primary: String,
    quiet: bool,
}

fn layout(settings: Settings) -> Result<(), Error> {
    // discover monitors
    let monitors = Monitors::new();

    // discover outputs
    

    // output verbose information
    // current info is all output, we're done
    // Order the outputs if the user wishes
    // activate outputs and determine primary
    // arrange mirrored or left to right
    // determine DPI from the primary
    // render desired command
    // execute

    Ok(())
}

fn main() {
    println!("Hello, world!");
}
