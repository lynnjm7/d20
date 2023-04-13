use std::time::{ SystemTime, UNIX_EPOCH };

/// Get the current time UNIX time in milliseconds to use for the seeding a 
/// pseudo-random number generator.
pub fn get_time_seed() -> u64 {
    let start = SystemTime::now();
    start.duration_since(UNIX_EPOCH).expect("Time went backwards!").as_millis() as u64
}

/// A standard interface that is required for a pseudo-random number generator.
pub trait PRNG {
    /// Require that a pseudo-random number generator is able to instantiate itself
    fn new() -> Self;
    
    /// Require that a pseudo-random number generator is able to calculate the next 
    /// number in the sequence.
    fn next(&mut self) -> u64;

    /// A helper method that will use the pseudo-random number generator's `next` 
    /// implementation to get a random number in the range `[min, max)`
    fn next_min_max(&mut self, min: u64, max: u64) -> u64 {
        min + (self.next() % (max - min))
    }
    
    /// A helper method that will use the pseudo-random number generator's `next` 
    /// implementation to get a random number in the range `[0, max)`
    fn next_max(&mut self, max: u64) -> u64 {
        self.next_min_max(0, max)
    }
}
