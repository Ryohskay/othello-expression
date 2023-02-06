fn main() {
    let four_in_b = 0b100; // always prepend 0b, just as you do for hex 0x
    println!("{}", four_in_b);
    println!("0b100 >> 2 is 0b{:b}", 0b100 >> 2);
    println!("0b10 << 5 is 0b{:b}", 0b10 << 5);
    println!("0b101 << 2 is 0b{:b}", 0b101 << 2);
}
