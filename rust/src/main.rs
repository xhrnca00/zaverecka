fn main() {
    println!("Hello World!");
    if 2 <= 5 {
        let v = Vec::new();
        v.push_back(5u64);
    } else {
        unreachable!("2 is lower than 5");
    }
}
// honestly -> quite -> incredible

