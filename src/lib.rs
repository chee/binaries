use rand::{rngs::StdRng, SeedableRng};
use rand::prelude::SliceRandom;

pub struct Consonant;
pub struct Vowel;
pub struct Digit;
pub struct Letter;

const CONSONANT: [char; 21] = [
    'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x',
    'y', 'z',
];

const VOWEL: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

pub fn word() -> String {
    let mut rng = StdRng::from_entropy();

    [
        *CONSONANT.choose(&mut rng).unwrap(),
        *VOWEL.choose(&mut rng).unwrap(),
        *CONSONANT.choose(&mut rng).unwrap(),
        *VOWEL.choose(&mut rng).unwrap(),
        *CONSONANT.choose(&mut rng).unwrap(),
    ]
    .iter()
    .collect()
}
