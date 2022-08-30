
mod test_mod;
use hello::rota::function as my_function;

fn main() {
    let testStructure = test_mod::Test::TestStructure {
        aaa : "aaaa".to_string(),
        bbb: "bbbb".to_string()
    };

    println!("{:?}", testStructure);
    my_function();

    println!("Coming");
    {
        use crate::hello::rota::function;
        function();
        println!("Returning");
    }
    function();
}