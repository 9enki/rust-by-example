struct Empty;
struct Null;

trait DobleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {}

fn main() {}
