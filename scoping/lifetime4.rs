fn main() {
    let inner = &"Unko";
    let value = {
        let hoge = Hoge { value: inner };
        hoge.value()
    };

    println!("hoge: {}", value)
}

struct Hoge {
    value: &str,
}

impl Hoge {
    fn value(&self) -> &str {
        self.value
    }
}
