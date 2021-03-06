use std::fmt;

use failure::Error;

use edid::Edid;
use mode::Mode;
use pos::Pos;

#[derive(Debug, PartialEq, Clone)]
pub enum State {
    Active,
    Connected,
    Disconnected,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                State::Active => "active",
                State::Connected => "connected",
                State::Disconnected => "disconnected",
            }
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Output {
    pub name: String,
    pub state: State,
    pub modes: Vec<Mode>,
    pub preferred_mode: Option<Mode>,
    pub current_mode: Option<Mode>,
    // optimal_mode: Arc<Mode>,
    pub current_pos: Option<Pos>,
    pub edid: Option<Edid>,
    pub desired_active: bool,
    // desired_mode: Arc<Mode>,
    // desired_pos: Arc<Pos>,
}

impl Output {
    pub fn new(
        name: String,
        state: State,
        modes: Vec<Mode>,
        current_mode: Option<Mode>,
        preferred_mode: Option<Mode>,
        current_pos: Option<Pos>,
        edid: Option<Edid>,
    ) -> Result<Output, Error> {
        match state {
            State::Active => {
                if current_mode.is_none() {
                    bail!("active output {} has no current_mode", name);
                }
                if current_pos.is_none() {
                    bail!("active output {} has no current_pos", name);
                }
                if modes.is_empty() {
                    bail!("active output {} has no modes", name);
                }
            }
            State::Connected => {
                if modes.is_empty() {
                    bail!("connected output {} has no modes", name);
                }
            }
            State::Disconnected => {}
        }

        // active / connected must have NULL or valid preferred mode
        if state == State::Active || state == State::Connected {
            if let Some(ref preferred_mode) = preferred_mode {
                if !modes.contains(preferred_mode) {
                    bail!("Output {} has preferred_mode not present in modes", name);
                }
            }
        }

        Ok(Output {
            name,
            state,
            modes,
            current_mode,
            preferred_mode,
            current_pos,
            edid,
            desired_active: false,
        })
    }
}
