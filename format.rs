fn main() {
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    println!("{number:>width$}", number = 1, width = 6);

    println!("{number:>5}", number = 1);

    println!("{number:0<5}", number = 1);

    println!("{number:0>width$}", number = 1, width = 5);

    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)]
    struct Structure(i32);

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    let pi: f64 = 3.141592;
    println!("Pi is roughly {:.3}", pi)
}
