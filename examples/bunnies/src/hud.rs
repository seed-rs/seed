#[derive(Default, Debug)]
pub struct Hud {
    num_bunnies: usize,
    fps: u32,
}

impl Hud {
    pub fn update(&mut self, len: usize, fps: u32) {
        self.num_bunnies = len;
        self.fps = fps;
    }

    pub fn num_bunnies(&self) -> usize {
        self.num_bunnies
    }

    pub fn fps(&self) -> u32 {
        self.fps
    }
}
