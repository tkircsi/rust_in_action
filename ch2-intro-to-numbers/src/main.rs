fn main() {
    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22i32;

    let addition = twenty + twenty_one + twenty_two;
    println!(
        "{} + {} + {} = {}",
        twenty, twenty_one, twenty_two, addition
    );

    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    let forty_twos = [42.0, 42f32, 42.0_f32];

    // print width 4 ad padding with 0-s on the left side if the digits less than 4
    println!("{:04.2}", forty_twos[0]);

    let x: i32 = 33;
    println!("base 10: {}", x);
    println!("base 2: {:b}", x);
    println!("base 2: {:020b}", x);
    println!("base 8: {:o}", x);
    println!("base 16: {:x}", x);
}
