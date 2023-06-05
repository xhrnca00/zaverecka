fn main() {
    // confirming non-null string termination
    let text = "Oh hello there";
    println!("{text}");
    for c in text.bytes() {
        print!("{} ", c as u16);
    }
    println!();
    // byte char
    unsafe {
        let s = String::from("Hello");
        let mut s = s.into_bytes();
        s[0] = b'h';
        let s = String::from_utf8_unchecked(s);
        println!("{s}");
    }
}
