fn main() {
    // ANCHOR: here
    let mut s = String::from("hola");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{r1}, {r2}");
    // ANCHOR_END: here
}
