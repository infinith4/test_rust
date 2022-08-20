fn main() {
    let x = another_function(5);
    println!("return x is: {}", x); 
}

fn another_function(x: i32) -> i32 {
    println!("The value of x is: {}", x);   // xの値は{}です
    x  //needless semicolon
}