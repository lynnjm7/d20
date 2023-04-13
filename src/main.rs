mod dice;
mod prng;

use dice::Dice;
use crate::prng::{ PRNG, MersenneTwister, XorshiftStar };

use clap::{ Parser, builder::PossibleValue, ValueEnum };

/// DiceTypes is the permissible type options that can be provided to the CLI
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum DiceTypes {
    /// A 20-sided die. This is the default selection.
    D20,

    /// A 12-sided die.
    D12,
    
    /// A 10-sided die.
    D10,

    /// An 8-sided die.
    D8,

    /// A 6-sided die.
    D6,

    /// A 4-sided die.
    D4,

    /// A percentage die.
    DPercent,
}

/// Implementation of ValueEnum to be able to parse DiceTypes provided to the CLI
/// We need this to use an enum type with clap for the cli args.
impl ValueEnum for DiceTypes {
    fn value_variants<'a>() -> &'a [Self] {
        &[DiceTypes::D20, DiceTypes::D12, DiceTypes::D10, DiceTypes::D8, DiceTypes::D6, DiceTypes::D4, DiceTypes::DPercent]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            DiceTypes::D20 => PossibleValue::new("d20").help("Use a d20 die."),
            DiceTypes::D12 => PossibleValue::new("d12").help("Use a d12 die."),
            DiceTypes::D10 => PossibleValue::new("d10").help("Use a d10 die."),
            DiceTypes::D8 => PossibleValue::new("d8").help("Use a d8 die."),
            DiceTypes::D6 => PossibleValue::new("d6").help("Use a d6 die."),
            DiceTypes::D4 => PossibleValue::new("d4").help("Use a d4 die."),
            DiceTypes::DPercent => PossibleValue::new("d_percent").help("Use a percentage die.")
        })
    }
}

/// The Display trait on DiceTypes is needed to be able to process args for the CLI
/// We need this to use an enum type with clap for the cli args.
impl std::fmt::Display for DiceTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value().expect("no values are skipped").get_name().fmt(f)
    }
}

/// PrngOptions are the available options that can be selected from the CLI
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum PrngOptions {
    /// Use the Mersenne Twister prng backend. This is the default selection.
    MersenneTwister,
    /// Use the Xorshift* prng backend.
    XorshiftStar,
}

/// ValueEnum is implemented for PrngOPtions to process args for the CLI
/// We need this to use an enum type with clap for the cli args.
impl ValueEnum for PrngOptions {
    fn value_variants<'a>() -> &'a [Self] {
        &[PrngOptions::MersenneTwister, PrngOptions::XorshiftStar]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            PrngOptions::MersenneTwister => PossibleValue::new("mersenne_twister").help("Use a Mersenne Twister PRNG"),
            PrngOptions::XorshiftStar => PossibleValue::new("xorshift_star").help("Use an Xorshift* PRNG")
        })
    }
}

/// Display is implemented for PrngOptions to process args for the CLI.
/// We need this to use an enum type with clap for the cli args.
impl std::fmt::Display for PrngOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value().expect("no values are skipped").get_name().fmt(f)
    }
}

/// Args holds the command line arguments that are parsed by clap.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    rolls: u64,

    #[arg(short, long, default_value_t = PrngOptions::MersenneTwister)]
    prng: PrngOptions,

    #[arg(short, long, default_value_t = DiceTypes::D20)]
    dice: DiceTypes
}

/// A helper method that allows for different Generator implementations to be
/// used and perform the dice roll operation.
fn dice_roller<Generator: PRNG>(dice: DiceTypes, rolls: u64) {
    let mut die = match dice {
        DiceTypes::D20 => Dice::<Generator>::d20(),
        DiceTypes::D12 => Dice::<Generator>::d12(),
        DiceTypes::D10 => Dice::<Generator>::d10(),
        DiceTypes::D8 => Dice::<Generator>::d8(),
        DiceTypes::D6 => Dice::<Generator>::d6(),
        DiceTypes::D4 => Dice::<Generator>::d4(),
        DiceTypes::DPercent => Dice::<Generator>::d_percent(),
    };

    for _ in 0..rolls {
        println!("{}", die.roll());
    }
}

fn main() {
    let args = Args::parse();
    match args.prng {
        PrngOptions::MersenneTwister => { dice_roller::<MersenneTwister>(args.dice, args.rolls); }
        PrngOptions::XorshiftStar => { dice_roller::<XorshiftStar>(args.dice, args.rolls); }
    };
}
