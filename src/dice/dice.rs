use crate::prng::prng::PRNG;

#[derive(Debug)]
pub struct Dice<Generator: PRNG> {
    /// The number of sides of the die
    sides: u64,

    /// The pseudo-random number generator used to perform the die rolls
    generator: Generator,
}

/// Implement the methods for Dice with a parameterized Generator PRNG type.
impl<Generator: PRNG> Dice<Generator> {
    /// A standard 20-sided die
    pub fn d20() -> Dice<Generator> {
        Self::from_sides(20)
    }

    /// A standard 12-sided die
    pub fn d12() -> Dice<Generator> {
        Self::from_sides(12)
    }

    /// A standard 10-sided die
    pub fn d10() -> Dice<Generator> {
        Self::from_sides(10)
    }

    /// A standard 8-sided die
    pub fn d8() -> Dice<Generator> {
        Self::from_sides(8)
    }

    /// A standard 6-sided die
    pub fn d6() -> Dice<Generator> {
        Self::from_sides(6)
    }

    /// A standard 4-sided die
    pub fn d4() -> Dice<Generator> {
        Self::from_sides(4)
    }

    /// A percentage die.
    pub fn d_percent() -> Dice<Generator> {
        Self::from_sides(100)
    }

    /// A helper constructor to produce a new die with the given sides and an
    /// instance of our Generator parameter.
    pub fn from_sides(sides: u64) -> Dice<Generator> {
        Dice{
            sides: sides,
            generator: Generator::new()
        }
    }

    /// Roll the dice. This will return a random number in the range `[1, sides]`
    pub fn roll(&mut self) -> u64 {
        self.generator.next_min_max(1, self.sides + 1)
    }
}
