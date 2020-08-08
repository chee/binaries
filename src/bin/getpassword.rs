use binaries::Digit;
use binaries::Letter;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let position = rng.gen_range(0, 4);
    for step in 0..4 {
        if step > 0 {
            print!("-");
        }
        let word = if step == position {
            format!("{}{}", rng.sample(Digit), rng.sample(Letter))
        } else {
            binaries::word()
        };
        print!("{}", word);
    }
    print!("\n");
}
