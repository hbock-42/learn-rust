// Primitive str is immutable fixed-length
// String is growable;

pub fn run() {
    // immutable
    let name_immutable = "Hugo";
    println!("the string \"{}\" is {} char long", name_immutable, name_immutable.len());

    // mutable
    let mut name_mutable = String::from("Bock");
    let nameqwd_mutable = String::from("qwdqqwdqdwdqqwdqqwdqdwdqqwdqqwdqdwdqqwdqqwdqdw");
    println!("the string \"{}\" is {} char long", name_mutable, name_mutable.len());
    name_mutable.push(' ');
    name_mutable.push_str(name_immutable);
    println!("\"{}\" is {} char long", name_mutable, name_mutable.len());
    println!("capacity = {}", name_mutable.capacity());
    name_mutable.push_str(&nameqwd_mutable);
    println!("capacity = {}", name_mutable.capacity());
}