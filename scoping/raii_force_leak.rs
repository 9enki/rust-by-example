fn create_box() {
    // Allocate an integer on the heap
    let _box1 = Box::new(3i32);

    // Leak the box by "forgetting" it
    // ボックスをリークさせる（忘れる）
    std::mem::forget(_box1);
}

fn main() {
    // Allocate an integer on the heap
    let _box2 = Box::new(5i32);

    // Leak the box by "forgetting" it
    // ボックスをリークさせる（忘れる）
    std::mem::forget(_box2);

    // Creating lots of boxes and intentionally leaking them
    // 大量のボックスを作成し、意図的にリークさせる
    for _ in 0u32..1_000 {
        create_box();
    }
}
