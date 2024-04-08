fn main() {
    println!("result: {}", interproduct(120, 100, 248));
}

// Example how to deal with overflow.
// fn interproduct(a: i16, b: i16, c: i16) -> i16 {
//     return (a * b).saturating_add(b * c).saturating_add(c * a);
// }

fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b * c + c * a;
}
