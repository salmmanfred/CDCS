pub struct Tween {
    duration: f32,
    start: f32,
    change: f32,
    time: f32,
}
impl Tween {
    pub fn new(duration: f32) -> Tween {
        Tween {
            duration: duration,
            start: 0.0,
            change: 0.0,
            time: 0.0,
        }
    }
    pub fn start(&mut self, start: f32, target: f32) {
        self.start = start;
        self.change = target - start;
        self.time = 0.;
    }
    pub fn get(&mut self, time_delta: f32) -> f32 {
        self.time += time_delta;
        if self.time < self.duration {
            // Ease in
            return -self.change * (self.time / self.duration) * (self.time / self.duration - 2.0)
                + self.start;

        // Ease out & in //time /= duration / 2;
        //if (time < 1)
        //	return change / 2 * time * time + start;
        //return -change / 2 * ((time - 1) * (time - 3) - 1) + start;
        } else {
            return self.start + self.change;
        }
    }
}
