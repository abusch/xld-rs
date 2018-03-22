use failure::Error;

use monitors::Monitors;
use settings::Settings;
use xrandrutils::discover_outputs;

pub fn layout(settings: Settings) -> Result<(), Error> {
    // discover monitors
    let monitors = Monitors::new();

    // discover outputs
    let current_outputs = discover_outputs();
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
            match (&output.current_mode, &output.current_pos) {
                (&Some(ref current_mode), &Some(ref current_pos)) => {
                    print!(" {}x{}", current_mode.width, current_mode.height);
                    print!("+{}+{}", current_pos.x, current_pos.y);
                    print!(" {}Hz", current_mode.refresh);
                }
                _ => {}
            }
            println!();
            for mode in &output.modes {
                println!("   {}x{} {}Hz", mode.width, mode.height, mode.refresh);
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
    // activate outputs and determine primary
    // arrange mirrored or left to right
    // determine DPI from the primary
    // render desired command
    // execute

    Ok(())
}
