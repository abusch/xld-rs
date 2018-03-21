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
    // edid: Arc<Edid>,
    // desired_active: bool,
    // desired_mode: Arc<Mode>,
    // desired_pos: Arc<Pos>,
}

