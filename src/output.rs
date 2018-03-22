use std::fmt;
use std::sync::Arc;

use pos::Pos;
use edid::Edid;
use mode::Mode;

#[derive(Debug)]
pub enum OutputState {
    Active, Connected, Disconnected
}

impl fmt::Display for OutputState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            OutputState::Active => "active",
            OutputState::Connected => "connected",
            OutputState::Disconnected => "disconnected",
        })
    }
}

#[derive(Debug)]
pub struct Output {
    pub name: String,
    pub state: OutputState,
    pub modes: Vec<Arc<Mode>>,
    pub preferred_mode: Option<Arc<Mode>>,
    pub current_mode: Option<Arc<Mode>>,
    // optimal_mode: Arc<Mode>,
    pub current_pos: Option<Pos>,
    pub edid: Option<Edid>,
    // desired_active: bool,
    // desired_mode: Arc<Mode>,
    // desired_pos: Arc<Pos>,
}
