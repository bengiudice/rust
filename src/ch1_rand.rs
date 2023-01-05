use std::io::Write;

use rand::Rng;

pub fn ran() {
    println!("Choose an upper limit: ");
    let stdin = &std::io::stdin();
    let max = &mut String::new();
    let max = loop {
        stdin.read_line(max).expect("bad max");
        let max = &**max;
        let max = max.trim().parse::<u64>();
        match max {
            Ok(res) => break res,
            Err(_) => continue,
        };
    };
    let mut remaining = f64::log2(max as f64).ceil() as u64;

    let rand = &mut rand::thread_rng();
    let rand = rand.gen_range(1..=max);

    println!("Guesses remaining: {}", remaining);

    let mut h = max;
    let mut l = 0u64;

    loop {
        print!("Enter a number: ");
        let stdout = &mut std::io::stdout();
        stdout.flush().expect("sheit");

        let guess = &mut String::new();
        stdin.read_line(guess).expect("nogood");
        let guess = &**guess;
        let guess = guess.trim();

        let guess = if guess == "b" {
            Ok((h + l) / 2)
        } else {
            guess.parse::<u64>()
        };

        let guess = match guess {
            Ok(ans) => {
                remaining -= 1;
                ans
            }
            Err(_) => continue,
        };

        let guess = &guess;
        let rand = &rand;
        let ans = guess.cmp(rand);

        let attempt = match ans {
            std::cmp::Ordering::Greater => {
                h = *guess;
                "too big"
            }
            std::cmp::Ordering::Equal => "right!",
            std::cmp::Ordering::Less => {
                l = *guess;
                "too small"
            }
        };

        println!(
            "Your guess of {} was {}. {} remaining.",
            guess, attempt, remaining
        );

        if ans == std::cmp::Ordering::Equal {
            break;
        }

        if remaining == 0 {
            println!("No guesses remaining!");
            return;
        }
    }
}
