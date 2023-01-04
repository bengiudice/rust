static _HI: &str = "HI";
const _YO: &str = "YO";

pub mod heyu {
    pub fn look_strings() {
        let _f: String = "".into();
        let _b = std::string::String::new();
        let _c = String::new();
        let _d = String::from("");
        let _e = "".to_owned();
        let _g = "".to_string();
        let _h = str::to_owned("");
        let _i = <&str as Into<String>>::into("");
        let _j = Into::<String>::into("");
        let _l = String::to_owned(&"".to_owned());
    }
    fn _look_str() {
        let _a: &'static str = "hi";
        let _o = "";
        let _oo: &str = &String::new();
        //let m: &str = &vec![1u8,2,3];    
    }
    fn _look_vec() {
        let _k = ["a", "b", "c"].map(str::to_owned);
        let _p = [""];
        let _n: std::vec::Vec<i32> = std::vec::Vec::new();
        let _nn: &[i32] = &vec![1,2,3];    
    }
}