fn main() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    // 一般的に `{} `はどんな引数であろうと自動的に置き換えられます。
    // 例えば以下は文字列に変換されます
    println!("{} days", 31);

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string.
    // Without a suffix, 31 becomes an i32. You can change what type 31 is
    // by providing a suffix. The number 31i64 for example has the type i64.
    // サフィックスで型を指定しなければ31はi32として扱われます。
    // サフィックスの指定により、31の型を自由に変換することができます。
    // There are various optional patterns this works with. Positional
    // arguments can be used.
    // 引数の位置から埋め込まれる場所を指定することができます。
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    // 名前での指定も可能です。
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    // `:` のあとにフォーマット型を指定することにより異なるフォーマットも可能です.
    println!("Base 10:               {}", 69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); // 10F2C

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    // 指定した幅の中に、右寄せで文字列を挿入することができます。
    // 以下の例では"     1". というように、５つの半角空白のあとに"1"が入ります.
    println!("{number:>width$}", number = 1, width = 6);

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    // 特定の幅に右詰めすることもできます. このアウトプットは "    1" になります.
    // (4つの空白と"1"で合計幅は5です)
    println!("{number:>5}", number = 1);

    // You can pad numbers with extra zeroes,
    // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number = 1);

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number = 1, width = 5);

    // Rust even checks to make sure the correct number of arguments are used.
    // used.
    // 引数の数が正しいかのチェックも行ってくれます。
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default.

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
                        // Create a structure named `Structure` which contains an `i32`.
                        // `i32`保持する `Structure` という名の構造体を定義します.
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    // このようにカスタム型を用いる場合、少々扱いが複雑になります。
    // 以下は動作しません。
    //println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1", 4 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    let pi: f64 = 3.141592;
    println!("Pi is roughly {:.3}", pi)
}
