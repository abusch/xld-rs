use std::sync::Arc;

use super::Pos;
use edid::Edid;
use mode::Mode;

pub enum OutputState {
    Active, Connected, Disconnected
}

pub struct Output {
    name: String,
    state: OutputState,
    modes: Vec<Arc<Mode>>,
    preferred_mode: Arc<Mode>,
    optimal_mode: Arc<Mode>,
    current_pos: Arc<Pos>,
    edid: Arc<Edid>,
    desired_active: bool,
    desired_mode: Arc<Mode>,
    desired_pos: Arc<Pos>,
}

