#[derive(Debug, Clone, Copy)]
struct Interval {
    min: f32,
    max: f32,
}

impl Interval {
    fn new(min: f32, max: f32) -> Self {
        Interval {
            min,
            max,
        }
    }

    fn size(self) -> f32 {
        self.max - self.min
    }   

    fn contains(self, x: f32) -> bool {
        self.min <= x && x <= self.min
    }

    fn surrounds(self, x: f32) -> bool {
        self.min < x &&  x < self.max
    }

    fn clamp(self, x: f32) -> f32 {
        if x < self.min  { return self.min };
        if x > self.max { return self.max} ;
        x
    }
}