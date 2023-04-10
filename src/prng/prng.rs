use std::time::{ SystemTime, UNIX_EPOCH };

// Get the current time UNIX time in milliseconds to use for the PRNG seed value.
pub fn get_time_seed() -> u64 {
    let start = SystemTime::now();
    start.duration_since(UNIX_EPOCH).expect("Time went backwards!").as_millis() as u64
}

pub trait PRNG {
    fn new() -> Self;
    
    fn next(&mut self) -> u64;

    fn next_min_max(&mut self, min: u64, max: u64) -> u64 {
        min + (self.next() % (max - min))
    }
    
    fn next_max(&mut self, max: u64) -> u64 {
        self.next_min_max(0, max)
    }
}
