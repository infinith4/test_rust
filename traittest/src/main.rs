struct Sheep {}
struct Cow {}

trait Animal {
    // Instance method signature
    fn noise(&self) -> &'static str;
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// Implement the `Animal` trait for `Cow`.
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

pub trait Geometry {
    fn area(&self) -> f64;
    fn name(&self) -> &str { return "Geometry" }
}

struct Rectangle { width: u32, height: u32 }

impl Geometry for Rectangle {
    fn area(&self) -> f64 {
        self.width as f64 * self.height as f64
    }
    fn name(&self) -> &str { return "Rectangle" }
}

struct Triangle { bottom: u32, height: u32 }

impl Geometry for Triangle {
    fn area(&self) -> f64 {
        self.bottom as f64 * self.height as f64 * 0.5
    }
    fn name(&self) -> &str { return "Triangle" }
}

fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());

    //https://zenn.dev/mebiusbox/books/22d4c1ed9b0003/viewer/497a21
    let a = Rectangle { width: 10, height: 20 };
    let b = Triangle  { bottom: 20, height: 5 };
    println!("{} area={}", a.name(), a.area());
    println!("{} area={}", b.name(), b.area());
}
