fn add_five(num: u32) -> u32 {
    num + 5
}

fn main() {
    let y: u32 = 50;

    let z: u32 = add_five(y);

    println!("{}", z);
}
