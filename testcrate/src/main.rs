use hello::rota::function as my_function;
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

mod test_mod;

fn main() {
    let testStructure : Test::TestStructure= {
        aaa : String.from("aaaa"),
        bbb: String.from("bbbb")
    }
    println!("{:?}",testStructure);
    my_function();

    println!("Coming");
    {
        use crate::hello::rota::function;
        function();
        println!("Returning");
    }
    function();
}