use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};

use x11::xrandr::RRMode;

#[derive(Debug, Clone, Eq)]
pub struct Mode {
    pub rr_mode: RRMode,
    pub width: u32,
    pub height: u32,
    pub refresh: u32,
}

impl Mode {
    pub fn new(rr_mode: RRMode, width: u32, height: u32, refresh: u32) -> Mode {
        Mode {
            rr_mode,
            width,
            height,
            refresh,
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_width() {
        assert!(Mode::new(0, 1, 2, 2) < Mode::new(0, 2, 1, 1));
    }
    #[test]
    fn order_height() {
        assert!(Mode::new(0, 1, 1, 2) < Mode::new(0, 1, 2, 1));
    }
    #[test]
    fn order_refresh() {
        assert!(Mode::new(0, 1, 1, 1) < Mode::new(0, 1, 1, 2));
    }
}
