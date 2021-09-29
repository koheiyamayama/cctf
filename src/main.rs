extern crate getopts;
use getopts::Options;
use std::env;

// -c N
// convert n from fahrenheit to celsius,
// - f N
// convert n from celsius to fahrenheit,
// without options
// default -c n

// e.g.
// cctf -f 2
// -> convert fahrenheit to celsius.
// cctf -c 10
// -> convert celsius to fahrenheit.
// cctf 2
// -> convert celsius to fahrenheit.
// -> 未実装
// cctf
// panic!!
// -> 未実装

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt(
        "f",
        "fahrenheit",
        "convert n from fahrenheit to celsius.",
        "",
    );
    opts.optopt("c", "celsius", "convert n from celsius to fahrenheit.", "");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    if matches.opt_present("f") {
        let c = match matches.opt_str("f") {
            Some(n) => n.trim().parse::<f64>().expect("not number. specify number"),
            None => panic!("not specified n."),
        };

        let f = (c * 1.8) + 32.0;
        println!("{}", f);
    };

    if matches.opt_present("c") {
        let f = match matches.opt_str("c") {
            Some(n) => n.trim().parse::<f64>().expect("not number. specify number"),
            None => panic!("not specified n."),
        };

        let c = (f - 32.0) / 1.8;
        println!("{}", c);
    }
}
