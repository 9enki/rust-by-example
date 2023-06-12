struct Texture {
    data: Vec<u8>,
}

struct Entity<'a> {
    texture: &'a Texture,
}

fn main() {
    let texture = Texture { data: vec![0; 100] };

    let entity = Entity { texture: &texture };

    drop(texture);

    println!("{:?}", texture.data);
    println!("{:?}", entity.texture.data);
}
