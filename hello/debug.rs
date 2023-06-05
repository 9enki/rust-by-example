fn main() {
    #[derive(Debug)]
    struct Structure(i32);

    #[derive(Debug)]
    struct Deep(Structure);

    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    println!("Now {:?} will print!", Structure(3));
    println!("Now {:#?} will print!", Structure(3));

    println!("Now {:?} will print!", Deep(Structure(7)));
    println!("Now {:#?} will print!", Deep(Structure(7)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let peter = Person {
        name: "Peter",
        age: 27,
    };

    println!("{:#?}", peter);
    println!("{:#?}", peter.name);
    println!("{:#?}", peter.age);
}
