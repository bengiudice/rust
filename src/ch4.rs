pub fn main () {
    let x = 4;
    let y = x;

    println!("{} {}", x, y);

    let a = Box::new(1);
    let _b = a;

    //println!("{} {}", a, b);

    let s1 = String::from("");

    let s2 = &String::from("");

    let s = &s1[..];

    let s = "";

    let s = &""[..];
}