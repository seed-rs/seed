impl Default for Theme {
    fn default() -> Self {
        Self::Light
    }
}
/// Theme for the App.
#[derive(Clone)]
pub enum Theme {
    Light,
    Dark,
}
