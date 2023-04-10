D20
===
An overcomplicated dice roll program.

This program will roll a set of D&D die a given number of times. 
For usage details use the `-h` option in the utility.

# Random Number Generators
This program implements 2 backend pseudo-random number generators:
  1. Mersenne Twister
  2. Xorshift* 

The Mersenne Twister is the default PRNG that we use when rolling a die. You 
can optionally specificy which PRNG to use when invoking the program on the
command line.

Both RNGs use a custom implementation (not a 3rd party library).

# Purpose of the Project
This is not a technically intersting project. It doesn't have very much depth 
to it. The main goal for this project was to play with Rust a little more.

# License
This is released under an MIT license. See the `LICENSE.txt` file.
