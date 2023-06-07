fn main() {
    use std::mem;

    let color = String::from("green");

    // `color`をプリントするためのクロージャ。
    // これは`color`を借用(`&`)し、その借用とクロージャを`print`
    // という名の変数に保持する。
    // 借用は`print`がスコープから出るまで続く。
    // `println!`は参照を与えれば機能するので、これ以上なにかする必要はない。
    let print = || println!("`color`: {}", color);

    // 借用を行ったクロージャをコールする。
    print();

    // `color`を再びイミュータブルで借用することができる。
    // これはクロージャが`color`に対するイミュータブルな参照しか保持しないからである。
    let _reborrow = &color;
    print();

    // 最後に`print`を使用した後は移動や再借用が許可される。
    let _color_moved = color;

    let mut count = 0;
    // `count`をインクリメントするためのクロージャ。`count`と`&mut count`
    // の両方を取ることができるが、後者のほうが制限が少ないため、
    // （訳注: `count`だと`&mut count`と違い、一度しか呼ぶことができない。）
    // そちらを取る。直後に`count`を借用する。
    //
    // `inc`には`mut`をつける必要がある。なぜならミュータブルな型が
    // 中で使用されているからである。ミュータブルなクロージャは呼ぶたびに
    // 内部変数を変更する。
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // クロージャを実行
    inc();

    // クロージャはまだ `count` をミュータブルで借用している。
    // なぜなら後で呼ばれるからである。
    // 再借用しようとするとエラーになる。
    // ^ TODO: この行のコメントアウトを解除しましょう。
    inc();

    // クロージャはもう`&mut count`を借用する必要がない。
    // なので、エラーを起こさず再借用することができる。
    let _count_reborrowed = &mut count;

    // コピー不可能な型
    let movable = Box::new(3);
    println!("{:?}", movable);

    // `mem::drop`は`T`（ジェネリック型）を取る必要があるため、このクロージャは
    // 参照ではなく値を取る。その場合、もしもコピー可能な値ならば、
    // 元の値はそのままでコピーのみを取る。不可能ならば値そのものを移動させる。
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume`は変数を消費（開放）するため、一度しか呼び出すことができない。
    consume();
    // consume();
    // println!("{:?}", movable);
    // ^ TODO: Try uncommenting this line.
    // ^ TODO: この行のコメントアウトを解除しましょう。
}
