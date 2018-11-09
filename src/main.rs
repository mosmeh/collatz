extern crate clap;
extern crate num_bigint;
extern crate num_traits;

use clap::{App, Arg};
use num_bigint::{BigUint, ToBigUint};
use num_traits::{one, zero};

fn main() {
    let matches = App::new("collatz")
        .arg(
            Arg::with_name("N")
                .help("Initial number")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("print")
                .help("Prints steps")
                .short("p")
                .long("print"),
        )
        .get_matches();
    let print: bool = matches.is_present("print");

    let two = 2u64.to_biguint().unwrap();
    let three = 3u64.to_biguint().unwrap();

    let mut n = BigUint::parse_bytes(matches.value_of("N").unwrap().as_bytes(), 10)
        .expect("Failed to parse number");
    let mut steps = 0;

    loop {
        if print {
            println!("{}", n);
        }

        if n == one() {
            break;
        }
        if &n % &two == zero() {
            n /= &two;
        } else {
            n = &three * n + one::<BigUint>();
        }

        steps += 1;
    }

    println!("Finished with {} steps", steps);
}
