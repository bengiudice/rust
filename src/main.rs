use std::vec;

static _HI: &str = "HI";
const _YO: &str = "YO";

fn main() {
    
    let a: &'static str = "hi";
    let b = std::string::String::new();
    let c = String::new();
    let d = String::from("");
    let e = "".to_owned();
    let f: String = "".into();
    let g = "".to_string();
    let h = str::to_owned("");
    let i = <&str as Into<String>>::into("");
    let j = Into::<String>::into("");
    let k = ["a", "b", "c"].map(str::to_owned);
    let l = String::to_owned(&"".to_owned());
    

    let o = "";
    let oo: &str = &String::new();
    //let m: &str = &vec![1u8,2,3];

    let p = [""];
    let n: vec::Vec<i32> = vec::Vec::new();
    let nn: &[i32] = &vec![1,2,3];

}
