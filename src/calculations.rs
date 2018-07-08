use monitors::Monitors;
use output::{Output, State};

pub fn order_outputs(outputs: Vec<Output>, order: &[String]) -> Vec<Output> {
    let mut ordered_outputs = outputs.clone();

    // Stack all the prefered, available outputs
    let mut preferred_outputs = Vec::new();
    for name in order {
        for output in &outputs {
            if &output.name == name {
                preferred_outputs.push(output.clone());
            }
        }
    }

    // moved preferred to the front
    for preferred in preferred_outputs {
        let i = ordered_outputs
            .iter()
            .position(|x| x == &preferred)
            .expect("Preferred output not found in the list of available outputs");
        ordered_outputs.remove(i);
        ordered_outputs.push(preferred);
    }

    ordered_outputs
}

pub fn activate_outputs(
    outputs: &mut [Output],
    desired_primary: &str,
    monitors: &Monitors,
) -> Output {
    let mut primary = None;

    for output in outputs.iter_mut() {
        if monitors.should_disable_output(&output.name) {
            continue;
        }
        if output.state != State::Active && output.state != State::Connected {
            continue;
        }

        output.desired_active = true;

        if primary.is_none() || (&output.name == desired_primary) {
            primary = Some(output.clone());
        }
    }

    primary.expect("No active or connected output found!")
}
