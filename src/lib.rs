use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use std::iter;

pub struct Consonant;
pub struct Vowel;
pub struct Digit;
pub struct Letter;

const CONSONANT: [char; 21] = [
    'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x',
    'y', 'z',
];

const VOWEL: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

impl Distribution<char> for Consonant {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
        let range = Uniform::new(0, 21);
        CONSONANT[range.sample(rng)]
    }
}

impl Distribution<char> for Vowel {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
        let range = Uniform::new(0, 5);
        VOWEL[range.sample(rng)]
    }
}

impl Distribution<char> for Letter {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> char {
        Uniform::new(b'A', b'Z').sample(rng).into()
    }
}

impl Distribution<usize> for Digit {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> usize {
        Uniform::new(0, 9).sample(rng)
    }
}

pub fn word() -> String {
    let mut crng = rand::thread_rng();
    let mut vrng = rand::thread_rng();
    let mut cons = iter::repeat(()).map(|()| crng.sample(Consonant));
    let mut vows = iter::repeat(()).map(|()| vrng.sample(Vowel));
    [
        cons.next().unwrap(),
        vows.next().unwrap(),
        cons.next().unwrap(),
        vows.next().unwrap(),
        cons.next().unwrap(),
    ]
    .iter()
    .collect()
}
