use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};

use x11::xrandr::RRMode;

#[derive(Debug, Clone, Eq)]
pub struct Mode {
    pub rr_mode: RRMode,
    pub width: u32,
    pub height: u32,
    pub refresh: u32,
}

impl Ord for Mode {
    fn cmp(&self, other: &Mode) -> Ordering {
        if self.width == other.width {
            if self.height == other.height {
                self.refresh.cmp(&other.refresh)
            } else {
                self.height.cmp(&other.height)
            }
        } else {
            self.width.cmp(&other.width)
        }
    }
}

impl PartialOrd for Mode {
    fn partial_cmp(&self, other: &Mode) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for Mode {
    fn eq(&self, other: &Mode) -> bool {
        self.cmp(&other) == Ordering::Equal
    }
}
