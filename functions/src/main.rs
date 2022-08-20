fn main() {
    let x = another_function(5);
    println!("return x is: {}", x);

    let condition = true;
    let number = if condition { 5 } else { 6 };

    // numberの値は、{}です
    println!("The value of number is: {}", number);

    //無限ループ
    // loop {
    //     println!("again!");   // また
    // }

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    // 発射！
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        // 値は{}です
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn another_function(x: i32) -> i32 {
    println!("The value of x is: {}", x);   // xの値は{}です
    x  //needless semicolon
}