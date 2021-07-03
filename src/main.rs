
mod node;
mod domain;

fn main() {
    let mut e = domain::entity::Entity { position: vec![0.0, 1.0, 2.0, 3.0], x_coordinate: 0.0, y_coordinate: 1.0 };
    
    //let mut node = node::Node::new();

    //node.addEntities(vec![e]);
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("Hello World!");
}
