#[derive(Debug, Default)]
pub struct Settings {
    pub info: bool,
    pub noop: bool,
    pub mirror: bool,
    pub order: Vec<String>,
    pub primary: String,
    pub quiet: bool,
}
