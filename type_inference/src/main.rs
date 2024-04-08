fn takes_u32(x: u32) {
    println!("x: {x}");
}
fn takes_i8(y: i8) {
    println!("y: {y}");
}
fn main() {
    let x = 10;
    let y = 20;

    takes_u32(x);
    takes_i8(y);
    // Undefined type will assume a data type by first use. Afterwards the data type cannot be simply changed. The rule for convertion need to be applied.
    //takes_u32(y);
}
