fn main() {
    let mut x: i32 = 6;
    print!("{x}");
    while x != 1 {
        x -= 1;
        print!(" -> {x}");
    }
    println!();
}
