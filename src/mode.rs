use x11::xrandr::RRMode;

pub struct Mode {
    pub rr_mode: RRMode,
    pub width: u32,
    pub height: u32,
    pub refresh: u32,
}

