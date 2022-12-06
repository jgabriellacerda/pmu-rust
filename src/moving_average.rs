#[derive(Debug)]
pub struct MovingAverage<const N: usize> {
    current_idx: usize,
    next_idx: usize,
    size: usize,
    buffer: [f64; N],
    out: f64,
}

impl<const N: usize> MovingAverage<N> {
    pub fn new() -> MovingAverage<N> {
        MovingAverage {
            current_idx: N,
            next_idx: 0,
            size: N,
            buffer: [0.0; N],
            out: 0.0,
        }
    }
    pub fn filter(&mut self, sample: f64) -> f64 {
        self.out = self.out + (sample - self.buffer[self.next_idx]) / self.size as f64;
        self.buffer[self.next_idx] = sample;
        self.current_idx = self.next_idx;
        self.next_idx = (self.current_idx + 1) % self.size;

        return self.out;
    }
}
