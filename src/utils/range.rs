pub struct Range {
    pub start: f32,
    pub end: f32,
}

impl Range {
    pub fn mix(&self, ratio: f32) -> f32 {
        self.start * (1.0 - ratio) + self.end * ratio
    }

    pub fn _ratio(&self, mix: f32) -> f32 {
        (mix - self.start) / (self.end - self.start)
    }

    pub fn step(&self, amount: f32) -> f32 {
        let d = self.end - self.start;
        if d.abs() < amount {
            self.end
        } else {
            self.start + d.signum() * amount
        }
    }
}