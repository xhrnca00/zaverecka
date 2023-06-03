fn main() {
    // confirming non-null string termination
    let text = "Oh hello there";
    println!("{text}");
    for c in text.bytes() {
        print!("{} ", c as u16);
    }
    println!();
}
