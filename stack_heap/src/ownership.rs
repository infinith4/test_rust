pub fn run() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);

    let i1 = 1;
    let i2 = i1;
    println!("{} {}", i1, i2);
    println!("stack address of i1 is: {:p}", &i1);
    println!("stack address of i2 is: {:p}", &i2);

    let sl1 = "literal";
    let sl2 = sl1;

    println!("{} {}", sl1, sl2);

    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("stack address of s3 is: {:p}", &s3);
    println!("stack address of s4 is: {:p}", &s4);
    println!("heap memory of s3 is: {:?}", s3.as_str());
    println!("heap memory of s4 is: {:?}", s4.as_str());
    println!("{} {}", s3, s4);

    let s5 = String::from("hello");
    println!("stack address of s5 is: {:p}", &s5);
    println!("heap memory of s5 is: {:?}", s5.as_str());
    println!("len is: {:?}", s5.len());
    println!("cap is: {:?}", s5.capacity());
    take_onwership(s5);

    let s6 = String::from("hello");
    let s7 = take_giveback_ownership(s6);
    println!("stack address of s7: {:p}", &s7);
    println!("heap memory of s7 is: {:?}", s7.as_str());
    println!("len of s7: {}", s7.len());

    let s8 = String::from("hello");
    let len = calculate_length(&s8);
    println!("The length of '{}' is {}.", s8, len);

    let mut s9 = String::from("hello");
    change(&mut s9);
    println!("{}", s9);

    let s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &s10;
    println!("{} {} {}", s10, r1, r2);
}

fn take_onwership(s: String) {
    println!("stack address of s is: {:p}", &s);
    println!("heap memory of s is: {:?}", s.as_str());
    println!("len is: {:?}", s.len());
    println!("cap is: {:?}", s.capacity());
    println!("{}", s);
}

fn take_giveback_ownership(s: String) -> String {
    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("_world");
}
