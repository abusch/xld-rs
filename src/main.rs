#![allow(dead_code)]
extern crate x11;
extern crate clap;
#[macro_use]
extern crate failure;

mod edid;
mod layout;
mod mode;
mod monitors;
mod output;
mod pos;
mod settings;
mod xrandrutils;

use clap::{App, Arg};

use settings::Settings;

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

    if let Err(e) = layout::layout(settings) {
        eprintln!("FAIL: {}", e);
    }
}
