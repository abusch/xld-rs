use std::sync::Arc;
use std::ffi::CStr;
use std::os::raw::{c_int, c_ulong, c_uchar};
use std::ptr;

use x11::xlib;
use x11::xrandr;

use pos::Pos;
use edid::Edid;
use output::{Output, OutputState};
use mode::Mode;

pub fn discover_outputs() -> Vec<Arc<Output>> {
    unsafe {
        // Get the display and root window
        let display = xlib::XOpenDisplay(ptr::null()).as_mut().expect("display was null!");
        let default_screen = xlib::XDefaultScreen(display);
        let root_window = xlib::XRootWindow(display, default_screen);
        let mut outputs = Vec::new();

        // Get xrandr resources
        let screen_resources = xrandr::XRRGetScreenResources(display, root_window).as_mut().expect("screen resources were null!");
        for i in 0..screen_resources.noutput {
            let rr_output = *screen_resources.outputs.offset(i as isize);
            let output_info = xrandr::XRRGetOutputInfo(display, screen_resources, rr_output).as_mut().expect("output info was null!");
            let name = CStr::from_ptr(output_info.name);
            let mut current_pos = None;
            let mut current_mode = None;
            let mut edid = None;

            let state = if output_info.crtc != 0 {
                // active outputs have crtc info
                // current position and mode
                let crtc_info = xrandr::XRRGetCrtcInfo(display, screen_resources, (*output_info).crtc);
                current_pos = Some(Pos { x: (*crtc_info).x, y: (*crtc_info).y });
                let rr_mode = (*crtc_info).mode;
                current_mode = Some(Arc::new(mode_from_xrr(rr_mode, screen_resources)));

                if output_info.nmode == 0 {
                    // output is active but disconnected
                    OutputState::Disconnected
                } else {
                    OutputState::Active
                }
            } else if output_info.nmode != 0 {
                // innactive connected outputs have modes
                OutputState::Connected
            } else {
                OutputState::Disconnected
            };

            // iterate all properties to find EDID; XRRQueryOutputProperty fails when queried with XInternAtom
            let mut nprop: c_int = 0;
            let atoms = xrandr::XRRListOutputProperties(display, rr_output, &mut nprop);
            for i in 0..nprop {
                let atom = *atoms.offset(i as isize);
                let atom_name = CStr::from_ptr(xlib::XGetAtomName(display, atom));
                if atom_name.to_string_lossy() == xrandr::RR_PROPERTY_RANDR_EDID {
                    let mut actual_type: xlib::Atom = 0;
                    let mut actual_format: c_int = 0;
                    let mut nitems: c_ulong = 0;
                    let mut bytes_after: c_ulong = 0;
                    let mut props: *mut c_uchar = ptr::null_mut();
                    xrandr::XRRGetOutputProperty(display, rr_output, atom,
                                                 0,
                                                 64,
                                                 false as xlib::Bool,
                                                 false as xlib::Bool,
                                                 xlib::AnyPropertyType as xlib::Atom,
                                                 &mut actual_type,
                                                 &mut actual_format,
                                                 &mut nitems,
                                                 &mut bytes_after,
                                                 &mut props,
                                                 );

                    // Convert edid to a slice
                    let edid_slice = ::std::slice::from_raw_parts(props, nitems as usize);
                    edid = Some(Edid::new(edid_slice, name.to_string_lossy().as_ref()));

                    break;
                }
            }

            // Add available modes
            let mut preferred_mode_idx = 0;
            let mut modes = Vec::new();
            for j in 0..output_info.nmode {
                let mode = Arc::new(mode_from_xrr(*output_info.modes.offset(j as isize), screen_resources));
                modes.push(mode);
                if output_info.npreferred == j + 1 {
                    preferred_mode_idx = j;
                }
            }
            let preferred_mode = modes.get(preferred_mode_idx as usize).cloned();

            outputs.push(Arc::new(Output {
                name: name.to_string_lossy().into_owned(),
                state,
                modes,
                preferred_mode,
                current_mode,
                current_pos,
                edid ,

            }));
        }

        outputs
    }
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
