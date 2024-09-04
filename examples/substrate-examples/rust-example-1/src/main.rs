fn main() {
    let x = Box::new(10);
    let y = x;
    println!("{}", *y); // ❌
}