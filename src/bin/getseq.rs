use rand::seq::SliceRandom;
use structopt::StructOpt;

const CONSONANT: [char; 21] = [
    'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x',
    'y', 'z',
];
const VOWEL: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn word() -> String {
    let mut rng = rand::thread_rng();
    [
        CONSONANT.choose(&mut rng).unwrap().to_owned(),
        VOWEL.choose(&mut rng).unwrap().to_owned(),
        CONSONANT.choose(&mut rng).unwrap().to_owned(),
        VOWEL.choose(&mut rng).unwrap().to_owned(),
        CONSONANT.choose(&mut rng).unwrap().to_owned(),
    ]
    .iter()
    .collect()
}

#[derive(StructOpt)]
struct Options {
    times: Option<u16>,
}

fn main() {
    let options = Options::from_args();
    let times = match options.times {
        Some(t) => t,
        None => 1,
    };
    print!("{}", word());
    if times > 1 {
        for _ in 1..times {
            print!("-{}", word());
        }
    }
    print!("\n");
}
