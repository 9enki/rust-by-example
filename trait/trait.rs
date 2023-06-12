struct Sheep {
    naked: bool,
    name: &'static str,
}

trait Animal {
    // 関連関数のシグネチャ。
    // `Self` はこのトレイトを実装している型になる。
    fn new(name: &'static str) -> Self;

    // メソッドのシグネチャ。
    // これらの関数は文字列を返す。
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // メソッドのデフォルトの挙動を定義することもできる。
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // Implementor methods can use the implementor's trait methods.
            // メソッドをある型に実装する際に、その型のトレイトメソッドを
            // 使用することができる。
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep {
            naked: false,
            name: name,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }
}

fn main() {
    let mut dolly: Sheep = Animal::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();
}
