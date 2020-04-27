use crate::geometry::*;
//re-exported so its easier to just use components::*
pub use crate::fps::FpsCounter;
pub use crate::hud::Hud;
pub use crate::renderer::SceneRenderer;

pub struct ImageArea(pub Area);
pub struct StageArea(pub Area);
pub struct InstancePositions(pub Vec<f32>);
pub struct Fps(pub u32);
pub struct Timestamp(pub f64);
#[derive(PartialEq)]
pub enum Controller {
    Adding,
    Waiting,
}

//the bunnies
pub struct Position(pub Point);
pub struct Speed(pub Point);
pub struct Gravity(pub f64);
