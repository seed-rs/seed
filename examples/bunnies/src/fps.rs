pub struct FpsCounter {
    begin_time: f64,
    prev_time: f64,
    frames: usize,
    pub current: f64,
}

impl FpsCounter {
    pub fn new() -> Self {
        let begin_time = Self::now();
        Self {
            begin_time,
            prev_time: begin_time,
            frames: 0,
            current: 0.0,
        }
    }

    pub fn now() -> f64 {
        web_sys::window().unwrap().performance().unwrap().now()
    }

    pub fn begin(&mut self) {
        self.begin_time = Self::now();
    }

    pub fn end(&mut self) {
        self.frames += 1;
        let time = Self::now();

        if time >= (self.prev_time + 1000.0) {
            self.current = ((self.frames * 1000) as f64) / (time - self.prev_time);
            self.prev_time = time;
            self.frames = 0;
        }
    }
}
