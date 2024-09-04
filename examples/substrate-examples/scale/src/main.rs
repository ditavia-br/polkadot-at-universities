use parity_scale_codec::{Encode};

#[derive(Encode)]
struct Example {
    number: u8,
    is_cool: bool,
    optional: Option<u32>,
}

fn main() {
    let my_struct = Example {
        number: 42,
        is_cool: true,
        optional: Some(69),
    };
    println!("{:?}", my_struct.encode());
    // [42, 1, 1, 69, 0, 0, 0]
    println!("{:?}", my_struct.encode().len());
    // 7
}