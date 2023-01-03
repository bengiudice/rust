use std::vec;
use std::string;

static HI: &str = "HI";
const YO: &str = "YO";

fn main() {
    
    let q: &'static str = "hi";
    let d = string::String::new();
    let s = String::new();
    let y = String::from("");
    let x = "".to_owned();
    let z: String = "".into();
    let f = str::to_owned("");
    let r = <&str as Into<String>>::into("");
    let b = Into::<String>::into("");
    let u = ["a", "b", "c"].map(str::to_owned);
    let o = String::to_owned(&"".to_owned());
    
    let l = vec!['a','b','c'];
    let ll: vec::Vec<i32> = vec::Vec::new();
    let c = "";
    let i = [""];


    println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", s, y, z, x, b, f, c, r, u, o, i);
}

