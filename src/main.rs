fn main() {
    println!("Hello, world from Rust!");
    let mod_result:i32 = 32 % 5;

    println!("32 % 5 is {}", mod_result);

    let result:i32 = i32::pow(8, 3);
    println!("8 to the power of 3 is: {}", result);   

    let result_fp:f32 = f32::powf(1.34, 0.3);
    println!("1.34 to the power of 0.3 is: {}", result_fp);   
}
