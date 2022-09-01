struct A(&'static str);

impl Drop for A {
    fn drop(&mut self) {
        println!("dropped: {}", self.0);
    }
}

#[allow(clippy::let_unit_value)]
fn main() {
    let _: () = ();

    let _ = A("x");
    let y = A("y");
    let z = A("z");
}

//https://github.com/rust-lang/rust-clippy/issues/1502
// fn main() {
//     let _: () = f();
// }

// fn f<T: Default>() -> T {
//     T::default()
// }
