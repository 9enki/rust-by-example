trait AAA {
    fn a(&self) -> &str;
    fn b(&self) -> &str;
    fn c(&self) -> &str;
    fn d(&self) -> &str;
    fn e(&self) -> &str;
}

fn hoge<T: AAA>(t: T) {
    println!("{}", t.c());
}

struct B {
    flag: bool,
}

impl AAA for B {
    fn c(&self) -> &str {
        "test"
    }
}

fn main() {
    let b: B = B { flag: false };
    hoge(b);
}
