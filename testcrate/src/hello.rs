
fn function() {
    println!("demo `function()`");
}
mod hello {
    pub mod rota {
        pub fn function() {
            println!("demo `hello::rota`");
        }
    }
}
