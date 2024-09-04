use serde::{Serialize};

#[derive(Serialize)]
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
    println!("{:?}", serde_json::to_string(&my_struct).unwrap());
    // "{\"number\":42,\"is_cool\":true,\"optional\":69}"
    println!("{:?}", serde_json::to_string(&my_struct).unwrap().len());
    // 42
}