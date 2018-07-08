use failure::Error;

use calculations::{activate_outputs, order_outputs};
use monitors::Monitors;
use settings::Settings;
use xrandrutils::{discover_outputs, render_xrandr_command};

pub fn layout(settings: &Settings) -> Result<(), Error> {
    // discover monitors
    let monitors = Monitors::new();

    // discover outputs
    let current_outputs = discover_outputs()?;
    if current_outputs.is_empty() {
        bail!("no outputs found");
    }

    // output verbose information
    if !settings.quiet || settings.info {
        for output in &current_outputs {
            print!("{} {}", output.name, output.state);
            if let Some(ref edid_info) = output.edid {
                print!(
                    " {}cm/{}cm",
                    edid_info.max_cm_horiz(),
                    edid_info.max_cm_vert()
                )
            }
            if let (&Some(ref current_mode), &Some(ref current_pos)) =
                (&output.current_mode, &output.current_pos)
            {
                print!(" {}x{}", current_mode.width, current_mode.height);
                print!("+{}+{}", current_pos.x, current_pos.y);
                print!(" {}Hz", current_mode.refresh);
            }
            println!();
            for mode in &output.modes {
                if Some(mode) == output.current_mode.as_ref() {
                    print!("*");
                } else {
                    print!(" ");
                }
                if Some(mode) == output.preferred_mode.as_ref() {
                    print!("+");
                } else {
                    print!(" ");
                }
                // if Some(mode) == output.optimal_mode.as_ref() {
                //     print!("*");
                // } else {
                //     print!(" ");
                // }
                println!(" {}x{} {}Hz", mode.width, mode.height, mode.refresh);
            }
        }
        println!();

        println!(
            "laptop lid {}",
            if monitors.laptop_lid_closed {
                "closed"
            } else {
                "open or not present"
            }
        );
    }

    // current info is all output, we're done
    if settings.info {
        return Ok(());
    }
    // Order the outputs if the user wishes
    let mut outputs = order_outputs(current_outputs, &settings.order);
    // activate outputs and determine primary
    let primary = activate_outputs(&mut outputs, &settings.primary, &monitors);
    // arrange mirrored or left to right
    // determine DPI from the primary
    // render desired command
    let xrandr_cmd = render_xrandr_command(&outputs, &primary, 120);
    println!("xrandr command: {:?}", xrandr_cmd);
    // execute

    Ok(())
}
