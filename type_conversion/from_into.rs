use std::convert::From;

#[derive(Debug)]
struct Test {
    x: i32,
    y: i32,
}

impl From<i32> for Test {
    fn from(item: i32) -> Self {
        Test {
            x: item,
            y: item + 1,
        }
    }
}

fn main() {
    let num = Test::from(5);
    println!("{:?}", num);

    let num: Test = 5i32.into();
    println!("{:?}", num);
}
