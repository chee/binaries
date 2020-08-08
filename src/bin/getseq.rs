use binaries::word;
use structopt::StructOpt;

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
