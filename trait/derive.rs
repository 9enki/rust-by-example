#[derive(Debug, PartialEq, PartialOrd)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        Centimeters(self.0 as f64 * 2.54)
    }
}

struct Seconds(i32);

fn main() {
    let _one_second = Seconds(1);

    // _one_secondはDebug deriveがないから:?でデバッグプリントできない
    // println!("{:?}", _one_second);

    // _one_secondはPartialEq deriveがないから比較できない
    // let _this_is_true = (_one_second == _one_second);

    let a = Inches(32);

    println!("{:?}", a.to_centimeters().0);
    println!("{:?}", a.0);

    let meter = Centimeters(100.0);

    let cmp = if a.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };

    println!("{:?}", cmp);
}
