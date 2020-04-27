use crate::components::*;
use crate::geometry::*;
use crate::hud::Hud;
use crate::renderer::SceneRenderer;
use shipyard::*;

pub fn init_world(img_area: Area, stage_area: Area, hud: Hud, renderer: SceneRenderer) -> World {
    let world = World::default();

    world.add_unique(ImageArea(img_area));
    world.add_unique(StageArea(stage_area));
    world.add_unique(InstancePositions(Vec::new()));
    world.add_unique(Fps(0));
    world.add_unique(Controller::Waiting);
    world.add_unique(FpsCounter::new());
    world.add_unique(Timestamp(0.0));
    world.add_unique_non_send_sync(renderer);
    world.add_unique_non_send_sync(hud);

    world
        .borrow::<(ViewMut<Position>, ViewMut<Speed>, ViewMut<Gravity>)>()
        .tight_pack();

    world
}
