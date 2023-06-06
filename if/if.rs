fn main() {
    let n = 5;

    if n < 0 {
        println!("negative");
    } else if 0 < n {
        println!("positive");
    } else {
        println!("zero");
    }

    let big_n = if -10 < n && n < 10 {
        println!(", and is a small number, increase ten-fold");
        10 * n
    } else {
        println!(", and is a big number, halve the number");
        n / 2
    };

    println!("{} -> {}", n, big_n);
}
