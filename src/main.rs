use std::ops::Range;

const DEFAULT_DIVISORS: [(u64, &'static str); 2] = [(3, "fizz"), (5, "buzz")];

fn fizzbuzz_one(arg: u64, divisors: &[(u64, &str)]) -> String {
    let ret = divisors
        .iter()
        .filter_map(|(divisor, word)| match arg % divisor {
            0 => Some(*word),
            _ => None,
        })
        .collect::<String>(); // turbofish operator   ::<>
    match ret.len() {
        0 => arg.to_string(),
        _ => ret,
    }
}

fn fizzbuzz_with_divisors(rng: Range<u64>, divisors: &[(u64, &str)]) -> Vec<String> {
    rng.map(|arg| fizzbuzz_one(arg, divisors)).collect()
}

fn fizzbuzz(rng: Range<u64>) -> Vec<String> {
    fizzbuzz_with_divisors(rng, &DEFAULT_DIVISORS)
}

fn main() {
    let arg = 5;
    println!("{}", fizzbuzz(0..arg).join("\n"));
}

#[cfg(test)]
mod tests {

    #[test]
    fn fizzbuzz_one() {
        assert_eq!(crate::fizzbuzz_one(1, &Vec::new()), "1");
    }

    #[test]
    fn fizzbuzz_no_divisors() {
        assert_eq!(
            crate::fizzbuzz_with_divisors(0..5, &Vec::new()),
            vec!["0", "1", "2", "3", "4"],
        );
    }

    #[test]
    fn fizzbuzz_default_divisors() {
        assert_eq!(
            crate::fizzbuzz(0..5),
            vec!["fizzbuzz", "1", "2", "fizz", "4"],
        );
    }
}
