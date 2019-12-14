pub type RenderTimestamp = f64;

#[derive(Copy, Clone, PartialEq, Default, Debug, PartialOrd)]
pub struct RenderTimestampDelta(f64);

impl RenderTimestampDelta {
    pub const fn new(delta: f64) -> Self {
        Self(delta)
    }
}

impl From<RenderTimestampDelta> for f64 {
    fn from(delta: RenderTimestampDelta) -> Self {
        delta.0
    }
}
