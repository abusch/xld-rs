use std::fmt;
use std::sync::Arc;

use super::Pos;
use edid::Edid;
use mode::Mode;

#[derive(Debug)]
pub enum OutputState {
    Active, Connected, Disconnected
}

#[derive(Debug)]
pub struct Output {
    pub name: String,
    pub state: OutputState,
    pub modes: Vec<Arc<Mode>>,
    // preferred_mode: Arc<Mode>,
    // optimal_mode: Arc<Mode>,
    // current_pos: Arc<Pos>,
    pub edid: Option<Arc<Edid>>,
    // desired_active: bool,
    // desired_mode: Arc<Mode>,
    // desired_pos: Arc<Pos>,
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let state = match self.state {
            OutputState::Active => "active",
            OutputState::Connected => "connected",
            OutputState::Disconnected => "disconnected",
        };
        let edid_info = self.edid.as_ref().map(|edid| {
            format!(" {}cm/{}cm", edid.max_cm_horiz(), edid.max_cm_vert())
        }).unwrap_or("".to_owned());
        write!(f, "{} {}{}", self.name, state, edid_info)
    }
}
