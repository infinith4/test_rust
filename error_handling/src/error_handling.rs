pub fn run() {
    //let res1 = division_option(5.0, 2.0);
    let res1 = division_option(5.0, 0.0);
    match res1 {
        Some(x) => println!("Result: {:3}", x),
        None => println!("Not allowed!!"),
    }
}

fn division_option(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 {
        None
    } else {
        Some(x / y)
    }
}
