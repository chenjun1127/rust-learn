use demo::modules::module;

fn function() {
    println!("called `function()`");
}

fn main() {
    module::function();

    function();

    module::indirect_access();

    module::nested::function();
}
