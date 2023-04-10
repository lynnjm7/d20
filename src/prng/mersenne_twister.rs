use crate::prng::{ get_time_seed, PRNG };

// See https://en.wikipedia.org/wiki/Mersenne_Twister
// This is a 64-bit implementation and uses the 64-bit MT parameters.
// NOTE: This implementation does use magic numbers!! While some of these
// could potentially be constants, there are not meaningful names that can
// necessarily be applied to every constant value in the implementation.

const STATE_VECTOR_LENGTH: usize = 312;
const LOWER_MASK: u64 = (1 << 31) - 1; // When applied will give the lower bits in a 64-bit uint
const UPPER_MASK: u64 = !(LOWER_MASK) - 1; // When applied will give the upper bits in a 64-bit uint

#[derive(Debug)]
pub struct MersenneTwister {
    mt_table: [u64; STATE_VECTOR_LENGTH],
    index: usize
}

impl MersenneTwister {
    fn twist(&mut self) {
        for i in 0..STATE_VECTOR_LENGTH {
            let x = (self.mt_table[i] & UPPER_MASK) | (self.mt_table[(i + 1) % STATE_VECTOR_LENGTH] & LOWER_MASK);
            let mut xshft = x >> 1;
            if xshft % 2 != 0 {
                xshft ^= 0xB5026F5AA96619E9;
            }
            self.mt_table[i] = self.mt_table[(i + 156) % STATE_VECTOR_LENGTH] ^ xshft;
        }
        self.index = 0;
    }
}

impl PRNG for MersenneTwister {
    fn new() -> Self {
        let mut table: [u64; STATE_VECTOR_LENGTH] = [0; STATE_VECTOR_LENGTH];
        table[0] = get_time_seed(); // Set the first element of our table to use current time as seed
        for i in 1..STATE_VECTOR_LENGTH {
            let prev = table[i-1];
            let factor = (6364136223846793005 * ((prev ^ (prev >> 29)) + (i as u64)) as u128) as u64;
            table[i] = (factor & LOWER_MASK) as u64;
        }

        MersenneTwister{
            mt_table: table,
            index: STATE_VECTOR_LENGTH
        }
    }

    fn next(&mut self) -> u64 {
        if self.index >= STATE_VECTOR_LENGTH {
            self.twist();
        }

        // These constants come from the Wikipedia link at the top of the page.
        let mut y = self.mt_table[self.index];
        y ^= (y >> 29) & 0x5555555555555555;
        y ^= (y << 17) & 0x71D67FFFEDA60000;
        y ^= (y << 37) & 0xFFF7EEE000000000;
        y ^= y >> 43;

        self.index += 1;
        y & LOWER_MASK
    }
}
