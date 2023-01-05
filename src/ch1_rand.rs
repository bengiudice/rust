use std::io::{Write, Stdout, Stdin};
use rand::Rng;

pub fn ran() {
    

    print!("Enter a number: ");
    let ref mut stdout: Stdout = std::io::stdout();
    stdout.flush().expect("sheit");

    let ref mut guess: String = String::new();
    let stdin: &Stdin = &*&mut std::io::stdin();
    stdin.read_line(guess).expect("nogood");
    
    let parsed: u32 = guess.trim().parse().expect("parse failed");
    println!("You entered: {}", parsed);

    let ref mut rand_gen = rand::thread_rng();
    let ans = rand_gen.gen_range(1..=5);
    println!("And here's a random number: {}", ans);



}
