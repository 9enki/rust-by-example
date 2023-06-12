fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let my_string = String::from("hello world");

    // first_word contains a reference to the first word of my_string
    let first_word = get_first_word(&my_string);
    println!("first word is: {}", first_word);

    // Error! We can't change my_string because first_word is referencing it
    // my_string.clear();
    println!("the first word is: {}", first_word);
}
