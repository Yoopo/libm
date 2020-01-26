// Step 1: write a program to convert the float into its integer representation
#[macro_use]
extern crate hexf;

fn main() {
    let f = hexf64!("-0x1.af74bfa0f1304p-56");
    println!("{:#x}", f.to_bits());
}
