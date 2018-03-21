use std::sync::Arc;
use std::ffi::CStr;
use std::ptr;

use x11::xlib;
use x11::xrandr;

use super::{Pos};
use output::{Output, OutputState};
use mode::Mode;

pub fn discover_outputs() -> Vec<Arc<Output>> {
    unsafe {
        // Get the display and root window
        let display = xlib::XOpenDisplay(ptr::null()).as_mut().expect("display was null!");
        let default_screen = xlib::XDefaultScreen(display);
        let root_window = xlib::XRootWindow(display, default_screen);

        // Get xrandr resources
        let screen_resources = xrandr::XRRGetScreenResources(display, root_window).as_mut().expect("screen resources were null!");
        for i in 0..screen_resources.noutput {
            let rr_output = *screen_resources.outputs.offset(i as isize);
            let output_info = xrandr::XRRGetOutputInfo(display, screen_resources, rr_output).as_mut().expect("output info was null!");
            let name = CStr::from_ptr(output_info.name);
            let mut state;
            let current_pos;
            let current_mode;
            let mut rr_mode = 0;
            if output_info.crtc != 0 {
                // active outputs have crtc info
                state = OutputState::Active;
                // current position and mode
                let crtc_info = xrandr::XRRGetCrtcInfo(display, screen_resources, (*output_info).crtc);
                current_pos = Pos { x: (*crtc_info).x, y: (*crtc_info).y };
                rr_mode = (*crtc_info).mode;
                current_mode = mode_from_xrr(rr_mode, screen_resources);

                if output_info.nmode == 0 {
                    // output is active but disconnected
                    state = OutputState::Disconnected;
                }
            } else if output_info.nmode != 0 {
                // innactive connected outputs have modes
                state = OutputState::Connected;
            } else {
                state = OutputState::Disconnected;
            }

            // iterate all properties to find EDID; XRRQueryOutputProperty fails when queried with XInternAtom
            // TODO finish

        }

    }

    Vec::new()
}

unsafe fn mode_from_xrr(id: xrandr::RRMode, resources: &xrandr::XRRScreenResources) -> Mode {
    let mut mode_info = None;
    for i in 0..resources.nmode {
        let mode = *resources.modes.offset(i as isize);
        if id == mode.id {
            mode_info = Some(mode);
            break;
        }
    }

    mode_info
        .map(|mode_info| Mode {rr_mode: id, width: mode_info.width, height: mode_info.height, refresh: refresh_from_mode_info(&mode_info)})
        .expect("cannot construct mode: cannot retrieve RRMode")
}

fn refresh_from_mode_info(mode_info: &xrandr::XRRModeInfo) -> u32 {
    let mut v_total = mode_info.vTotal as f64;

    if mode_info.modeFlags & (xrandr::RR_DoubleScan as u64) != 0 {
        v_total *= 2.0;
    }
    if mode_info.modeFlags & (xrandr::RR_Interlace as u64) != 0 {
        v_total /= 2.0;
    }

    let rate = if mode_info.hTotal != 0 && v_total != 0.0 {
        mode_info.dotClock as f64 / (mode_info.hTotal as f64 * v_total)
    } else {
        0.0
    };

    rate as u32
}
