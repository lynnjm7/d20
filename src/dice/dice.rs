use crate::prng::prng::PRNG;

#[derive(Debug)]
pub struct Dice<Generator: PRNG> {
    sides: u64,
    generator: Generator,
}

impl<Generator: PRNG> Dice<Generator> {
    pub fn d20() -> Dice<Generator> {
        Self::from_sides(20)
    }

    pub fn d12() -> Dice<Generator> {
        Self::from_sides(12)
    }

    pub fn d10() -> Dice<Generator> {
        Self::from_sides(10)
    }

    pub fn d8() -> Dice<Generator> {
        Self::from_sides(8)
    }

    pub fn d6() -> Dice<Generator> {
        Self::from_sides(6)
    }

    pub fn d4() -> Dice<Generator> {
        Self::from_sides(4)
    }

    pub fn d_percent() -> Dice<Generator> {
        Self::from_sides(100)
    }

    pub fn from_sides(sides: u64) -> Dice<Generator> {
        Dice{
            sides: sides,
            generator: Generator::new()
        }
    }

    pub fn roll(&mut self) -> u64 {
        // Draw a random number in the range [1, sides + 1)
        self.generator.next_min_max(1, self.sides + 1)
    }
}
