mod entity;
mod node;

fn main() {
    let mut e = entity::Entity { position: vec![0.0, 1.0, 2.0, 3.0]};
    
    let mut node = node::node_impl::new();

    node.addEntities(vec![e]);
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("Hello World!");
}
