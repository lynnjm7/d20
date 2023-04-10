use crate::prng::{ get_time_seed, PRNG };

// See https://en.wikipedia.org/wiki/Xorshift#xorshift*
// NOTE: This implementaion uses magic numbers!! Named constants don't make too 
// much sense in this implementaton because meaninful names can't really be 
// decided for this implementation.

#[derive(Debug)]
pub struct XorshiftStar {
    state: u64
}

impl PRNG for XorshiftStar {
    fn new() -> Self {
        XorshiftStar{
            state: get_time_seed()
        }
    }

    fn next(&mut self) -> u64 {
        self.state ^= self.state >> 12;
        self.state ^= self.state << 25;
        self.state ^= self.state >> 27;

        (self.state as u128 * 0x2545F4914F6CDD1D as u128) as u64
    }
}
