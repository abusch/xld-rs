#![allow(dead_code)]
extern crate x11;
extern crate clap;
#[macro_use]
extern crate failure;

mod edid;
mod mode;
mod monitors;
mod output;
mod xrandrutils;

use monitors::Monitors;

use clap::{App, Arg};
use failure::Error;

pub struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug, Default)]
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
    let current_outputs = xrandrutils::discover_outputs();
    if current_outputs.is_empty() {
        bail!("no outputs found");
    }

    // output verbose information
    if !settings.quiet || settings.info {
        for output in &current_outputs {
            println!("{}", output);
        }
        println!();
        println!("laptop lid {}", if monitors.laptop_lid_closed { "closed" } else { "open or not present" });
    }

    // current info is all output, we're done
    if settings.info {
        return Ok(());
    }
    // Order the outputs if the user wishes
    // activate outputs and determine primary
    // arrange mirrored or left to right
    // determine DPI from the primary
    // render desired command
    // execute

    Ok(())
}

fn main() {
    let matches = App::new("xld-rs")
        .version("0.1")
        .author("Antoine Busch <antoine.busch@gmail.com>")
        .about("Arranges outputs in a left to right manner, using highest resolution and refresh.\nDPI is calculated exactly based on the first or primary output's EDID information.\nLaptop outputs are turned off when the lid is closed.\n\ne.g. xld-rs ...")
        .arg(Arg::with_name("quiet")
             .help("")
             .short("q")
             .long("quiet"))
        .arg(Arg::with_name("info")
             .help("")
             .short("i")
             .long("info"))
        .arg(Arg::with_name("mirror")
             .help("")
             .short("m")
             .long("mirror"))
        .get_matches();

    let settings = Settings {
        info: matches.is_present("info"),
        quiet: matches.is_present("quiet"),
        mirror: matches.is_present("mirror"),
        ..Settings::default()
    };

    if let Err(e) = layout(settings) {
        eprintln!("FAIL: {}", e);
    }
}
