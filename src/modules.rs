use std::io::Write;

use rand::Rng;

pub fn ran() {
    let mut r = rand::thread_rng();
    let rr = r.gen_range(1..=5);
    let mut guess = String::new();
    print!("Enter a number: ");
    let mut out = std::io::stdout();
    out.flush().expect("sheit");
    let sin = std::io::stdin();
    sin.read_line(&mut guess).expect("nogood");
    println!("You entered: {}", guess);
    println!("And here's a random number: {}", rr);



}