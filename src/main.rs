fn main() {
    let x = [(1,2u32),(3,4),(5,6),(7isize,8)];
    println!("-> {:?}", &x[2..]);
    let mut out = String::new();
    let _f = std::fmt::write(&mut out, format_args!("heyy"));
    println!("{out:?}");
}



