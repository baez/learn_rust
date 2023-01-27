fn main() {
    println!("Hello, world from Rust!");
    let mod_result:i32 = 32 % 5;

    println!("32 % 5 is {}", mod_result);

    let result:i32 = i32::pow(8, 3);
    println!("8 to the power of 3 is: {}", result);   

    let result_fp:f32 = f32::powf(1.34, 0.3);
    println!("1.34 to the power of 0.3 is: {}", result_fp);   

    let is_this_real:bool = true;
    println!("is this real? {}", is_this_real);

    let bits_two = 2;
    let bits_four = 4;
    let bits_one = 1;

    let r = bits_four | bits_two | bits_one;
    println!("let and these together? {}", r);
}
